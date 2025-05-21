use std::collections::HashMap;

use super::user::User;

pub struct Seat {
    /// 座席形状を表す2次元ベクタ
    /// trueは座席があることを示し、falseは座席がないことを示す
    /// 例: [[true, false], [false, true]] は、2x2の座席で、左上と右下に座席があることを示す
    pub structure: Vec<Vec<bool>>,
}

pub struct ForSortSeat {
    /// 座席状態保持
    /// maxは座席がないことを示す
    /// usizeにはUserのnumberが入る
    /// 例: [[0, 1], [max, 2]] は、左上にUser 0、右上にUser 1、右下にUser 2がいることを示す
    pub structure: Vec<Vec<usize>>,
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
        // 初期化
        self.structure = vec![vec![usize::MAX; w]; h];
        self.user_map.clear();
        self.user_pos.clear();
        // ユーザー登録
        for user in &users {
            self.user_map.insert(user.number, user.clone());
        }
        // ユーザーを空席に順次配置
        let mut counter = 0;
        for i in 0..h {
            for j in 0..w {
                if structure.structure[i][j] {
                    let uid = users.get(counter)
                        .unwrap_or(&User::new(counter, "N/A".to_string()))
                        .number;
                    self.structure[i][j] = uid;
                    self.user_pos.insert(uid, (i, j));
                    counter += 1;
                }
            }
        }
    }

    pub fn optimization(&mut self) {
    pub fn optimization(&mut self) {
        // 初期 user_pos が空なら構築
        if self.user_pos.is_empty() {
            let no_seat = usize::MAX;
            for (i, row) in self.structure.iter().enumerate() {
                for (j, &uid) in row.iter().enumerate() {
                    if uid != no_seat {
                        self.user_pos.insert(uid, (i, j));
                    }
                }
            }
        }
        // 現在の合計コスト
        let mut best = self.total_cost();
        let no_seat = usize::MAX;
        let h = self.structure.len();
        let w = if h > 0 { self.structure[0].len() } else { return; };
        let dirs = [(1,0),(-1,0),(0,1),(0,-1)];
        let mut improved = true;
        while improved {
            improved = false;
            for i in 0..h {
                for j in 0..w {
                    let uid = self.structure[i][j];
                    if uid == no_seat { continue; }
                    for &(di,dj) in &dirs {
                        let ni = i as isize + di;
                        let nj = j as isize + dj;
                        if ni<0||nj<0 { continue; }
                        let (ni, nj) = (ni as usize, nj as usize);
                        if ni>=h||nj>=w { continue; }
                        let other = self.structure[ni][nj];
                        // スワップ
                        self.structure[i][j] = other;
                        self.structure[ni][nj] = uid;
                        self.user_pos.insert(uid, (ni,nj));
                        if other != no_seat {
                            self.user_pos.insert(other, (i,j));
                        }
                        let cost = self.total_cost();
                        if cost < best {
                            best = cost;
                            improved = true;
                            break;
                        }
                        // リバート
                        self.structure[i][j] = uid;
                        self.structure[ni][nj] = other;
                        self.user_pos.insert(uid, (i,j));
                        if other != no_seat {
                            self.user_pos.insert(other, (ni,nj));
                        }
                    }
                    if improved { break; }
                }
                if improved { break; }
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
