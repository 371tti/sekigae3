use std::sync::Arc;

use dashmap::DashMap;
use kurosabi::{kurosabi::Context, response::Res, Kurosabi};
use rand::{distributions::Alphanumeric, rngs::OsRng, Rng};
use sekigae3::{api::{ApiStruct, IDResult}, collector::Sekigae};

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

    /// Sekigaeセッションを作成する
    /// # Arguments
    /// * `c` - Kurosabiのコンテキスト
    /// # Returns
    /// * Result<String, String> - SekigaeセッションのID / エラーメッセージ
    /// 
    pub async fn create(&self, c: &mut Context<Arc<SekigaeContext>>) -> Result<String/* id */, String> {
        let body = c.req.body_json().await.map_err(|_| "Failed to parse JSON".to_string())?;
        let sekigae = self.create_sekigae(body).map_err(|_| "Failed to create Sekigae".to_string())?;
        if self.sekigae_sessions.contains_key(&sekigae.id) {
            return Err("Sekigae session already exists".to_string());
        }
        let sekigae_id = sekigae.id.clone();
        self.sekigae_sessions.insert(sekigae_id.clone(), sekigae);
        Ok(sekigae_id)
    }
}


fn main() {
    let arc_context = Arc::new(SekigaeContext::new());
    let mut kurosabi = Kurosabi::with_context(arc_context);

    
    kurosabi.get("/", |mut c| async move {
        let html = r#"
        <!DOCTYPE html>
        <html lang="en">
        <head>
            <meta charset="UTF-8">
            <meta name="viewport" content="width=device-width, initial-scale=1.0">
            <title>API Test</title>
        </head>
        <body>
            <h1>Test /api/create</h1>
            <form id="apiForm">
                <textarea id="jsonInput" rows="10" cols="50">
{
    "seat_structure": {
        "rows": 3,
        "columns": 4
    },
    "user_set": [
        [1, "Alice"],
        [2, "Bob"],
        [3, null]
    ]
}
                </textarea>
                <br>
                <button type="button" onclick="sendRequest()">Send</button>
            </form>
            <h2>Response:</h2>
            <pre id="responseOutput"></pre>
            <script>
                async function sendRequest() {
                    const jsonInput = document.getElementById("jsonInput").value;
                    try {
                        const response = await fetch('/api/create', {
                            method: 'POST',
                            headers: {
                                'Content-Type': 'application/json'
                            },
                            body: jsonInput
                        });
                        const responseData = await response.json();
                        document.getElementById("responseOutput").textContent = JSON.stringify(responseData, null, 2);
                    } catch (error) {
                        document.getElementById("responseOutput").textContent = "Error: " + error;
                    }
                }
            </script>
        </body>
        </html>
        "#;
        c.res.html(html);
        c
    });

    kurosabi.post("/api/create", |mut c| async move {
        let context = Arc::clone(&c.c);
        let result = context.create(&mut c).await;
        let res_json = serde_json::to_string(&IDResult::new(result)).unwrap();
        c.res.json(&res_json);
        c
    });

    kurosabi.server()
        .host([127, 0, 0, 1])
        .port(8080)
        .build()
        .run();
}
