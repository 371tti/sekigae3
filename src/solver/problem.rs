/// 座席座標
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Seat {
    pub x: i16,
    pub y: i16,
}

/// 問題定義
pub struct Problem {
    /// 有効な座席一覧（index が SeatId になる）
    pub seats: Vec<Seat>,
    /// 生徒ごとの希望座席 ID のリスト
    /// want_seats[student] -> Vec<SeatId>
    pub want_seats: Vec<Vec<u16>>,
    /// 生徒ごとの (相手, 重み f32) 隣接リスト
    /// pair_edges[student] -> [(other, weight)]
    pub pair_edges: Vec<Vec<(u16, f32)>>,
}

impl Problem {
    pub fn new(
        seats: Vec<Seat>,
        want_seats: Vec<Vec<u16>>,
        pair_edges: Vec<Vec<(u16, f32)>>,
    ) -> Self {
        Self {
            seats,
            want_seats,
            pair_edges,
        }
    }

    #[inline]
    pub fn seat_count(&self) -> usize {
        self.seats.len()
    }

    #[inline]
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
