use reqwest;
use serde::Deserialize;
use serde_json::json;
use std::env;
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
struct Payload {
    name: String,
    email: String,
    message: String,
}

#[event(fetch, respond_with_errors)]
pub async fn main(req: Request, env: Env, _ctx: worker::Context) -> Result<Response> {
    log_request(&req);

    // Optionally, get more helpful error messages written to the console in the case of a panic.
    utils::set_panic_hook();

    Router::new()
        .post_async("/send", |mut req, ctx| async move {
            let api_key = ctx.secret("SENDGRID_API_KEY")?.to_string();

            return match req.json::<Payload>().await {
                Ok(payload) => {
                    let data = json!({
                        "personalizations": [{
                            "to": [
                                { "email": "marco.otte-witte@mainmatter.com" }
                            ]}
                        ],
                        "from": { "email": "no-reply@mainmatter.com", "name": "Mainmatter Website" },
                        "reply_to": { "email": payload.email, "name": payload.name },
                        "subject": format!("{} via mainmatter.com", payload.name),
                        "content": [{
                            "type": "text/plain",
                            "value": "and easy to do anywhere, even with cURL"
                        }]
                    });

                    let client = reqwest::Client::new();
                    let result = client.post("https://api.sendgrid.com/v3/mail/send")
                    .header("Authorization", format!("Bearer {}", api_key))
                    .header("Content-Type", "application/json")
                        .body(data.to_string())
                        .send()
                        .await;

                    match result {
                        Ok(response) => {
                            match response.status() {
                                reqwest::StatusCode::ACCEPTED => Response::ok(""),
                                _ => Response::error("Bad Gateway", 502)
                            }
                        },
                        Err(_) => Response::error("Internal Server Error", 500)
                    }

                },
                Err(_) => Response::error("Unprocessable Entity", 422)
            };
        })
        .run(req, env)
        .await
}
