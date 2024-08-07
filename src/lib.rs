use reqwest::Client;
use serde::Deserialize;
use serde_json::json;
use std::future::Future;
use std::result::Result;
use worker::{
    console_log, event, Cors, Date, Env, Method, Request, Response, Result as WorkerResult, Router,
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
    pub service: Option<String>,
    pub company: String,
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
            let zapier_email = ctx.secret("ZAPIER_EMAIL")?.to_string();

            match req.json::<Payload>().await {
                Ok(payload) => {
                    send_message(payload, &api_key, &zapier_email, &request_sendgrid).await
                }
                Err(_) => Response::error("Unprocessable Entity", 422),
            }
        })
        .run(req, env)
        .await?;

    response.with_cors(&cors)
}

pub async fn send_message<'a, Fut>(
    payload: Payload,
    api_key: &'a str,
    zapier_email: &'a str,
    sendgrid: impl FnOnce(&'a str, String) -> Fut + 'a,
) -> WorkerResult<Response>
where
    Fut: Future<Output = Result<u16, NetworkError>>,
{
    let message = payload.message.trim();
    let message = if !message.is_empty() { message } else { "–" };
    let service = if let Some(service) = payload.service {
        service
    } else {
        "".to_owned()
    };

    let subject = if !service.is_empty() && service.to_lowercase() != "other" {
        format!("Mainmatter inquiry for {service}")
    } else {
        "Mainmatter inquiry".to_owned()
    };

    let company = payload.company.trim();
    let name = if !company.is_empty() {
        format!("{} ({}) via mainmatter.com", payload.name, company)
    } else {
        format!("{} via mainmatter.com", payload.name)
    };

    let data = json!({
        "personalizations": [{
            "to": [
                { "email": "contact@mainmatter.com", "name": "Mainmatter" }
            ],
            "bcc": [
                { "email": zapier_email }
            ]
        }],
        "from": { "email": "no-reply@mainmatter.com", "name": name },
        "reply_to": { "email": payload.email, "name": payload.name },
        "subject": subject,
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
