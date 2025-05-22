use serde::Deserialize;

use crate::collector::{seat::SeatStructure, user::User};

#[derive(Deserialize)]
pub struct ApiStruct {
    pub seat_structure: SeatStructure,
    pub user_set: Vec<(usize, Option<String>)>,
}

impl ApiStruct {
    pub fn convert(self) -> (SeatStructure, Vec<User>) {
        let user_set = self.user_set.iter().map(|(number, name)| {
            User::new(*number, name.clone().unwrap_or_else(|| "".to_string()))
        }).collect::<Vec<User>>();

        (
            self.seat_structure,
            user_set,
        )
    }
}