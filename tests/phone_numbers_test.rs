//! Tests for Phone Numbers API

mod common;

use common::*;
use whatsapp_cloud_api::phone_numbers::BusinessProfileUpdate;
use wiremock::matchers::{body_json, method, path};
use wiremock::{Mock, MockServer, ResponseTemplate};

#[tokio::test]
async fn test_get_phone_number() {
    let mock_server = MockServer::start().await;
    let client = create_test_client(&mock_server);

    Mock::given(method("GET"))
        .and(path(format!("/v21.0/{}", TEST_PHONE_ID)))
        .respond_with(ResponseTemplate::new(200).set_body_json(serde_json::json!({
            "id": TEST_PHONE_ID,
            "display_phone_number": "+62 812-3456-7890",
            "verified_name": "My Business",
            "code_verification_status": "VERIFIED",
            "quality_rating": "GREEN",
            "platform_type": "CLOUD_API",
            "throughput": {
                "level": "STANDARD"
            }
        })))
        .mount(&mock_server)
        .await;

    let response = client.phone_numbers().get(TEST_PHONE_ID).await.unwrap();

    assert_eq!(response.id, TEST_PHONE_ID);
    assert_eq!(response.display_phone_number, "+62 812-3456-7890");
}

#[tokio::test]
async fn test_register_phone_number() {
    let mock_server = MockServer::start().await;
    let client = create_test_client(&mock_server);

    Mock::given(method("POST"))
        .and(path(format!("/v21.0/{}/register", TEST_PHONE_ID)))
        .and(body_json(serde_json::json!({
            "messaging_product": "whatsapp",
            "pin": "123456"
        })))
        .respond_with(ResponseTemplate::new(200).set_body_json(success_response()))
        .mount(&mock_server)
        .await;

    let response = client.phone_numbers().register("123456").await.unwrap();

    assert!(response.success);
}

#[tokio::test]
async fn test_deregister_phone_number() {
    let mock_server = MockServer::start().await;
    let client = create_test_client(&mock_server);

    Mock::given(method("POST"))
        .and(path(format!("/v21.0/{}/deregister", TEST_PHONE_ID)))
        .respond_with(ResponseTemplate::new(200).set_body_json(success_response()))
        .mount(&mock_server)
        .await;

    let response = client.phone_numbers().deregister().await.unwrap();

    assert!(response.success);
}

#[tokio::test]
async fn test_request_verification_code_sms() {
    let mock_server = MockServer::start().await;
    let client = create_test_client(&mock_server);

    Mock::given(method("POST"))
        .and(path(format!("/v21.0/{}/request_code", TEST_PHONE_ID)))
        .and(body_json(serde_json::json!({
            "code_method": "SMS",
            "language": "en_US"
        })))
        .respond_with(ResponseTemplate::new(200).set_body_json(success_response()))
        .mount(&mock_server)
        .await;

    let response = client
        .phone_numbers()
        .request_verification_code("SMS", "en_US")
        .await
        .unwrap();

    assert!(response.success);
}

#[tokio::test]
async fn test_request_verification_code_voice() {
    let mock_server = MockServer::start().await;
    let client = create_test_client(&mock_server);

    Mock::given(method("POST"))
        .and(path(format!("/v21.0/{}/request_code", TEST_PHONE_ID)))
        .and(body_json(serde_json::json!({
            "code_method": "VOICE",
            "language": "en_US"
        })))
        .respond_with(ResponseTemplate::new(200).set_body_json(success_response()))
        .mount(&mock_server)
        .await;

    let response = client
        .phone_numbers()
        .request_verification_code("VOICE", "en_US")
        .await
        .unwrap();

    assert!(response.success);
}

#[tokio::test]
async fn test_verify_code() {
    let mock_server = MockServer::start().await;
    let client = create_test_client(&mock_server);

    Mock::given(method("POST"))
        .and(path(format!("/v21.0/{}/verify_code", TEST_PHONE_ID)))
        .and(body_json(serde_json::json!({
            "code": "123456"
        })))
        .respond_with(ResponseTemplate::new(200).set_body_json(success_response()))
        .mount(&mock_server)
        .await;

    let response = client.phone_numbers().verify_code("123456").await.unwrap();

    assert!(response.success);
}

#[tokio::test]
async fn test_set_two_step_verification() {
    let mock_server = MockServer::start().await;
    let client = create_test_client(&mock_server);

    Mock::given(method("POST"))
        .and(path(format!("/v21.0/{}", TEST_PHONE_ID)))
        .and(body_json(serde_json::json!({
            "pin": "654321"
        })))
        .respond_with(ResponseTemplate::new(200).set_body_json(success_response()))
        .mount(&mock_server)
        .await;

    let response = client
        .phone_numbers()
        .set_two_step_verification("654321")
        .await
        .unwrap();

    assert!(response.success);
}

#[tokio::test]
async fn test_get_business_profile() {
    let mock_server = MockServer::start().await;
    let client = create_test_client(&mock_server);

    Mock::given(method("GET"))
        .and(path(format!(
            "/v21.0/{}/whatsapp_business_profile",
            TEST_PHONE_ID
        )))
        .respond_with(ResponseTemplate::new(200).set_body_json(serde_json::json!({
            "data": [{
                "about": "Welcome to our business!",
                "address": "123 Business Street",
                "description": "We provide the best services.",
                "email": "contact@business.com",
                "profile_picture_url": "https://example.com/profile.jpg",
                "websites": ["https://business.com"],
                "vertical": "RETAIL"
            }]
        })))
        .mount(&mock_server)
        .await;

    let response = client.phone_numbers().get_business_profile().await.unwrap();

    assert_eq!(
        response.data[0].about.as_ref().unwrap(),
        "Welcome to our business!"
    );
    assert_eq!(response.data[0].vertical.as_ref().unwrap(), "RETAIL");
}

#[tokio::test]
async fn test_update_business_profile() {
    let mock_server = MockServer::start().await;
    let client = create_test_client(&mock_server);

    Mock::given(method("POST"))
        .and(path(format!(
            "/v21.0/{}/whatsapp_business_profile",
            TEST_PHONE_ID
        )))
        .respond_with(ResponseTemplate::new(200).set_body_json(success_response()))
        .mount(&mock_server)
        .await;

    let profile = BusinessProfileUpdate::new()
        .about("Updated business description")
        .address("456 New Street")
        .email("new@business.com");

    let response = client
        .phone_numbers()
        .update_business_profile(&profile)
        .await
        .unwrap();

    assert!(response.success);
}
