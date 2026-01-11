//! Tests for Client

mod common;

use common::*;
use wacloudapi::Client;
use wiremock::matchers::{header, method, path};
use wiremock::{Mock, MockServer, ResponseTemplate};

#[test]
fn test_client_new() {
    let client = Client::new("test_token", "phone_123");

    assert_eq!(client.phone_number_id(), "phone_123");
    assert_eq!(client.api_version(), "v21.0");
}

#[test]
fn test_client_with_version() {
    let client = Client::with_version("test_token", "phone_123", "v20.0");

    assert_eq!(client.api_version(), "v20.0");
}

#[test]
fn test_client_base_url() {
    let client = Client::with_config("test_token", "phone_123", "v21.0", "https://custom.api.com");

    assert_eq!(client.base_url(), "https://custom.api.com/v21.0/phone_123");
}

#[test]
fn test_client_endpoint_url() {
    let client = Client::with_config(
        "test_token",
        "phone_123",
        "v21.0",
        "https://graph.facebook.com",
    );

    assert_eq!(
        client.endpoint_url("media_456"),
        "https://graph.facebook.com/v21.0/media_456"
    );
}

#[test]
fn test_client_clone() {
    let client1 = Client::new("test_token", "phone_123");
    let client2 = client1.clone();

    assert_eq!(client1.phone_number_id(), client2.phone_number_id());
}

#[test]
fn test_client_debug() {
    let client = Client::new("secret_token", "phone_123");
    let debug_str = format!("{:?}", client);

    // Ensure token is not exposed in debug output
    assert!(!debug_str.contains("secret_token"));
    assert!(debug_str.contains("phone_123"));
}

#[tokio::test]
async fn test_api_error_handling() {
    let mock_server = MockServer::start().await;
    let client = create_test_client(&mock_server);

    Mock::given(method("POST"))
        .and(path(format!("/v21.0/{}/messages", TEST_PHONE_ID)))
        .respond_with(ResponseTemplate::new(400).set_body_json(serde_json::json!({
            "error": {
                "message": "Invalid phone number",
                "type": "OAuthException",
                "code": 100,
                "error_subcode": 33,
                "fbtrace_id": "trace123"
            }
        })))
        .mount(&mock_server)
        .await;

    let result = client.messages().send_text("invalid", "Hello").await;

    assert!(result.is_err());
    let error = result.unwrap_err();
    assert!(
        error.to_string().contains("Invalid phone number") || error.to_string().contains("100")
    );
}

#[tokio::test]
async fn test_rate_limit_error() {
    let mock_server = MockServer::start().await;
    let client = create_test_client(&mock_server);

    Mock::given(method("POST"))
        .and(path(format!("/v21.0/{}/messages", TEST_PHONE_ID)))
        .respond_with(ResponseTemplate::new(429).set_body_json(serde_json::json!({
            "error": {
                "message": "Rate limit exceeded",
                "type": "OAuthException",
                "code": 80007,
                "fbtrace_id": "trace456"
            }
        })))
        .mount(&mock_server)
        .await;

    let result = client.messages().send_text("628123456789", "Hello").await;

    assert!(result.is_err());
}

#[tokio::test]
async fn test_unauthorized_error() {
    let mock_server = MockServer::start().await;
    let client = create_test_client(&mock_server);

    Mock::given(method("POST"))
        .and(path(format!("/v21.0/{}/messages", TEST_PHONE_ID)))
        .respond_with(ResponseTemplate::new(401).set_body_json(serde_json::json!({
            "error": {
                "message": "Invalid OAuth access token",
                "type": "OAuthException",
                "code": 190,
                "fbtrace_id": "trace789"
            }
        })))
        .mount(&mock_server)
        .await;

    let result = client.messages().send_text("628123456789", "Hello").await;

    assert!(result.is_err());
}

#[tokio::test]
async fn test_authorization_header() {
    let mock_server = MockServer::start().await;
    let client = create_test_client(&mock_server);

    Mock::given(method("POST"))
        .and(path(format!("/v21.0/{}/messages", TEST_PHONE_ID)))
        .and(header("Authorization", format!("Bearer {}", TEST_TOKEN)))
        .respond_with(ResponseTemplate::new(200).set_body_json(message_response("wamid.auth123")))
        .mount(&mock_server)
        .await;

    let result = client.messages().send_text("628123456789", "Hello").await;

    assert!(result.is_ok());
}
