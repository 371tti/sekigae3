//! # sekigae3
//! 座席配置最適化のための ILSA (Iterated Local Search + Annealing) ライブラリ。
//! 本アルゴリズムは人とその人の希望席、希望ペアとのマンハッタン距離をコストとする
//! コスト最小化問題を解くものです。
//!
//! クレート直下で主要 API を再エクスポートしているため、
//! `sekigae3::{ILSA, Problem, Seat}` の形で利用できます。
//!
//! ## Example
//! ```rust
//! use sekigae3::{ILSA, Problem, Seat};
//! // 有効な座席座標リスト
//! // index は SeatId として利用される
//! let seats = vec![Seat { x: 0, y: 0 }];
//! // 学生ごとの希望座席リスト (seat_id, weight)
//! // index は StudentId として利用される
//! // weight > 0 では希望席、weight = 0 では無関心、weight < 0 では嫌いな席と定義される
//! let want_seats = vec![vec![(0u16, 1.0f32)]];
//! // 学生ごとのペア関係リスト (other_student_id, weight)
//! // index は StudentId として利用される
//! // weight > 0 では近くに座りたい、weight = 0 では無関心、weight < 0 では離れたいと定義される
//! let pair_edges = vec![Vec::<(u16, f32)>::new()];
//!
//! let problem = Problem::new(seats, want_seats, pair_edges);
//! let mut solver = ILSA::new(&problem, 0 /* 乱数シード 0 でシステムからgetrandom */);
//! let best = solver.solve(1 /* 最大ジャンプ回数 座席数程度で安定 */);
//!
//! assert_eq!(best.by_seat().len(), 1);
//! ```

pub mod engine;

pub use engine::{ILSA, Individual, Problem, Seat, WeightedSeatPref};
