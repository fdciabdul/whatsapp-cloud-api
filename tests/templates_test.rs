//! Tests for Templates API

mod common;

use common::*;
use whatsapp_cloud_api::templates::{CreateTemplate, TemplateCategory, TemplateStatus};
use wiremock::matchers::{method, path, query_param};
use wiremock::{Mock, MockServer, ResponseTemplate};

#[tokio::test]
async fn test_list_templates() {
    let mock_server = MockServer::start().await;
    let client = create_test_client(&mock_server);

    Mock::given(method("GET"))
        .and(path(format!("/v21.0/{}/message_templates", TEST_WABA_ID)))
        .respond_with(ResponseTemplate::new(200).set_body_json(serde_json::json!({
            "data": [
                {
                    "name": "hello_world",
                    "status": "APPROVED",
                    "category": "UTILITY",
                    "language": "en_US",
                    "id": "tpl_001",
                    "components": [
                        { "type": "BODY", "text": "Hello!" }
                    ]
                },
                {
                    "name": "order_update",
                    "status": "PENDING",
                    "category": "UTILITY",
                    "language": "en_US",
                    "id": "tpl_002",
                    "components": [
                        { "type": "BODY", "text": "Order update" }
                    ]
                }
            ],
            "paging": {
                "cursors": {
                    "before": "cursor_before",
                    "after": "cursor_after"
                }
            }
        })))
        .mount(&mock_server)
        .await;

    let response = client.templates().list(TEST_WABA_ID).await.unwrap();

    assert_eq!(response.data.len(), 2);
    assert_eq!(response.data[0].name, "hello_world");
    assert_eq!(response.data[1].status, "PENDING");
}

#[tokio::test]
async fn test_list_templates_by_status() {
    let mock_server = MockServer::start().await;
    let client = create_test_client(&mock_server);

    Mock::given(method("GET"))
        .and(path(format!("/v21.0/{}/message_templates", TEST_WABA_ID)))
        .respond_with(ResponseTemplate::new(200).set_body_json(serde_json::json!({
            "data": [
                {
                    "name": "hello_world",
                    "status": "APPROVED",
                    "category": "UTILITY",
                    "language": "en_US",
                    "id": "tpl_001",
                    "components": [
                        { "type": "BODY", "text": "Hello!" }
                    ]
                }
            ]
        })))
        .mount(&mock_server)
        .await;

    let response = client
        .templates()
        .list_by_status(TEST_WABA_ID, TemplateStatus::Approved)
        .await
        .unwrap();

    assert_eq!(response.data.len(), 1);
    assert_eq!(response.data[0].status, "APPROVED");
}

#[tokio::test]
async fn test_get_template_by_name() {
    let mock_server = MockServer::start().await;
    let client = create_test_client(&mock_server);

    Mock::given(method("GET"))
        .and(path(format!("/v21.0/{}/message_templates", TEST_WABA_ID)))
        .and(query_param("name", "hello_world"))
        .respond_with(ResponseTemplate::new(200).set_body_json(serde_json::json!({
            "data": [
                {
                    "name": "hello_world",
                    "status": "APPROVED",
                    "category": "UTILITY",
                    "language": "en_US",
                    "id": "tpl_001",
                    "components": [
                        {
                            "type": "BODY",
                            "text": "Hello {{1}}! Welcome to our service."
                        }
                    ]
                }
            ]
        })))
        .mount(&mock_server)
        .await;

    let response = client
        .templates()
        .get_by_name(TEST_WABA_ID, "hello_world")
        .await
        .unwrap();

    assert_eq!(response.data.len(), 1);
    assert_eq!(response.data[0].name, "hello_world");
}

#[tokio::test]
async fn test_create_template() {
    let mock_server = MockServer::start().await;
    let client = create_test_client(&mock_server);

    Mock::given(method("POST"))
        .and(path(format!("/v21.0/{}/message_templates", TEST_WABA_ID)))
        .respond_with(ResponseTemplate::new(200).set_body_json(serde_json::json!({
            "id": "new_tpl_123",
            "status": "PENDING",
            "category": "UTILITY"
        })))
        .mount(&mock_server)
        .await;

    let template = CreateTemplate::new("my_new_template", TemplateCategory::Utility, "en_US")
        .with_body("Hello {{1}}!");

    let response = client
        .templates()
        .create(TEST_WABA_ID, &template)
        .await
        .unwrap();

    assert_eq!(response.id, "new_tpl_123");
    assert_eq!(response.status, "PENDING");
}

#[tokio::test]
async fn test_delete_template() {
    let mock_server = MockServer::start().await;
    let client = create_test_client(&mock_server);

    Mock::given(method("DELETE"))
        .and(path(format!("/v21.0/{}/message_templates", TEST_WABA_ID)))
        .and(query_param("name", "old_template"))
        .respond_with(ResponseTemplate::new(200).set_body_json(success_response()))
        .mount(&mock_server)
        .await;

    let response = client
        .templates()
        .delete(TEST_WABA_ID, "old_template")
        .await
        .unwrap();

    assert!(response.success);
}
