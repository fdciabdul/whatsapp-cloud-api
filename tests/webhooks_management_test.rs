//! Tests for Webhook Subscriptions Management API

mod common;

use common::*;
use wacloudapi::webhooks_management::SubscriptionField;
use wiremock::matchers::{method, path};
use wiremock::{Mock, MockServer, ResponseTemplate};

#[tokio::test]
async fn test_get_webhook_subscriptions() {
    let mock_server = MockServer::start().await;
    let client = create_test_client(&mock_server);

    Mock::given(method("GET"))
        .and(path(format!("/v21.0/{}/subscriptions", TEST_APP_ID)))
        .respond_with(ResponseTemplate::new(200).set_body_json(serde_json::json!({
            "data": [
                {
                    "object": "whatsapp_business_account",
                    "callback_url": "https://example.com/webhook",
                    "active": true,
                    "fields": [
                        { "name": "messages", "version": "v21.0" },
                        { "name": "message_template_status_update", "version": "v21.0" }
                    ]
                }
            ]
        })))
        .mount(&mock_server)
        .await;

    let response = client
        .webhook_subscriptions(TEST_APP_ID)
        .get()
        .await
        .unwrap();

    assert_eq!(response.data.len(), 1);
    assert!(response.data[0].active);
    assert_eq!(response.data[0].fields.len(), 2);
}

#[tokio::test]
async fn test_subscribe_webhook() {
    let mock_server = MockServer::start().await;
    let client = create_test_client(&mock_server);

    Mock::given(method("POST"))
        .and(path(format!("/v21.0/{}/subscriptions", TEST_APP_ID)))
        .respond_with(ResponseTemplate::new(200).set_body_json(success_response()))
        .mount(&mock_server)
        .await;

    let response = client
        .webhook_subscriptions(TEST_APP_ID)
        .subscribe(
            "https://example.com/webhook",
            "my_verify_token",
            vec![
                SubscriptionField::Messages,
                SubscriptionField::MessageTemplateStatusUpdate,
                SubscriptionField::Security,
            ],
        )
        .await
        .unwrap();

    assert!(response.success);
}

#[tokio::test]
async fn test_subscribe_all_fields() {
    let mock_server = MockServer::start().await;
    let client = create_test_client(&mock_server);

    Mock::given(method("POST"))
        .and(path(format!("/v21.0/{}/subscriptions", TEST_APP_ID)))
        .respond_with(ResponseTemplate::new(200).set_body_json(success_response()))
        .mount(&mock_server)
        .await;

    let response = client
        .webhook_subscriptions(TEST_APP_ID)
        .subscribe(
            "https://example.com/webhook",
            "verify_token",
            vec![
                SubscriptionField::Messages,
                SubscriptionField::MessageTemplateStatusUpdate,
                SubscriptionField::MessageTemplateQualityUpdate,
                SubscriptionField::AccountAlerts,
                SubscriptionField::AccountReviewUpdate,
                SubscriptionField::AccountUpdate,
                SubscriptionField::BusinessCapabilityUpdate,
                SubscriptionField::PhoneNumberNameUpdate,
                SubscriptionField::PhoneNumberQualityUpdate,
                SubscriptionField::Security,
                SubscriptionField::Flows,
            ],
        )
        .await
        .unwrap();

    assert!(response.success);
}

#[tokio::test]
async fn test_unsubscribe_webhook() {
    let mock_server = MockServer::start().await;
    let client = create_test_client(&mock_server);

    Mock::given(method("DELETE"))
        .and(path(format!("/v21.0/{}/subscriptions", TEST_APP_ID)))
        .respond_with(ResponseTemplate::new(200).set_body_json(success_response()))
        .mount(&mock_server)
        .await;

    let response = client
        .webhook_subscriptions(TEST_APP_ID)
        .unsubscribe("whatsapp_business_account")
        .await
        .unwrap();

    assert!(response.success);
}

#[tokio::test]
async fn test_unsubscribe_all_webhooks() {
    let mock_server = MockServer::start().await;
    let client = create_test_client(&mock_server);

    Mock::given(method("DELETE"))
        .and(path(format!("/v21.0/{}/subscriptions", TEST_APP_ID)))
        .respond_with(ResponseTemplate::new(200).set_body_json(success_response()))
        .mount(&mock_server)
        .await;

    let response = client
        .webhook_subscriptions(TEST_APP_ID)
        .unsubscribe_all()
        .await
        .unwrap();

    assert!(response.success);
}
