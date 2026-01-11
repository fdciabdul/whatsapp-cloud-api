//! Tests for Flows API

mod common;

use common::*;
use whatsapp_cloud_api::flows::{FlowAction, FlowCategory};
use wiremock::matchers::{method, path};
use wiremock::{Mock, MockServer, ResponseTemplate};

#[tokio::test]
async fn test_send_flow() {
    let mock_server = MockServer::start().await;
    let client = create_test_client(&mock_server);

    Mock::given(method("POST"))
        .and(path(format!("/v21.0/{}/messages", TEST_PHONE_ID)))
        .respond_with(ResponseTemplate::new(200).set_body_json(message_response("wamid.flow123")))
        .mount(&mock_server)
        .await;

    let response = client
        .flows()
        .send_flow(
            "628123456789",
            "flow_token_abc",
            "flow_123",
            "Start Survey",
            FlowAction::Navigate,
            "WELCOME_SCREEN",
            None,
            Some("Survey"),
            "Please complete this survey",
            Some("Takes 2 minutes"),
        )
        .await
        .unwrap();

    assert_eq!(response.messages[0].id, "wamid.flow123");
}

#[tokio::test]
async fn test_send_flow_with_data() {
    let mock_server = MockServer::start().await;
    let client = create_test_client(&mock_server);

    Mock::given(method("POST"))
        .and(path(format!("/v21.0/{}/messages", TEST_PHONE_ID)))
        .respond_with(ResponseTemplate::new(200).set_body_json(message_response("wamid.flowdata123")))
        .mount(&mock_server)
        .await;

    let flow_data = serde_json::json!({
        "user_name": "John",
        "order_id": "ORD123"
    });

    let response = client
        .flows()
        .send_flow(
            "628123456789",
            "flow_token_abc",
            "flow_123",
            "Track Order",
            FlowAction::DataExchange,
            "ORDER_STATUS",
            Some(flow_data),
            None,
            "Track your order status",
            None,
        )
        .await
        .unwrap();

    assert_eq!(response.messages[0].id, "wamid.flowdata123");
}

#[tokio::test]
async fn test_list_flows() {
    let mock_server = MockServer::start().await;
    let client = create_test_client(&mock_server);

    Mock::given(method("GET"))
        .and(path(format!("/v21.0/{}/flows", TEST_WABA_ID)))
        .respond_with(ResponseTemplate::new(200).set_body_json(serde_json::json!({
            "data": [
                {
                    "id": "flow_001",
                    "name": "Customer Survey",
                    "status": "PUBLISHED",
                    "categories": ["SURVEY"]
                },
                {
                    "id": "flow_002",
                    "name": "Lead Gen Form",
                    "status": "DRAFT",
                    "categories": ["LEAD_GENERATION"]
                }
            ]
        })))
        .mount(&mock_server)
        .await;

    let response = client.flows().list_flows(TEST_WABA_ID).await.unwrap();

    assert_eq!(response.data.len(), 2);
    assert_eq!(response.data[0].name, "Customer Survey");
    assert_eq!(response.data[1].status, "DRAFT");
}

#[tokio::test]
async fn test_get_flow() {
    let mock_server = MockServer::start().await;
    let client = create_test_client(&mock_server);

    Mock::given(method("GET"))
        .and(path("/v21.0/flow_123"))
        .respond_with(ResponseTemplate::new(200).set_body_json(serde_json::json!({
            "id": "flow_123",
            "name": "My Flow",
            "status": "PUBLISHED",
            "categories": ["SURVEY"],
            "json_version": "3.0",
            "data_api_version": "3.0"
        })))
        .mount(&mock_server)
        .await;

    let response = client.flows().get_flow("flow_123").await.unwrap();

    assert_eq!(response.id, "flow_123");
    assert_eq!(response.name, "My Flow");
    assert_eq!(response.status, "PUBLISHED");
}

#[tokio::test]
async fn test_create_flow() {
    let mock_server = MockServer::start().await;
    let client = create_test_client(&mock_server);

    Mock::given(method("POST"))
        .and(path(format!("/v21.0/{}/flows", TEST_WABA_ID)))
        .respond_with(ResponseTemplate::new(200).set_body_json(serde_json::json!({
            "id": "new_flow_123"
        })))
        .mount(&mock_server)
        .await;

    let response = client
        .flows()
        .create_flow(
            TEST_WABA_ID,
            "New Survey Flow",
            vec![FlowCategory::Survey, FlowCategory::LeadGeneration],
        )
        .await
        .unwrap();

    assert_eq!(response.id, "new_flow_123");
}

#[tokio::test]
async fn test_publish_flow() {
    let mock_server = MockServer::start().await;
    let client = create_test_client(&mock_server);

    Mock::given(method("POST"))
        .and(path("/v21.0/flow_123"))
        .respond_with(ResponseTemplate::new(200).set_body_json(success_response()))
        .mount(&mock_server)
        .await;

    let response = client.flows().publish_flow("flow_123").await.unwrap();

    assert!(response.success);
}

#[tokio::test]
async fn test_deprecate_flow() {
    let mock_server = MockServer::start().await;
    let client = create_test_client(&mock_server);

    Mock::given(method("POST"))
        .and(path("/v21.0/flow_123"))
        .respond_with(ResponseTemplate::new(200).set_body_json(success_response()))
        .mount(&mock_server)
        .await;

    let response = client.flows().deprecate_flow("flow_123").await.unwrap();

    assert!(response.success);
}

#[tokio::test]
async fn test_delete_flow() {
    let mock_server = MockServer::start().await;
    let client = create_test_client(&mock_server);

    Mock::given(method("DELETE"))
        .and(path("/v21.0/flow_123"))
        .respond_with(ResponseTemplate::new(200).set_body_json(success_response()))
        .mount(&mock_server)
        .await;

    let response = client.flows().delete_flow("flow_123").await.unwrap();

    assert!(response.success);
}

#[tokio::test]
async fn test_get_flow_preview() {
    let mock_server = MockServer::start().await;
    let client = create_test_client(&mock_server);

    Mock::given(method("GET"))
        .and(path("/v21.0/flow_123/preview"))
        .respond_with(ResponseTemplate::new(200).set_body_json(serde_json::json!({
            "preview_url": "https://wa.me/flow/preview/flow_123",
            "expires_at": "2025-01-15T00:00:00Z"
        })))
        .mount(&mock_server)
        .await;

    let response = client.flows().get_preview("flow_123").await.unwrap();

    assert!(response.preview_url.contains("flow_123"));
}
