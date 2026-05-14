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

pub trait DistanceFn {
    fn distance(&self, a: (i16, i16), b: (i16, i16)) -> u16;
}

pub struct DefaultDistanceFn;

impl DistanceFn for DefaultDistanceFn {
    fn distance(&self, a: (i16, i16), b: (i16, i16)) -> u16 {
        ((a.0 - b.0).abs() + (a.1 - b.1).abs()) as u16
    }
}

/// 最適化問題定義。
///
/// すべてのベクタ長は座席数と整合していることを想定します。
pub struct Problem<D: DistanceFn = DefaultDistanceFn> {
    /// 有効な座席一覧（index が SeatId になる）
    pub seats: Vec<Seat>,
    /// 生徒ごとの希望座席のリスト
    /// want_seats[student] -> [(SeatId, weight)]
    pub want_seats: Vec<Vec<WeightedSeatPref>>,
    /// 生徒ごとの (相手, 重み f32) 隣接リスト
    /// pair_edges[student] -> [(other, weight)]
    pub pair_edges: Vec<Vec<WeightedSeatPref>>,
    /// 距離関数
    pub distance_fn: D,
}

impl<D: DistanceFn> Problem<D> {
    /// 距離関数を指定して問題を構築します。
    pub fn with_distance_fn(
        seats: Vec<Seat>,
        want_seats: Vec<Vec<WeightedSeatPref>>,
        pair_edges: Vec<Vec<WeightedSeatPref>>,
        distance_fn: D,
    ) -> Self {
        Self {
            seats,
            want_seats,
            pair_edges,
            distance_fn,
        }
    }
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
            distance_fn: DefaultDistanceFn,
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
    pub(crate) fn distance(&self, a: u16, b: u16) -> u16 {
        let sa = self.seats[a as usize];
        let sb = self.seats[b as usize];
        self.distance_fn.distance((sa.x, sa.y), (sb.x, sb.y))
    }
}
