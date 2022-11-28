use mainmatter_website_mailer::{send_message, NetworkError, Payload};
use serde_json::json;
use wasm_bindgen_test::*;
use worker::{Response, Url};

wasm_bindgen_test::wasm_bindgen_test_configure!(run_in_browser);

#[wasm_bindgen_test]
async fn it_works_for_the_happy_path() {
    async fn request_sendgrid(_api_key: &str, _data: String) -> Result<u16, NetworkError> {
        Ok(202)
    }

    let payload = Payload {
        name: String::from("name"),
        email: String::from("email@domain.tld"),
        message: String::from("Hi!"),
    };
    let result = send_message(payload, "api_key", &request_sendgrid, Response::ok("")).await;

    assert_eq!(result.unwrap().status_code(), 200);
}

#[wasm_bindgen_test]
async fn it_sends_the_right_payload_to_sendgrid() {
    async fn request_sendgrid(_api_key: &str, data: String) -> Result<u16, NetworkError> {
        let expected = json!({
            "personalizations": [{
                "to": [
                    { "email": "contact@mainmatter.com", "name": "Mainmatter" }
                ]}
            ],
            "from": { "email": "no-reply@mainmatter.com", "name": "name via mainmatter.com" },
            "reply_to": { "email": "email@domain.tld", "name": "name" },
            "subject": "Mainmatter inquiry",
            "content": [{
                "type": "text/plain",
                "value": "Hi!"
            }]
        });

        assert_eq!(data, expected.to_string());

        Ok(202)
    }

    let payload = Payload {
        name: String::from("name"),
        email: String::from("email@domain.tld"),
        message: String::from("Hi!"),
    };
    let _result = send_message(payload, "api_key", &request_sendgrid, Response::ok("")).await;
}

#[wasm_bindgen_test]
async fn it_sends_an_empty_message_if_none_is_provided() {
    async fn request_sendgrid(_api_key: &str, data: String) -> Result<u16, NetworkError> {
        let expected = json!({
            "personalizations": [{
                "to": [
                    { "email": "contact@mainmatter.com", "name": "Mainmatter" }
                ]}
            ],
            "from": { "email": "no-reply@mainmatter.com", "name": "name via mainmatter.com" },
            "reply_to": { "email": "email@domain.tld", "name": "name" },
            "subject": "Mainmatter inquiry",
            "content": [{
                "type": "text/plain",
                "value": "–"
            }]
        });

        assert_eq!(data, expected.to_string());

        Ok(202)
    }

    let payload = Payload {
        name: String::from("name"),
        email: String::from("email@domain.tld"),
        message: String::from(""),
    };
    let _result = send_message(payload, "api_key", &request_sendgrid, Response::ok("")).await;
}

#[wasm_bindgen_test]
async fn it_responds_with_502_if_sendgrid_errors() {
    async fn request_sendgrid(_api_key: &str, _data: String) -> Result<u16, NetworkError> {
        Ok(500)
    }

    let payload = Payload {
        name: String::from("name"),
        email: String::from("email@domain.tld"),
        message: String::from("Hi!"),
    };
    let result = send_message(payload, "api_key", &request_sendgrid, Response::ok("")).await;

    assert_eq!(result.unwrap().status_code(), 502);
}

#[wasm_bindgen_test]
async fn it_responds_with_500_if_calling_sendgrid_errors() {
    async fn request_sendgrid(_api_key: &str, _data: String) -> Result<u16, NetworkError> {
        Err(NetworkError)
    }

    let payload = Payload {
        name: String::from("name"),
        email: String::from("email@domain.tld"),
        message: String::from("Hi!"),
    };
    let result = send_message(payload, "api_key", &request_sendgrid, Response::ok("")).await;

    assert_eq!(result.unwrap().status_code(), 500);
}

#[wasm_bindgen_test]
async fn it_responds_with_the_passed_in_response() {
    async fn request_sendgrid(_api_key: &str, _data: String) -> Result<u16, NetworkError> {
        Ok(202)
    }

    let payload = Payload {
        name: String::from("name"),
        email: String::from("email@domain.tld"),
        message: String::from("Hi!"),
    };
    let result = send_message(
        payload,
        "api_key",
        &request_sendgrid,
        Response::redirect(Url::parse("https://domain.tld/path").unwrap()),
    )
    .await;

    if let Ok(result) = result {
        assert_eq!(result.status_code(), 302);
        assert_eq!(
            result.headers().get("location").unwrap(),
            Some(String::from("https://domain.tld/path"))
        );
    }
}
