use std::collections::HashMap;

/// ユーザー構造体
/// ユーザーの番号、名前、希望座席を持つ
/// ユーザーは複数の希望座席を持つことができる
/// ここでの希望席はすべての要素において考慮されます。
#[derive(Clone)]
pub struct User {
    pub number: usize,
    pub name: String,
    pub want: Vec<WantSeat>,
}

/// ユーザーの希望座席構造体
/// ユーザーが希望する座席の位置と、他のユーザーとの関係を持つ
/// ここでの希望席は1つが必ず考慮されます。
#[derive(Clone)]
pub struct WantSeat {
    pub poss: Vec<SeatPos>,
    pub with: Vec<WithUser>,
}

#[derive(Clone)]
pub struct SeatPos {
    pub x: usize,
    pub y: usize,
    pub weight: f32,
}

#[derive(Clone)]
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
            }).min_by(|a, b| a.partial_cmp(b).unwrap()).unwrap_or(0.0);
            // 他のユーザーとの関係での最小値
            let min_cost_with = want.with.iter().map(|w| {
                let pos2 = user_pos.get(&w.number).unwrap();
                w.weight * ((pos.0 as f32 - pos2.0 as f32).powi(2) + (pos.1 as f32 - pos2.1 as f32).powi(2)).sqrt()
            }).min_by(|a, b| a.partial_cmp(b).unwrap()).unwrap_or(0.0);
            // 最小値を返す
            cost_vec.push(min_cost_pos.min(min_cost_with));
        }

        cost_vec.iter().copied().sum::<f32>() / cost_vec.len() as f32
    }
    
}