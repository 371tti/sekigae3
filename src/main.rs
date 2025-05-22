use std::sync::Arc;

use dashmap::DashMap;
use kurosabi::Kurosabi;
use rand::{distributions::Alphanumeric, Rng};
use rand::rngs::OsRng;
use sekigae3::collector::Sekigae;

struct SekigaeContext {
    sekigae_sessions: DashMap<String, Sekigae>,
}

impl SekigaeContext {
    fn new() -> Self {
        SekigaeContext {
            sekigae_sessions: DashMap::new(),
        }
    }
    
}


fn main() {
    let arc_context = Arc::new(SekigaeContext::new());
    let mut kurosabi = Kurosabi::with_context(arc_context);

    kurosabi.get("/:id", |mut c| async move {
        let id = c.req.path.get_field("id").unwrap_or("".to_string());
        let sekigae = c.c.sekigae_sessions.get_mut(&id);
    })
}
