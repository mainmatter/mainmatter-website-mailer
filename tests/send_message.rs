use mainmatter_website_mailer::{send_message, NetworkError, Payload};
use serde_json::json;
use wasm_bindgen_test::*;

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
        service: String::from("Digital Products & Design"),
    };
    let result = send_message(payload, "api_key", &request_sendgrid).await;

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
                "value": "Service: Digital Products & Design\n\nHi!"
            }]
        });

        assert_eq!(data, expected.to_string());

        Ok(200)
    }

    let payload = Payload {
        name: String::from("name"),
        email: String::from("email@domain.tld"),
        message: String::from("Hi!"),
        service: String::from("Digital Products & Design"),
    };
    let _result = send_message(payload, "api_key", &request_sendgrid).await;
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
                "value": "Service: Digital Products & Design\n\n–"
            }]
        });

        assert_eq!(data, expected.to_string());

        Ok(200)
    }

    let payload = Payload {
        name: String::from("name"),
        email: String::from("email@domain.tld"),
        message: String::from(""),
        service: String::from("Digital Products & Design"),
    };
    let _result = send_message(payload, "api_key", &request_sendgrid).await;
}

#[wasm_bindgen_test]
async fn it_sends_an_empty_service_if_none_is_provided() {
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
                "value": "Service: –\n\nHi!"
            }]
        });

        assert_eq!(data, expected.to_string());

        Ok(200)
    }

    let payload = Payload {
        name: String::from("name"),
        email: String::from("email@domain.tld"),
        message: String::from("Hi!"),
        service: String::from(""),
    };
    let _result = send_message(payload, "api_key", &request_sendgrid).await;
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
        service: String::from(""),
    };
    let result = send_message(payload, "api_key", &request_sendgrid).await;

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
        service: String::from(""),
    };
    let result = send_message(payload, "api_key", &request_sendgrid).await;

    assert_eq!(result.unwrap().status_code(), 500);
}
