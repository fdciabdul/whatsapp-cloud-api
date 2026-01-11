//! Tests for Media API

mod common;

use common::*;
use wiremock::matchers::{method, path};
use wiremock::{Mock, MockServer, ResponseTemplate};

#[tokio::test]
async fn test_get_media_url() {
    let mock_server = MockServer::start().await;
    let client = create_test_client(&mock_server);

    Mock::given(method("GET"))
        .and(path("/v21.0/media_123"))
        .respond_with(ResponseTemplate::new(200).set_body_json(serde_json::json!({
            "url": "https://lookaside.fbsbx.com/whatsapp_business/attachments/media_123",
            "mime_type": "image/jpeg",
            "sha256": "abc123",
            "file_size": 12345,
            "id": "media_123",
            "messaging_product": "whatsapp"
        })))
        .mount(&mock_server)
        .await;

    let response = client.media().get_url("media_123").await.unwrap();

    assert_eq!(response.id, "media_123");
    assert!(response.url.contains("media_123"));
}

#[tokio::test]
async fn test_delete_media() {
    let mock_server = MockServer::start().await;
    let client = create_test_client(&mock_server);

    Mock::given(method("DELETE"))
        .and(path("/v21.0/media_123"))
        .respond_with(ResponseTemplate::new(200).set_body_json(success_response()))
        .mount(&mock_server)
        .await;

    let response = client.media().delete("media_123").await.unwrap();

    assert!(response.success);
}

#[tokio::test]
async fn test_upload_bytes() {
    let mock_server = MockServer::start().await;
    let client = create_test_client(&mock_server);

    Mock::given(method("POST"))
        .and(path(format!("/v21.0/{}/media", TEST_PHONE_ID)))
        .respond_with(ResponseTemplate::new(200).set_body_json(serde_json::json!({
            "id": "uploaded_media_123"
        })))
        .mount(&mock_server)
        .await;

    let image_bytes = vec![0xFF, 0xD8, 0xFF, 0xE0]; // JPEG magic bytes
    let response = client
        .media()
        .upload_bytes(&image_bytes, "test.jpg", "image/jpeg")
        .await
        .unwrap();

    assert_eq!(response.id, "uploaded_media_123");
}

#[tokio::test]
async fn test_upload_base64() {
    let mock_server = MockServer::start().await;
    let client = create_test_client(&mock_server);

    Mock::given(method("POST"))
        .and(path(format!("/v21.0/{}/media", TEST_PHONE_ID)))
        .respond_with(ResponseTemplate::new(200).set_body_json(serde_json::json!({
            "id": "base64_media_123"
        })))
        .mount(&mock_server)
        .await;

    let base64_data = "SGVsbG8gV29ybGQ="; // "Hello World" in base64
    let response = client
        .media()
        .upload_base64(base64_data, "test.txt", "text/plain")
        .await
        .unwrap();

    assert_eq!(response.id, "base64_media_123");
}
