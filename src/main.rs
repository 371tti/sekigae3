use std::sync::Arc;

use dashmap::DashMap;
use kurosabi::Kurosabi;
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

    
    kurosabi.get("/",  |mut c| async move {
        c.res.text("Hello, World!");
        let key = "session_id";
        let value = "123456";
        c.res.header.set_cookie(key, value);
        c.res.header.set("X-Custom-Header", "MyValue");
        c
    });

    kurosabi.get("/api/:id/info", |mut c| async move {
        let id = c.req.path.get_field("id").unwrap_or("".to_string());
        {
            // `c.c.sekigae_sessions` を借用するスコープを限定
            let sekigae = c.c.sekigae_sessions.get_mut(&id);
            // 必要な処理をここで行う
        }
        // 借用が解除された後に `c` を返す
        c
    });
}
