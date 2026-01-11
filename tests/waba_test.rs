//! Tests for WABA Management API

mod common;

use common::*;
use whatsapp_cloud_api::waba::WebhookField;
use wiremock::matchers::{method, path};
use wiremock::{Mock, MockServer, ResponseTemplate};

#[tokio::test]
async fn test_get_waba() {
    let mock_server = MockServer::start().await;
    let client = create_test_client(&mock_server);

    Mock::given(method("GET"))
        .and(path(format!("/v21.0/{}", TEST_WABA_ID)))
        .respond_with(ResponseTemplate::new(200).set_body_json(serde_json::json!({
            "id": TEST_WABA_ID,
            "name": "My Business Account",
            "timezone_id": "Asia/Jakarta",
            "message_template_namespace": "namespace_123",
            "account_review_status": "APPROVED",
            "business_verification_status": "VERIFIED"
        })))
        .mount(&mock_server)
        .await;

    let response = client.waba(TEST_WABA_ID).get().await.unwrap();

    assert_eq!(response.id, TEST_WABA_ID);
    assert_eq!(response.name.unwrap(), "My Business Account");
    assert_eq!(response.account_review_status.unwrap(), "APPROVED");
}

#[tokio::test]
async fn test_subscribe_webhooks() {
    let mock_server = MockServer::start().await;
    let client = create_test_client(&mock_server);

    Mock::given(method("POST"))
        .and(path(format!("/v21.0/{}/subscribed_apps", TEST_WABA_ID)))
        .respond_with(ResponseTemplate::new(200).set_body_json(success_response()))
        .mount(&mock_server)
        .await;

    let response = client.waba(TEST_WABA_ID).subscribe_webhooks().await.unwrap();

    assert!(response.success);
}

#[tokio::test]
async fn test_subscribe_specific_fields() {
    let mock_server = MockServer::start().await;
    let client = create_test_client(&mock_server);

    Mock::given(method("POST"))
        .and(path(format!("/v21.0/{}/subscribed_apps", TEST_WABA_ID)))
        .respond_with(ResponseTemplate::new(200).set_body_json(success_response()))
        .mount(&mock_server)
        .await;

    let response = client
        .waba(TEST_WABA_ID)
        .subscribe_fields(vec![
            WebhookField::Messages,
            WebhookField::MessageTemplateStatusUpdate,
            WebhookField::Security,
        ])
        .await
        .unwrap();

    assert!(response.success);
}

#[tokio::test]
async fn test_unsubscribe_webhooks() {
    let mock_server = MockServer::start().await;
    let client = create_test_client(&mock_server);

    Mock::given(method("DELETE"))
        .and(path(format!("/v21.0/{}/subscribed_apps", TEST_WABA_ID)))
        .respond_with(ResponseTemplate::new(200).set_body_json(success_response()))
        .mount(&mock_server)
        .await;

    let response = client
        .waba(TEST_WABA_ID)
        .unsubscribe_webhooks()
        .await
        .unwrap();

    assert!(response.success);
}

#[tokio::test]
async fn test_get_subscribed_apps() {
    let mock_server = MockServer::start().await;
    let client = create_test_client(&mock_server);

    Mock::given(method("GET"))
        .and(path(format!("/v21.0/{}/subscribed_apps", TEST_WABA_ID)))
        .respond_with(ResponseTemplate::new(200).set_body_json(serde_json::json!({
            "data": [
                {
                    "id": "app_123",
                    "name": "My WhatsApp App",
                    "subscribed_fields": ["messages", "message_template_status_update"]
                }
            ]
        })))
        .mount(&mock_server)
        .await;

    let response = client.waba(TEST_WABA_ID).get_subscribed_apps().await.unwrap();

    assert_eq!(response.data.len(), 1);
    assert_eq!(response.data[0].id, "app_123");
    assert!(response.data[0].subscribed_fields.contains(&"messages".to_string()));
}

#[tokio::test]
async fn test_get_phone_numbers() {
    let mock_server = MockServer::start().await;
    let client = create_test_client(&mock_server);

    Mock::given(method("GET"))
        .and(path(format!("/v21.0/{}/phone_numbers", TEST_WABA_ID)))
        .respond_with(ResponseTemplate::new(200).set_body_json(serde_json::json!({
            "data": [
                {
                    "id": "phone_001",
                    "display_phone_number": "+62 812-3456-7890",
                    "verified_name": "My Business",
                    "quality_rating": "GREEN"
                },
                {
                    "id": "phone_002",
                    "display_phone_number": "+62 821-9876-5432",
                    "verified_name": "Support Line",
                    "quality_rating": "GREEN"
                }
            ]
        })))
        .mount(&mock_server)
        .await;

    let response = client.waba(TEST_WABA_ID).get_phone_numbers().await.unwrap();

    assert_eq!(response.data.len(), 2);
    assert_eq!(response.data[0].id, "phone_001");
    assert_eq!(response.data[0].quality_rating.as_ref().unwrap(), "GREEN");
}

#[tokio::test]
async fn test_get_assigned_users() {
    let mock_server = MockServer::start().await;
    let client = create_test_client(&mock_server);

    Mock::given(method("GET"))
        .and(path(format!("/v21.0/{}/assigned_users", TEST_WABA_ID)))
        .respond_with(ResponseTemplate::new(200).set_body_json(serde_json::json!({
            "data": [
                {
                    "id": "user_001",
                    "name": "John Admin",
                    "tasks": ["MANAGE", "DEVELOP"]
                },
                {
                    "id": "user_002",
                    "name": "Jane Developer",
                    "tasks": ["DEVELOP"]
                }
            ]
        })))
        .mount(&mock_server)
        .await;

    let response = client.waba(TEST_WABA_ID).get_assigned_users().await.unwrap();

    assert_eq!(response.data.len(), 2);
    assert!(response.data[0].tasks.contains(&"MANAGE".to_string()));
}

#[tokio::test]
async fn test_get_system_users() {
    let mock_server = MockServer::start().await;
    let client = create_test_client(&mock_server);

    Mock::given(method("GET"))
        .and(path(format!("/v21.0/{}/system_users", TEST_WABA_ID)))
        .respond_with(ResponseTemplate::new(200).set_body_json(serde_json::json!({
            "data": [
                {
                    "id": "sys_user_001",
                    "name": "API System User",
                    "role": "ADMIN"
                }
            ]
        })))
        .mount(&mock_server)
        .await;

    let response = client.waba(TEST_WABA_ID).get_system_users().await.unwrap();

    assert_eq!(response.data.len(), 1);
    assert_eq!(response.data[0].role.as_ref().unwrap(), "ADMIN");
}

#[tokio::test]
async fn test_get_templates() {
    let mock_server = MockServer::start().await;
    let client = create_test_client(&mock_server);

    Mock::given(method("GET"))
        .and(path(format!("/v21.0/{}/message_templates", TEST_WABA_ID)))
        .respond_with(ResponseTemplate::new(200).set_body_json(serde_json::json!({
            "data": [
                {
                    "id": "tpl_001",
                    "name": "hello_world",
                    "status": "APPROVED",
                    "category": "UTILITY",
                    "language": "en_US"
                },
                {
                    "id": "tpl_002",
                    "name": "order_confirmation",
                    "status": "APPROVED",
                    "category": "UTILITY",
                    "language": "en_US"
                }
            ],
            "paging": {
                "cursors": {
                    "after": "cursor_abc123"
                }
            }
        })))
        .mount(&mock_server)
        .await;

    let response = client.waba(TEST_WABA_ID).get_templates().await.unwrap();

    assert_eq!(response.data.len(), 2);
    assert_eq!(response.data[0].name, "hello_world");
    assert_eq!(response.data[0].status, "APPROVED");
}
