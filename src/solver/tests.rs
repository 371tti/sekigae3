use std::collections::HashSet;

use super::{ILSA, Problem, Seat};

fn sample_problem() -> Problem {
    let seats = vec![
        Seat { x: 0, y: 0 },
        Seat { x: 0, y: 1 },
        Seat { x: 1, y: 0 },
        Seat { x: 1, y: 1 },
    ];

    let want_seats = vec![vec![0], vec![1], vec![2], vec![3]];

    let pair_edges = vec![
        vec![(1, 1.0), (2, 0.5)],
        vec![(0, 1.0), (3, 0.5)],
        vec![(0, 0.5), (3, 1.0)],
        vec![(1, 0.5), (2, 1.0)],
    ];

    Problem::new(seats, want_seats, pair_edges)
}

#[test]
fn solve_returns_valid_permutation() {
    let problem = sample_problem();
    let mut ilsa = ILSA::new(&problem, 42);
    let best = ilsa.solve(50);

    assert_eq!(best.by_seat().len(), problem.seat_count());
    assert_eq!(best.seat_of().len(), problem.student_count());
    assert!(best.cost().is_finite());

    let uniq: HashSet<u16> = best.by_seat().iter().copied().collect();
    assert_eq!(uniq.len(), problem.student_count());
}

#[test]
fn non_zero_seed_is_deterministic() {
    let problem = sample_problem();

    let mut ilsa_a = ILSA::new(&problem, 12345);
    let best_a = ilsa_a.solve(40);

    let mut ilsa_b = ILSA::new(&problem, 12345);
    let best_b = ilsa_b.solve(40);

    assert_eq!(best_a.by_seat(), best_b.by_seat());
    assert_eq!(best_a.seat_of(), best_b.seat_of());
    assert!((best_a.cost() - best_b.cost()).abs() < 1e-6);
}

#[test]
fn zero_budget_still_returns_solution() {
    let problem = sample_problem();
    let mut ilsa = ILSA::new(&problem, 99);
    let best = ilsa.solve(0);

    assert_eq!(best.by_seat().len(), problem.student_count());
    assert!(best.cost().is_finite());
}

fn symmetric_problem() -> Problem {
    let seats = vec![
        Seat { x: 0, y: 0 },
        Seat { x: 0, y: 1 },
        Seat { x: 1, y: 0 },
        Seat { x: 1, y: 1 },
    ];

    let want_seats = vec![vec![], vec![], vec![], vec![]];
    let pair_edges = vec![vec![], vec![], vec![], vec![]];

    Problem::new(seats, want_seats, pair_edges)
}

#[test]
fn solve_candidates_prefers_unique_candidates() {
    let problem = symmetric_problem();
    let mut ilsa = ILSA::new(&problem, 7);
    let candidates = ilsa.solve_candidates(40, 3);

    assert_eq!(candidates.len(), 3);

    let uniq: HashSet<Vec<u16>> = candidates.iter().map(|c| c.by_seat().to_vec()).collect();
    assert_eq!(uniq.len(), candidates.len());
}

#[test]
fn solve_candidates_always_returns_requested_count() {
    let problem = Problem::new(vec![Seat { x: 0, y: 0 }], vec![vec![0]], vec![vec![]]);

    let mut ilsa = ILSA::new(&problem, 1);
    let candidates = ilsa.solve_candidates(10, 5);

    assert_eq!(candidates.len(), 5);
    assert!(candidates.iter().all(|c| c.by_seat() == [0]));
    assert!(
        candidates
            .windows(2)
            .all(|w| w[0].cost() <= w[1].cost() + 1e-6)
    );
}
