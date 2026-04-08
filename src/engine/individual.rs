use super::{problem::Problem, rng::SimpleRng};

/// 個体：座席割当とコスト
#[derive(Clone)]
pub struct Individual {
    /// index = SeatId, value = StudentId
    pub(crate) by_seat: Vec<u16>,
    /// index = StudentId, value = SeatId
    pub(crate) seat_of: Vec<u16>,
    pub(crate) cost: f32,
}

impl Individual {
    pub(crate) fn new_random(problem: &Problem, rng: &mut SimpleRng) -> Self {
        let mut by_seat: Vec<u16> = (0..problem.student_count() as u16).collect();
        rng.shuffle(&mut by_seat);
        let seat_of = Self::inverse(&by_seat);
        let cost = Self::calc_cost(problem, &seat_of);
        Self {
            by_seat,
            seat_of,
            cost,
        }
    }

    /// `by_seat` から `seat_of` を生成
    #[inline]
    fn inverse(by_seat: &[u16]) -> Vec<u16> {
        let mut seat_of = vec![0u16; by_seat.len()];
        for (seat, &student) in by_seat.iter().enumerate() {
            seat_of[student as usize] = seat as u16;
        }
        seat_of
    }

    /// 総コストを計算
    pub(crate) fn calc_cost(problem: &Problem, seat_of: &[u16]) -> f32 {
        let mut cost = 0.0f32;

        // 個人希望
        for (student, wants) in problem.want_seats.iter().enumerate() {
            if wants.is_empty() {
                continue;
            }
            let seat = seat_of[student] as u16;
            let mut best = f32::INFINITY;
            for &(ws, w) in wants {
                let d = problem.manhattan(seat, ws) as f32 * w;
                if d < best {
                    best = d;
                }
            }
            cost += best;
        }

        // ペア距離
        for (a, edges) in problem.pair_edges.iter().enumerate() {
            let seat_a = seat_of[a] as u16;
            for &(b, w) in edges {
                if a as u16 >= b {
                    continue;
                }
                let seat_b = seat_of[b as usize] as u16;
                cost += w * (problem.manhattan(seat_a, seat_b) as f32);
            }
        }
        cost
    }

    /// 2 座席 swap の差分コストを計算
    pub(crate) fn delta_swap_cost(&self, problem: &Problem, i: usize, j: usize) -> f32 {
        if i == j {
            return 0.0;
        }
        let a = self.by_seat[i] as usize;
        let b = self.by_seat[j] as usize;
        let seat_a_old = i as u16;
        let seat_a_new = j as u16;
        let seat_b_old = j as u16;
        let seat_b_new = i as u16;

        let mut delta = 0.0f32;

        // 個人希望 A,B のみ再計算
        for &student in &[a, b] {
            let wants = &problem.want_seats[student];
            if wants.is_empty() {
                continue;
            }
            let old_seat = self.seat_of[student] as u16;
            let new_seat = if student == a { seat_a_new } else { seat_b_new };
            let old_best = wants
                .iter()
                .map(|&(ws, w)| problem.manhattan(old_seat, ws) as f32 * w)
                .fold(f32::INFINITY, f32::min);
            let new_best = wants
                .iter()
                .map(|&(ws, w)| problem.manhattan(new_seat, ws) as f32 * w)
                .fold(f32::INFINITY, f32::min);
            delta += new_best - old_best;
        }

        // ペア距離：A, B 関連のみ
        for &(other, w) in &problem.pair_edges[a] {
            let other_idx = other as usize;
            let seat_other_old = self.seat_of[other_idx] as u16;
            let seat_other_new = if other_idx == b {
                seat_b_new
            } else {
                seat_other_old
            };
            let old = problem.manhattan(seat_a_old, seat_other_old) as f32;
            let new = problem.manhattan(seat_a_new, seat_other_new) as f32;
            delta += w * (new - old);
        }
        for &(other, w) in &problem.pair_edges[b] {
            let other_idx = other as usize;
            let seat_other_old = self.seat_of[other_idx] as u16;
            let seat_other_new = if other_idx == a {
                seat_a_new
            } else {
                seat_other_old
            };
            let old = problem.manhattan(seat_b_old, seat_other_old) as f32;
            let new = problem.manhattan(seat_b_new, seat_other_new) as f32;
            delta += w * (new - old);
        }
        delta
    }

    /// 2 座席 swap を適用し cost を更新（delta を受け取る）
    pub(crate) fn apply_swap(&mut self, i: usize, j: usize, delta: f32) {
        self.by_seat.swap(i, j);
        let a = self.by_seat[i] as usize;
        let b = self.by_seat[j] as usize;
        self.seat_of[a] = i as u16;
        self.seat_of[b] = j as u16;
        self.cost += delta;
    }

    /// 現在の評価コストを返します。
    pub fn cost(&self) -> f32 {
        self.cost
    }

    /// `seat_id -> student_id` の割り当て配列を返します。
    pub fn by_seat(&self) -> &[u16] {
        &self.by_seat
    }

    /// `student_id -> seat_id` の逆引き配列を返します。
    pub fn seat_of(&self) -> &[u16] {
        &self.seat_of
    }
}
