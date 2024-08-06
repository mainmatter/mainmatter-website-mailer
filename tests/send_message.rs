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
        service: Some(String::from("Digital Products & Design")),
        company: None,
    };
    let result = send_message(payload, "api_key", "trigger@zapier.com", &request_sendgrid).await;

    assert_eq!(result.unwrap().status_code(), 200);
}

#[wasm_bindgen_test]
async fn it_sends_the_right_payload_to_sendgrid() {
    async fn request_sendgrid(_api_key: &str, data: String) -> Result<u16, NetworkError> {
        let expected = json!({
            "personalizations": [{
                "to": [
                    { "email": "contact@mainmatter.com", "name": "Mainmatter" }
                ],
                "bcc": [
                    { "email": "trigger@zapier.com" }
                ]
            }],
            "from": { "email": "no-reply@mainmatter.com", "name": "name via mainmatter.com" },
            "reply_to": { "email": "email@domain.tld", "name": "name" },
            "subject": "Mainmatter inquiry for Digital Products & Design",
            "content": [{
                "type": "text/plain",
                "value": "Hi!"
            }]
        });

        assert_eq!(data, expected.to_string());

        Ok(200)
    }

    let payload = Payload {
        name: String::from("name"),
        email: String::from("email@domain.tld"),
        message: String::from("Hi!"),
        service: Some(String::from("Digital Products & Design")),
        company: None,
    };
    let _result = send_message(payload, "api_key", "trigger@zapier.com", &request_sendgrid).await;
}

#[wasm_bindgen_test]
async fn it_sends_an_empty_message_if_none_is_provided() {
    async fn request_sendgrid(_api_key: &str, data: String) -> Result<u16, NetworkError> {
        let expected = json!({
            "personalizations": [{
                "to": [
                    { "email": "contact@mainmatter.com", "name": "Mainmatter" }
                ],
                "bcc": [
                    { "email": "trigger@zapier.com" }
                ]
            }],
            "from": { "email": "no-reply@mainmatter.com", "name": "name via mainmatter.com" },
            "reply_to": { "email": "email@domain.tld", "name": "name" },
            "subject": "Mainmatter inquiry for Digital Products & Design",
            "content": [{
                "type": "text/plain",
                "value": "–"
            }]
        });

        assert_eq!(data, expected.to_string());

        Ok(200)
    }

    let payload = Payload {
        name: String::from("name"),
        email: String::from("email@domain.tld"),
        message: String::from(""),
        service: Some(String::from("Digital Products & Design")),
        company: None,
    };
    let _result = send_message(payload, "api_key", "trigger@zapier.com", &request_sendgrid).await;
}

#[wasm_bindgen_test]
async fn it_leaves_out_the_service_if_an_empty_one_is_provided() {
    async fn request_sendgrid(_api_key: &str, data: String) -> Result<u16, NetworkError> {
        let expected = json!({
            "personalizations": [{
                "to": [
                    { "email": "contact@mainmatter.com", "name": "Mainmatter" }
                ],
                "bcc": [
                    { "email": "trigger@zapier.com" }
                ]
            }],
            "from": { "email": "no-reply@mainmatter.com", "name": "name via mainmatter.com" },
            "reply_to": { "email": "email@domain.tld", "name": "name" },
            "subject": "Mainmatter inquiry",
            "content": [{
                "type": "text/plain",
                "value": "Hi!"
            }]
        });

        assert_eq!(data, expected.to_string());

        Ok(200)
    }

    let payload = Payload {
        name: String::from("name"),
        email: String::from("email@domain.tld"),
        message: String::from("Hi!"),
        service: Some(String::from("")),
        company: None,
    };
    let _result = send_message(payload, "api_key", "trigger@zapier.com", &request_sendgrid).await;
}

#[wasm_bindgen_test]
async fn it_leaves_out_the_service_if_none_is_provided() {
    async fn request_sendgrid(_api_key: &str, data: String) -> Result<u16, NetworkError> {
        let expected = json!({
            "personalizations": [{
                "to": [
                    { "email": "contact@mainmatter.com", "name": "Mainmatter" }
                ],
                "bcc": [
                    { "email": "trigger@zapier.com" }
                ]
            }],
            "from": { "email": "no-reply@mainmatter.com", "name": "name via mainmatter.com" },
            "reply_to": { "email": "email@domain.tld", "name": "name" },
            "subject": "Mainmatter inquiry",
            "content": [{
                "type": "text/plain",
                "value": "Hi!"
            }]
        });

        assert_eq!(data, expected.to_string());

        Ok(200)
    }

    let payload = Payload {
        name: String::from("name"),
        email: String::from("email@domain.tld"),
        message: String::from("Hi!"),
        service: None,
        company: None,
    };
    let _result = send_message(payload, "api_key", "trigger@zapier.com", &request_sendgrid).await;
}

#[wasm_bindgen_test]
async fn it_leaves_out_the_service_if_other_is_provided() {
    async fn request_sendgrid(_api_key: &str, data: String) -> Result<u16, NetworkError> {
        let expected = json!({
            "personalizations": [{
                "to": [
                    { "email": "contact@mainmatter.com", "name": "Mainmatter" }
                ],
                "bcc": [
                    { "email": "trigger@zapier.com" }
                ]
            }],
            "from": { "email": "no-reply@mainmatter.com", "name": "name via mainmatter.com" },
            "reply_to": { "email": "email@domain.tld", "name": "name" },
            "subject": "Mainmatter inquiry",
            "content": [{
                "type": "text/plain",
                "value": "Hi!"
            }]
        });

        assert_eq!(data, expected.to_string());

        Ok(200)
    }

    let payload = Payload {
        name: String::from("name"),
        email: String::from("email@domain.tld"),
        message: String::from("Hi!"),
        service: Some(String::from("Other")),
        company: None,
    };
    let _result = send_message(payload, "api_key", "trigger@zapier.com", &request_sendgrid).await;
}

#[wasm_bindgen_test]
async fn it_adds_company_to_the_subject_if_one_is_provided() {
    async fn request_sendgrid(_api_key: &str, data: String) -> Result<u16, NetworkError> {
        let expected = json!({
            "personalizations": [{
                "to": [
                    { "email": "contact@mainmatter.com", "name": "Mainmatter" }
                ],
                "bcc": [
                    { "email": "trigger@zapier.com" }
                ]
            }],
            "from": { "email": "no-reply@mainmatter.com", "name": "name via mainmatter.com" },
            "reply_to": { "email": "email@domain.tld", "name": "name" },
            "subject": "Mainmatter inquiry from Company",
            "content": [{
                "type": "text/plain",
                "value": "Hi!"
            }]
        });

        assert_eq!(data, expected.to_string());

        Ok(200)
    }

    let payload = Payload {
        name: String::from("name"),
        email: String::from("email@domain.tld"),
        message: String::from("Hi!"),
        service: Some(String::from("Other")),
        company: Some(String::from("Company")),
    };
    let _result = send_message(payload, "api_key", "trigger@zapier.com", &request_sendgrid).await;
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
        service: Some(String::from("")),
        company: None,
    };
    let result = send_message(payload, "api_key", "trigger@zapier.com", &request_sendgrid).await;

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
        service: Some(String::from("")),
        company: None,
    };
    let result = send_message(payload, "api_key", "trigger@zapier.com", &request_sendgrid).await;

    assert_eq!(result.unwrap().status_code(), 500);
}
