use serde::Deserialize;
use serde_json::json;
use worker::*;

mod utils;

fn log_request(req: &Request) {
    console_log!(
        "{} - [{}], located at: {:?}, within: {}",
        Date::now().to_string(),
        req.path(),
        req.cf().coordinates().unwrap_or_default(),
        req.cf().region().unwrap_or_else(|| "unknown region".into())
    );
}

#[derive(Deserialize)]
pub struct Payload {
    pub name: String,
    pub email: String,
    pub message: String,
}

#[event(fetch, respond_with_errors)]
pub async fn main(req: Request, env: Env, _ctx: worker::Context) -> Result<Response> {
    log_request(&req);

    // Optionally, get more helpful error messages written to the console in the case of a panic.
    utils::set_panic_hook();

    let cors = Cors::new()
        .with_origins(vec!["*"])
        .with_allowed_headers(vec!["Content-Type", ""])
        .with_methods([Method::Post]);

    let response = Router::new()
        .options_async("/send", |_req, _ctx| async move { Response::ok("") })
        .post_async("/send", |mut req, ctx| async move {
            let api_key = ctx.secret("SENDGRID_API_KEY")?.to_string();

            match req.json::<Payload>().await {
                Ok(payload) => send_message(payload, &api_key).await,
                Err(_) => Response::error("Unprocessable Entity", 422),
            }
        })
        .run(req, env)
        .await?;

    response.with_cors(&cors)
}

pub async fn send_message(payload: Payload, api_key: &str) -> Result<Response> {
    let message = payload.message.trim();
    let message = if !message.is_empty() { message } else { "–" };

    let data = json!({
        "personalizations": [{
            "to": [
                { "email": "contact@mainmatter.com", "name": "Mainmatter" }
            ]}
        ],
        "from": { "email": "no-reply@mainmatter.com", "name": format!("{} via mainmatter.com", payload.name) },
        "reply_to": { "email": payload.email, "name": payload.name },
        "subject": "Mainmatter inquiry",
        "content": [{
            "type": "text/plain",
            "value": message
        }]
    });

    let client = reqwest::Client::new();
    let result = client
        .post("https://api.sendgrid.com/v3/mail/send")
        .header("Authorization", format!("Bearer {}", api_key))
        .header("Content-Type", "application/json")
        .body(data.to_string())
        .send()
        .await;

    match result {
        Ok(response) => match response.status() {
            reqwest::StatusCode::ACCEPTED => Response::ok(""),
            _ => Response::error("Bad Gateway", 502),
        },
        Err(_) => Response::error("Internal Server Error", 500),
    }
}
