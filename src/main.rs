use std::sync::Arc;

use dashmap::DashMap;
use kurosabi::{kurosabi::Context, response::Res, Kurosabi};
use rand::{distributions::Alphanumeric, rngs::OsRng, Rng};
use sekigae3::{api::ApiStruct, collector::Sekigae};

struct SekigaeContext {
    pub sekigae_sessions: DashMap<String, Sekigae>,
}

impl SekigaeContext {
    fn new() -> Self {
        SekigaeContext {
            sekigae_sessions: DashMap::new(),
        }
    }

    /// キーの生成
    fn generate_key(&self) -> String {
        let random_id: String = {
            let mut rng = OsRng; // 暗号学的に安全な乱数生成器
            (0..30).map(|_| rng.sample(Alphanumeric) as char).collect()
        };
        random_id
    }

    /// adminユーザーのキーを生成してCookieと席替えにセット
    pub fn key_set(&self, sekigae_id: &str, c: &mut Context<Arc<SekigaeContext>>) {
        let key = self.generate_key();
        c.res.header.set_cookie(sekigae_id, &key);
        self.sekigae_sessions.get_mut(sekigae_id).map(|mut s| {
            s.admin_session = key.clone();
        });
    }

    fn create_sekigae(&self, body: serde_json::Value) -> Result<Sekigae, serde_json::Error> {
        let body_deserialized: ApiStruct = serde_json::from_value(body)?;

        let sekigae = body_deserialized.convert();
        Ok(Sekigae::new(
            sekigae.0,
            &self.generate_key(),
        ))
    }

    pub async fn create(&self, c: &mut Context<Arc<SekigaeContext>>) -> Result<(), String> {
        let body = c.req.body_json().await.map_err(|_| "Failed to parse JSON".to_string())?;

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
        // Clone the Arc context first to avoid borrowing `c` immutably
        let context = Arc::clone(&c.c);
        context.key_set("1w", &mut c);
        // 借用が解除された後に `c` を返す
        c
    });
}
