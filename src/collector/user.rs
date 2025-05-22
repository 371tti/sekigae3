use std::collections::HashMap;

use serde::{Deserialize, Serialize};

/// ユーザー構造体
/// ユーザーの番号、名前、希望座席を持つ
/// ユーザーは複数の希望座席を持つことができる
/// ここでの希望席はすべての要素において考慮されます。
/// 
/// # example
/// デシリアライズ時の構造例
/// ```json
/// {
///    "number": 1,
///    "name": "John Doe",
///    "want": [ 
///        {
///            "poss": [
///               {
///                "x": 0,
///                "y": 0,
///                "weight": 1.0
///               }
///           ],
///            "with": [
///               {
///                   "number": 2,
///                   "weight": 1.0
///               }
///            ]
///        }
///    ]
/// }
/// ```
#[derive(Clone, Deserialize, Serialize)]
pub struct User {
    pub number: usize,
    pub name: String,
    pub want: Vec<WantSeat>,
}

/// ユーザーの希望座席構造体
/// ユーザーが希望する座席の位置と、他のユーザーとの関係を持つ
/// ここでの希望席は1つが必ず考慮されます。
#[derive(Clone, Deserialize, Serialize)]
pub struct WantSeat {
    pub poss: Vec<SeatPos>,
    pub with: Vec<WithUser>,
}

#[derive(Clone, Deserialize, Serialize)]
pub struct SeatPos {
    pub x: usize,
    pub y: usize,
    pub weight: f32,
}

#[derive(Clone, Deserialize, Serialize)]
pub struct WithUser {
    pub number: usize,
    pub weight: f32,
}

impl User {
    pub fn new(number: usize, name: String) -> Self {
        Self {
            number,
            name,
            want: Vec::new(),
        }
    }

    pub fn cost_calc(&self, pos: &(usize, usize), user_pos: &HashMap<usize, (usize, usize)>) -> f32 {
        let mut cost_vec: Vec<f32> = Vec::new();
        for want in &self.want {
            // 座標での最小値
            let min_cost_pos = want.poss.iter().map(|p| {
                p.weight * ((p.x as f32 - pos.0 as f32).powi(2) + (p.y as f32 - pos.1 as f32).powi(2)).sqrt()
            }).min_by(|a, b| a.partial_cmp(b).unwrap()).unwrap_or(f32::MAX);
            // 他のユーザーとの関係での最小値
            let min_cost_with = want.with.iter().filter_map(|w| {
                user_pos.get(&w.number).map(|pos2| {
                    w.weight * ((pos.0 as f32 - pos2.0 as f32).powi(2) + (pos.1 as f32 - pos2.1 as f32).powi(2)).sqrt()
                })
            }).min_by(|a, b| a.partial_cmp(b).unwrap()).unwrap_or(f32::MAX);
            // 最小値を返す
            cost_vec.push(min_cost_pos.min(min_cost_with));
        }

        if cost_vec.is_empty() {
            0.0
        } else {
            cost_vec.iter().copied().sum::<f32>() / cost_vec.len() as f32
        }
    }
    
    pub fn add_want(&mut self, want: WantSeat) {
        self.want.push(want);
    }
}

impl WantSeat {
    pub fn new() -> Self {
        Self {
            poss: Vec::new(),
            with: Vec::new(),
        }
    }

    pub fn add_pos(&mut self, pos: SeatPos) {
        self.poss.push(pos);
    }

    pub fn add_with(&mut self, with: WithUser) {
        self.with.push(with);
    }
}

impl SeatPos {
    pub fn new(x: usize, y: usize, weight: f32) -> Self {
        Self { x, y, weight }
    }
}

impl WithUser {
    pub fn new(number: usize, weight: f32) -> Self {
        Self { number, weight }
    }
}