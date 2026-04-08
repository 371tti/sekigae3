/// 座席座標。
///
/// `x` が列、`y` が行を表します。
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Seat {
    pub x: i16,
    pub y: i16,
}

/// 希望座席 `(seat_id, weight)`。
pub type WeightedSeatPref = (u16, f32);

/// 最適化問題定義。
///
/// すべてのベクタ長は座席数と整合していることを想定します。
pub struct Problem {
    /// 有効な座席一覧（index が SeatId になる）
    pub seats: Vec<Seat>,
    /// 生徒ごとの希望座席のリスト
    /// want_seats[student] -> [(SeatId, weight)]
    pub want_seats: Vec<Vec<WeightedSeatPref>>,
    /// 生徒ごとの (相手, 重み f32) 隣接リスト
    /// pair_edges[student] -> [(other, weight)]
    pub pair_edges: Vec<Vec<WeightedSeatPref>>,
}

impl Problem {
    /// 問題を構築します。
    ///
    /// - `seats`: 使用可能な座席一覧
    /// - `want_seats[student]`: 学生ごとの希望座席候補 `(seat_id, weight)`
    /// - `pair_edges[student]`: 学生ごとの関係重み (相手ID, 重み)
    pub fn new(
        seats: Vec<Seat>,
        want_seats: Vec<Vec<WeightedSeatPref>>,
        pair_edges: Vec<Vec<WeightedSeatPref>>,
    ) -> Self {
        Self {
            seats,
            want_seats,
            pair_edges,
        }
    }

    #[inline]
    /// 座席数を返します。
    pub fn seat_count(&self) -> usize {
        self.seats.len()
    }

    #[inline]
    /// 学生数を返します。
    ///
    /// 現在は「座席数 = 学生数」を前提にしています。
    pub fn student_count(&self) -> usize {
        self.seats.len()
    }

    #[inline]
    pub(crate) fn manhattan(&self, a: u16, b: u16) -> u16 {
        let sa = self.seats[a as usize];
        let sb = self.seats[b as usize];
        ((sa.x - sb.x).abs() + (sa.y - sb.y).abs()) as u16
    }
}
