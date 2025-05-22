use std::collections::HashMap;

use seat::{SeatStructure, SekigaeEngine};
use user::User;
use rand::{distributions::Alphanumeric, rngs::OsRng, Rng};

pub mod user;
pub mod seat;

pub struct Sekigae {
    pub id: String,
    pub admin_session: String,
    pub seat_structure: SeatStructure,
    pub users: HashMap<usize, User>,
    pub engine: SekigaeEngine,
}

impl Sekigae {
    pub fn new(structure: SeatStructure, admin_session: &str) -> Self {
        let random_id: String = {
            let mut rng = OsRng; // 暗号学的に安全な乱数生成器
            (0..20).map(|_| rng.sample(Alphanumeric) as char).collect()
        };
        Sekigae {
            id: random_id,
            admin_session: admin_session.to_string(),
            seat_structure: structure,
            users: HashMap::new(),
            engine: SekigaeEngine::new(),
        }
    }

    pub fn id(&self) -> &str {
        &self.id
    }

    pub fn set_user(&mut self, number: usize, user: User) {
        self.users.insert(number, user);
    }

    pub fn get_user(&self, number: usize) -> Option<&User> {
        self.users.get(&number)
    }
}