use std::{collections::HashMap, usize};

use serde::{Deserialize, Serialize};

use super::user::User;

/// 座席構造体
/// 座席構造を指定するための構造体
/// 座席は2次元ベクタで表現され、各要素は座席の有無を示す
#[derive(Deserialize)]
pub struct SeatStructure {
    /// 座席形状を表す2次元ベクタ
    /// trueは座席があることを示し、falseは座席がないことを示す
    /// 例: [[true, false], [false, true]] は、2x2の座席で、左上と右下に座席があることを示す
    pub structure: Vec<Vec<bool>>,
}

/// 座席の状態を表す構造体
/// 席替えの結果を返すための構造体
#[derive(Serialize)]
pub struct ResponseSeat {
    pub result: Vec<Vec<SeatType>>,
}

/// 座席の状態を表す列挙型
/// Empty: 空席
/// User(usize): ユーザーの座席（ユーザー番号を保持）
/// Filled: 足りない分を埋めるための座席（ユーザーがいない状態）
#[derive(Clone, Copy, Debug, Serialize)]
pub enum SeatType {
    Empty,
    User(usize),
    Filled,
}

/// 席替えを行うための構造体
/// 状態保持用
pub struct SekigaeEngine {
    /// 座席状態保持（Enumで表現）
    pub structure: Vec<Vec<SeatType>>,
    /// ユーザーと番号のマップ
    pub user_map: HashMap<usize, User>,
    /// ユーザーの座席位置
    pub user_pos: HashMap<usize, (usize, usize)>,
}

impl SeatStructure {
    /// 新しい座席構造体を作成する
    /// # Arguments
    /// * `structure` - 座席の2次元ベクタ
    /// # Returns
    /// 新しい座席構造体
    pub fn new(structure: Vec<Vec<bool>>) -> Self {
        SeatStructure { structure }
    }

    pub fn count_seat_num(&self) -> usize {
        self.structure.iter().flat_map(|row| row.iter()).filter(|&&seat| seat).count()
    }
}

impl SekigaeEngine {
    pub fn new() -> Self {
        SekigaeEngine {
            structure: Vec::new(),
            user_map: HashMap::new(),
            user_pos: HashMap::new(),
        }
    }

    pub fn return_structure(&self) -> ResponseSeat {
        ResponseSeat {
            result: self.structure.clone(),
        }
    }

    pub fn init(&mut self, structure: SeatStructure, users: Vec<User>) {
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

        // ユーザーIDを個別コストの降順でソート
        let mut user_order: Vec<usize> = self.user_map.keys().cloned().collect();
        user_order.sort_by(|&a, &b| {
            let ca = self.user_map[&a].cost_calc(&self.user_pos[&a], &self.user_pos);
            let cb = self.user_map[&b].cost_calc(&self.user_pos[&b], &self.user_pos);
            cb.partial_cmp(&ca).unwrap_or(std::cmp::Ordering::Equal)
        });

        let mut improved = true;
        while improved {
            improved = false;
            // 毎ループ開始時に最新のコスト順で再ソート
            user_order.sort_by(|&a, &b| {
                let ca = self.user_map[&a].cost_calc(&self.user_pos[&a], &self.user_pos);
                let cb = self.user_map[&b].cost_calc(&self.user_pos[&b], &self.user_pos);
                cb.partial_cmp(&ca).unwrap_or(std::cmp::Ordering::Equal)
            });

            'outer: for &uid1 in &user_order {
                let (i1, j1) = self.user_pos[&uid1];
                // 位置 (i1,j1) が User(uid1) であることは保証済み
                for i2 in 0..h {
                    for j2 in 0..w {
                        if i1 == i2 && j1 == j2 { continue; }
                        match self.structure[i2][j2] {
                            SeatType::User(uid2) => {
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
                            SeatType::Filled => {
                                // Filled とユーザーの入れ替え
                                // swap (ユーザーを Filled 位置へ)
                                self.structure[i1][j1] = SeatType::Filled;
                                self.structure[i2][j2] = SeatType::User(uid1);
                                self.user_pos.insert(uid1, (i2, j2));
                                let cost = self.total_cost();
                                if cost < best {
                                    best = cost;
                                    improved = true;
                                    break 'outer;
                                }
                                // revert
                                self.structure[i1][j1] = SeatType::User(uid1);
                                self.structure[i2][j2] = SeatType::Filled;
                                self.user_pos.insert(uid1, (i1, j1));
                            }
                            _ => {}
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
