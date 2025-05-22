use std::{collections::HashMap, usize};

use serde::Deserialize;

use super::user::User;

#[derive(Deserialize)]
pub struct Seat {
    /// 座席形状を表す2次元ベクタ
    /// trueは座席があることを示し、falseは座席がないことを示す
    /// 例: [[true, false], [false, true]] は、2x2の座席で、左上と右下に座席があることを示す
    pub structure: Vec<Vec<bool>>,
}

pub struct ResponseSeat {
    pub result: Vec<Vec<SeatType>>,
}

#[derive(Clone, Copy, Debug)]
pub enum SeatType {
    Empty,
    User(usize),
    Filled,
}

pub struct ForSortSeat {
    /// 座席状態保持（Enumで表現）
    pub structure: Vec<Vec<SeatType>>,
    /// ユーザーと番号のマップ
    pub user_map: HashMap<usize, User>,
    /// ユーザーの座席位置
    pub user_pos: HashMap<usize, (usize, usize)>,
}


impl ForSortSeat {
    pub fn new() -> Self {
        ForSortSeat {
            structure: Vec::new(),
            user_map: HashMap::new(),
            user_pos: HashMap::new(),
        }
    }

    pub fn init(&mut self, structure: Seat, users: Vec<User>) {
        let h = structure.structure.len();
        let w = if h > 0 { structure.structure[0].len() } else { return; };
        // 初期化: デフォルトは Empty
        self.structure = vec![vec![SeatType::Empty; w]; h];
        self.user_map.clear();
        self.user_pos.clear();
        // ユーザー登録
        for user in &users {
            self.user_map.insert(user.number, user.clone());
        }
        let mut counter = 0;
        for i in 0..h {
            for j in 0..w {
                if structure.structure[i][j] {
                    if let Some(user) = users.get(counter) {
                        let uid = user.number;
                        self.structure[i][j] = SeatType::User(uid);
                        self.user_pos.insert(uid, (i, j));
                    } else {
                        self.structure[i][j] = SeatType::Filled;
                    }
                    counter += 1;
                }
            }
        }
    }

    pub fn optimize(&mut self) {
        // 初期 user_pos が空なら構築
        if self.user_pos.is_empty() {
            for (i, row) in self.structure.iter().enumerate() {
                for (j, seat) in row.iter().enumerate() {
                    if let SeatType::User(uid) = seat {
                        self.user_pos.insert(*uid, (i, j));
                    }
                }
            }
        }
        // 現在の合計コスト
        let mut best = self.total_cost();
        let h = self.structure.len();
        let w = if h > 0 { self.structure[0].len() } else { return; };
        let mut improved = true;
        while improved {
            improved = false;
            'outer: for i1 in 0..h {
                for j1 in 0..w {
                    if let SeatType::User(uid1) = self.structure[i1][j1] {
                        for i2 in 0..h {
                            for j2 in 0..w {
                                if i1 == i2 && j1 == j2 { continue; }
                                if let SeatType::User(uid2) = self.structure[i2][j2] {
                                    // swap
                                    self.structure[i1][j1] = SeatType::User(uid2);
                                    self.structure[i2][j2] = SeatType::User(uid1);
                                    self.user_pos.insert(uid1, (i2, j2));
                                    self.user_pos.insert(uid2, (i1, j1));
                                    let cost = self.total_cost();
                                    if cost < best {
                                        best = cost;
                                        improved = true;
                                        break 'outer;
                                    }
                                    // revert
                                    self.structure[i1][j1] = SeatType::User(uid1);
                                    self.structure[i2][j2] = SeatType::User(uid2);
                                    self.user_pos.insert(uid1, (i1, j1));
                                    self.user_pos.insert(uid2, (i2, j2));
                                }
                            }
                        }
                    }
                }
            }
        }
    }

    // 全ユーザーの合計コストを計算
    pub fn total_cost(&self) -> f32 {
        self.user_map.iter().map(|(&uid, user)| {
            let pos = &self.user_pos[&uid];
            user.cost_calc(pos, &self.user_pos)
        }).sum()
    }
}
