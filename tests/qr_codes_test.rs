//! Tests for QR Codes API

mod common;

use common::*;
use wacloudapi::qr_codes::QrImageFormat;
use wiremock::matchers::{method, path};
use wiremock::{Mock, MockServer, ResponseTemplate};

#[tokio::test]
async fn test_create_qr_code_png() {
    let mock_server = MockServer::start().await;
    let client = create_test_client(&mock_server);

    Mock::given(method("POST"))
        .and(path(format!("/v21.0/{}/message_qrdls", TEST_PHONE_ID)))
        .respond_with(ResponseTemplate::new(200).set_body_json(serde_json::json!({
            "code": "qr_code_123",
            "prefilled_message": "Hello! I want to know more about your products.",
            "deep_link_url": "https://wa.me/message/qr_code_123",
            "qr_image_url": "https://scontent.whatsapp.net/v/qr/qr_code_123.png"
        })))
        .mount(&mock_server)
        .await;

    let response = client
        .qr_codes()
        .create(
            "Hello! I want to know more about your products.",
            QrImageFormat::Png,
        )
        .await
        .unwrap();

    assert_eq!(response.code, "qr_code_123");
    assert!(response.deep_link_url.contains("qr_code_123"));
    assert!(response.qr_image_url.is_some());
}

#[tokio::test]
async fn test_create_qr_code_svg() {
    let mock_server = MockServer::start().await;
    let client = create_test_client(&mock_server);

    Mock::given(method("POST"))
        .and(path(format!("/v21.0/{}/message_qrdls", TEST_PHONE_ID)))
        .respond_with(ResponseTemplate::new(200).set_body_json(serde_json::json!({
            "code": "qr_svg_123",
            "prefilled_message": "Hi there!",
            "deep_link_url": "https://wa.me/message/qr_svg_123",
            "qr_image_url": "https://scontent.whatsapp.net/v/qr/qr_svg_123.svg"
        })))
        .mount(&mock_server)
        .await;

    let response = client
        .qr_codes()
        .create("Hi there!", QrImageFormat::Svg)
        .await
        .unwrap();

    assert_eq!(response.code, "qr_svg_123");
}

#[tokio::test]
async fn test_list_qr_codes() {
    let mock_server = MockServer::start().await;
    let client = create_test_client(&mock_server);

    Mock::given(method("GET"))
        .and(path(format!("/v21.0/{}/message_qrdls", TEST_PHONE_ID)))
        .respond_with(ResponseTemplate::new(200).set_body_json(serde_json::json!({
            "data": [
                {
                    "code": "qr_001",
                    "prefilled_message": "Message 1",
                    "deep_link_url": "https://wa.me/message/qr_001"
                },
                {
                    "code": "qr_002",
                    "prefilled_message": "Message 2",
                    "deep_link_url": "https://wa.me/message/qr_002"
                }
            ]
        })))
        .mount(&mock_server)
        .await;

    let response = client.qr_codes().list().await.unwrap();

    assert_eq!(response.data.len(), 2);
    assert_eq!(response.data[0].code, "qr_001");
    assert_eq!(response.data[1].prefilled_message, "Message 2");
}

#[tokio::test]
async fn test_get_qr_code() {
    let mock_server = MockServer::start().await;
    let client = create_test_client(&mock_server);

    Mock::given(method("GET"))
        .and(path(format!(
            "/v21.0/{}/message_qrdls/qr_123",
            TEST_PHONE_ID
        )))
        .respond_with(ResponseTemplate::new(200).set_body_json(serde_json::json!({
            "code": "qr_123",
            "prefilled_message": "Hello from QR",
            "deep_link_url": "https://wa.me/message/qr_123"
        })))
        .mount(&mock_server)
        .await;

    let response = client.qr_codes().get("qr_123").await.unwrap();

    assert_eq!(response.code, "qr_123");
    assert_eq!(response.prefilled_message, "Hello from QR");
}

#[tokio::test]
async fn test_update_qr_code() {
    let mock_server = MockServer::start().await;
    let client = create_test_client(&mock_server);

    Mock::given(method("POST"))
        .and(path(format!(
            "/v21.0/{}/message_qrdls/qr_123",
            TEST_PHONE_ID
        )))
        .respond_with(ResponseTemplate::new(200).set_body_json(serde_json::json!({
            "code": "qr_123",
            "prefilled_message": "Updated message",
            "deep_link_url": "https://wa.me/message/qr_123"
        })))
        .mount(&mock_server)
        .await;

    let response = client
        .qr_codes()
        .update("qr_123", "Updated message")
        .await
        .unwrap();

    assert_eq!(response.prefilled_message, "Updated message");
}

#[tokio::test]
async fn test_delete_qr_code() {
    let mock_server = MockServer::start().await;
    let client = create_test_client(&mock_server);

    Mock::given(method("DELETE"))
        .and(path(format!(
            "/v21.0/{}/message_qrdls/qr_123",
            TEST_PHONE_ID
        )))
        .respond_with(ResponseTemplate::new(200).set_body_json(success_response()))
        .mount(&mock_server)
        .await;

    let response = client.qr_codes().delete("qr_123").await.unwrap();

    assert!(response.success);
}
