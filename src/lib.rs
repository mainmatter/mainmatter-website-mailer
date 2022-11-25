use reqwest::Client;
use serde::Deserialize;
use serde_json::json;
use std::future::Future;
use std::result::Result;
use worker::{
    console_log, event, Cors, Date, Env, FormEntry, Method, Request, Response,
    Result as WorkerResult, Router,
};

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
pub async fn main(req: Request, env: Env, _ctx: worker::Context) -> WorkerResult<Response> {
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
            let mut req_clone = req.clone()?;

            if let Ok(payload) = req.json::<Payload>().await {
                send_message(payload, &api_key, &request_sendgrid).await
            } else if let Ok(form_data) = req_clone.form_data().await {
                match (
                    form_data.get("name"),
                    form_data.get("email"),
                    form_data.get("message"),
                ) {
                    (
                        Some(FormEntry::Field(name)),
                        Some(FormEntry::Field(email)),
                        Some(FormEntry::Field(message)),
                    ) => {
                        let payload = Payload {
                            name,
                            email,
                            message,
                        };
                        send_message(payload, &api_key, &request_sendgrid).await
                    }
                    _ => Response::error("Unprocessable Entity", 422),
                }
            } else {
                Response::error("Unprocessable Entity", 422)
            }
        })
        .run(req, env)
        .await?;

    response.with_cors(&cors)
}

pub async fn send_message<'a, Fut>(
    payload: Payload,
    api_key: &'a str,
    sendgrid: impl FnOnce(&'a str, String) -> Fut + 'a,
) -> WorkerResult<Response>
where
    Fut: Future<Output = Result<u16, NetworkError>>,
{
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

    let result = sendgrid(api_key, data.to_string()).await;

    match result {
        Ok(status) => match status {
            202 => Response::ok(""),
            _ => Response::error("Bad Gateway", 502),
        },
        Err(_) => Response::error("Internal Server Error", 500),
    }
}

pub struct NetworkError;

async fn request_sendgrid(api_key: &str, data: String) -> Result<u16, NetworkError> {
    let client = Client::new();
    let result = client
        .post("https://api.sendgrid.com/v3/mail/send")
        .header("Authorization", format!("Bearer {}", api_key))
        .header("Content-Type", "application/json")
        .body(data)
        .send()
        .await;

    match result {
        Ok(response) => Ok(response.status().as_u16()),
        Err(_) => Err(NetworkError),
    }
}
