//! Tests for Typing Indicators API

mod common;

use common::*;
use wiremock::matchers::{body_json, method, path};
use wiremock::{Mock, MockServer, ResponseTemplate};

#[tokio::test]
async fn test_show_typing_indicator() {
    let mock_server = MockServer::start().await;
    let client = create_test_client(&mock_server);

    Mock::given(method("POST"))
        .and(path(format!("/v21.0/{}/messages", TEST_PHONE_ID)))
        .and(body_json(serde_json::json!({
            "messaging_product": "whatsapp",
            "recipient_type": "individual",
            "to": "628123456789",
            "status": "typing"
        })))
        .respond_with(ResponseTemplate::new(200).set_body_json(success_response()))
        .mount(&mock_server)
        .await;

    let response = client.typing().show("628123456789").await.unwrap();

    assert!(response.success);
}
