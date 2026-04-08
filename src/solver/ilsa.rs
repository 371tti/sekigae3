use std::{cmp::Ordering, collections::HashSet};

use log::{debug, info};

use super::{individual::Individual, problem::Problem, rng::SimpleRng};

const IMPROVEMENT_EPSILON: f32 = 1e-6;
const CONVERGENCE_MIN_STALL: usize = 12;
const CONVERGENCE_MAX_STALL: usize = 80;
const MIN_OUTER_PASSES: usize = 20;
const CANDIDATE_POOL_MULTIPLIER: usize = 8;

/// ILSA 探索エンジン
pub struct ILSA<'p> {
    problem: &'p Problem,
    rng: SimpleRng,
}

impl<'p> ILSA<'p> {
    /// 新規インスタンス
    pub fn new(problem: &'p Problem, seed: u64) -> Self {
        let rng = SimpleRng::new(seed);
        Self { problem, rng }
    }

    /// メインソルバー
    /// - `budget` – 大ジャンプ回数（例: 300）
    pub fn solve(&mut self, budget: usize) -> Individual {
        self.solve_candidates(budget, 1)
            .into_iter()
            .next()
            .expect("solve_candidates always returns at least one candidate")
    }

    /// 十分収束した段階で複数候補を返すソルバー
    /// - `budget` – 最大ジャンプ回数
    /// - `max_candidates` – 返す候補数（必ずこの件数を返す）
    pub fn solve_candidates(&mut self, budget: usize, max_candidates: usize) -> Vec<Individual> {
        let candidate_limit = max_candidates.max(1);
        let history_limit = candidate_limit
            .saturating_mul(CANDIDATE_POOL_MULTIPLIER)
            .max(candidate_limit);
        let stall_threshold = Self::stall_threshold(budget);
        let min_outer_passes = budget.min(MIN_OUTER_PASSES);

        info!(
            "ILSA start: students={}, budget={}, candidate_limit={}, stall_threshold={}, min_outer_passes={}",
            self.problem.student_count(),
            budget,
            candidate_limit,
            stall_threshold,
            min_outer_passes
        );

        let mut current = Individual::new_random(self.problem, &mut self.rng);
        Self::hill_climb(&mut current, self.problem);
        debug!("initial hill-climb complete: cost={:.3}", current.cost());

        let mut best = current.clone();
        let mut history = Vec::new();
        let mut seen = HashSet::<Vec<u16>>::new();
        Self::push_history_candidate(&mut history, &mut seen, &best, history_limit);

        let mut temp = 100.0f32;
        let alpha = 0.95f32;
        let mut stall_iters = 0usize;

        for iter in 0..budget {
            let mut trial = current.clone();
            Self::random_k_swaps(&mut trial, 12, self.problem, &mut self.rng);
            Self::hill_climb(&mut trial, self.problem);

            let delta = trial.cost() - current.cost();
            let accepted = delta < 0.0 || self.rng.next_f32() < (-delta / temp).exp();
            if accepted {
                current = trial.clone();
            }

            let improved_best = trial.cost() + IMPROVEMENT_EPSILON < best.cost();
            if improved_best {
                best = trial.clone();
                stall_iters = 0;
                debug!(
                    "new best found: iter={}, best_cost={:.3}",
                    iter + 1,
                    best.cost()
                );
            } else {
                stall_iters += 1;
            }

            let candidate_band = Self::candidate_band(best.cost());
            if improved_best || trial.cost() <= best.cost() + candidate_band {
                Self::push_history_candidate(&mut history, &mut seen, &trial, history_limit);
            }
            if accepted && current.cost() <= best.cost() + candidate_band {
                Self::push_history_candidate(&mut history, &mut seen, &current, history_limit);
            }

            if iter == 0 || (iter + 1) % 25 == 0 || iter + 1 == budget {
                debug!(
                    "iter={} temp={:.3} current={:.3} best={:.3} delta={:.3} accepted={} stall={} pool={}",
                    iter + 1,
                    temp,
                    current.cost(),
                    best.cost(),
                    delta,
                    accepted,
                    stall_iters,
                    history.len()
                );
            }

            if iter + 1 >= min_outer_passes
                && stall_iters >= stall_threshold
                && history.len() >= candidate_limit
            {
                info!(
                    "ILSA converged: iter={}, stall={}, collected={}",
                    iter + 1,
                    stall_iters,
                    history.len()
                );
                break;
            }

            temp *= alpha;
        }

        Self::push_history_candidate(&mut history, &mut seen, &best, history_limit);
        self.collect_more_candidates(
            &mut history,
            &mut seen,
            &best,
            candidate_limit,
            history_limit,
        );

        let candidates = Self::select_top_n_history_ordered(&history, candidate_limit);

        info!(
            "ILSA done: best_cost={:.3}, returned_candidates={}",
            best.cost(),
            candidates.len()
        );
        candidates
    }

    #[inline]
    fn compare_cost(a: &Individual, b: &Individual) -> Ordering {
        a.cost().partial_cmp(&b.cost()).unwrap_or(Ordering::Equal)
    }

    fn push_history_candidate(
        history: &mut Vec<Individual>,
        seen: &mut HashSet<Vec<u16>>,
        candidate: &Individual,
        history_limit: usize,
    ) {
        let key = candidate.by_seat().to_vec();
        if !seen.insert(key) {
            return;
        }

        history.push(candidate.clone());

        if history.len() > history_limit {
            if let Some((worst_idx, worst)) = history
                .iter()
                .enumerate()
                .max_by(|(_, a), (_, b)| Self::compare_cost(a, b))
                .map(|(idx, item)| (idx, item.clone()))
            {
                history.swap_remove(worst_idx);
                seen.remove(&worst.by_seat().to_vec());
            }
        }
    }

    fn select_top_n_history_ordered(history: &[Individual], n: usize) -> Vec<Individual> {
        if history.is_empty() {
            return Vec::new();
        }

        let mut ranked: Vec<usize> = (0..history.len()).collect();
        ranked.sort_by(|&ia, &ib| {
            let a = &history[ia];
            let b = &history[ib];
            Self::compare_cost(a, b).then_with(|| ia.cmp(&ib))
        });

        let base_count = n.min(history.len());
        let mut selected_idx = ranked.into_iter().take(base_count).collect::<Vec<_>>();
        selected_idx.sort_unstable();

        let mut out = selected_idx
            .iter()
            .map(|&idx| history[idx].clone())
            .collect::<Vec<_>>();

        // unique 候補が不足するケースでは履歴を循環し、重複を最小化しつつ N 件を埋める。
        let unique_len = out.len().max(1);
        while out.len() < n {
            let src = (out.len() - base_count) % unique_len;
            out.push(out[src].clone());
        }
        out
    }

    fn collect_more_candidates(
        &mut self,
        history: &mut Vec<Individual>,
        seen: &mut HashSet<Vec<u16>>,
        best: &Individual,
        candidate_limit: usize,
        history_limit: usize,
    ) {
        if history.len() >= candidate_limit {
            return;
        }

        let seat_count = self.problem.student_count().max(1);
        let extra_attempts = candidate_limit.saturating_mul(32).max(32);

        for attempt in 0..extra_attempts {
            if history.len() >= candidate_limit {
                break;
            }

            let mut trial = best.clone();
            let jump_k = ((attempt % seat_count) + 2).min(seat_count.max(2));
            Self::random_k_swaps(&mut trial, jump_k, self.problem, &mut self.rng);
            Self::hill_climb(&mut trial, self.problem);

            let candidate_band = Self::candidate_band(best.cost());
            if trial.cost() <= best.cost() + candidate_band {
                Self::push_history_candidate(history, seen, &trial, history_limit);
            }
        }

        if history.len() < candidate_limit {
            info!(
                "candidate diversity was insufficient: unique_collected={}, requested={}",
                history.len(),
                candidate_limit
            );
        }
    }

    #[inline]
    fn stall_threshold(budget: usize) -> usize {
        (budget / 4)
            .max(CONVERGENCE_MIN_STALL)
            .min(CONVERGENCE_MAX_STALL)
    }

    #[inline]
    fn candidate_band(best_cost: f32) -> f32 {
        (best_cost.abs() * 0.03).max(0.5)
    }

    /// 2-swap ヒルクライム（最良改善を即時採用）
    fn hill_climb(ind: &mut Individual, prob: &Problem) {
        let n = ind.by_seat.len();
        if n < 2 {
            ind.cost = Individual::calc_cost(prob, &ind.seat_of);
            return;
        }

        let mut accepted_moves = 0usize;

        loop {
            let mut best_move: Option<(usize, usize, f32)> = None;

            for i in 0..n {
                for j in (i + 1)..n {
                    let delta = ind.delta_swap_cost(prob, i, j);
                    if delta < -IMPROVEMENT_EPSILON {
                        match best_move {
                            Some((_, _, best_delta)) if delta >= best_delta => {}
                            _ => best_move = Some((i, j, delta)),
                        }
                    }
                }
            }

            let Some((i, j, delta)) = best_move else {
                break;
            };

            let old_cost = ind.cost();
            ind.apply_swap(i, j, delta);
            let exact_cost = Individual::calc_cost(prob, &ind.seat_of);

            if exact_cost + IMPROVEMENT_EPSILON < old_cost {
                ind.cost = exact_cost;
                accepted_moves += 1;
            } else {
                ind.apply_swap(i, j, -delta);
                ind.cost = old_cost;
                break;
            }
        }

        // 差分更新での誤差を吸収するため、終了時に厳密コストへ同期する
        ind.cost = Individual::calc_cost(prob, &ind.seat_of);
        debug!(
            "hill_climb done: accepted_moves={} synced_cost={:.3}",
            accepted_moves,
            ind.cost()
        );
    }

    /// 座席を k 回ランダム swap して大ジャンプを作る
    fn random_k_swaps(ind: &mut Individual, k: usize, prob: &Problem, rng: &mut SimpleRng) {
        let n = ind.by_seat.len();
        if n < 2 {
            ind.cost = Individual::calc_cost(prob, &ind.seat_of);
            debug!("random_k_swaps skipped: seat_count < 2");
            return;
        }
        for _ in 0..k {
            let i = rng.gen_range(0..n);
            let j = rng.gen_range(0..n);
            if i == j {
                continue;
            }
            ind.by_seat.swap(i, j);
            let a = ind.by_seat[i] as usize;
            let b = ind.by_seat[j] as usize;
            ind.seat_of[a] = i as u16;
            ind.seat_of[b] = j as u16;
        }
        // ジャンプ後にコスト再評価（安全策）
        ind.cost = Individual::calc_cost(prob, &ind.seat_of);
    }
}
