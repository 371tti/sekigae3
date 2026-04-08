//! ILSA: Iterated Local Search + Annealing による座席配置最適化エンジン
//! =============================================================
//! このモジュールは教室の座席配置問題を、
//!  * 差分計算付き 2-swap ヒルクライム
//!  * ランダム大ジャンプ
//!  * シミュレーテッドアニーリング確率受容
//! のハイブリッドで高速に近似最適化する。
//!
//! 通常はクレート直下の再エクスポート経由で
//! `sekigae3::{ILSA, Problem, Seat}` を使うのが簡単です。
//!
//! ## Example
//! ```rust
//! use sekigae3::{ILSA, Problem, Seat};
//!
//! let seats = vec![Seat { x: 0, y: 0 }];
//! let want_seats = vec![vec![(0u16, 1.0f32)]];
//! let pair_edges = vec![Vec::<(u16, f32)>::new()];
//!
//! let problem = Problem::new(seats, want_seats, pair_edges);
//! let mut ilsa = ILSA::new(&problem, 42);
//! let _best = ilsa.solve(10);
//! ```

mod ilsa;
mod individual;
mod problem;
mod rng;

pub use ilsa::ILSA;
pub use individual::Individual;
pub use problem::{Problem, Seat, WeightedSeatPref};

#[cfg(test)]
mod tests;
