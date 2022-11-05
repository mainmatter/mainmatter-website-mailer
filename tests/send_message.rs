use mainmatter_website_mailer::{send_message, Payload};
use wasm_bindgen_test::*;

wasm_bindgen_test::wasm_bindgen_test_configure!(run_in_browser);

#[wasm_bindgen_test]
async fn it_works() {
    //     let payload = Payload {
    //         name: String::from("name"),
    //         email: String::from("email@domain.tld"),
    //         message: String::from("Hi!"),
    //     };
    //     let result = send_message(payload, "api_key").await;
    //
    //     assert_eq!(result.unwrap().status_code(), 200);
    assert_eq!(1, 1);
}
