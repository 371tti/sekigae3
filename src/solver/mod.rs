//! ILSA: Iterated Local Search + Annealing による座席配置最適化エンジン
//! =============================================================
//! このモジュールは教室の座席配置問題を、
//!  * 差分計算付き 2-swap ヒルクライム
//!  * ランダム大ジャンプ
//!  * シミュレーテッドアニーリング確率受容
//! のハイブリッドで高速に近似最適化する。

mod ilsa;
mod individual;
mod problem;
mod rng;

pub use ilsa::ILSA;
pub use individual::Individual;
pub use problem::{Problem, Seat};

#[cfg(test)]
mod tests;
