//! Tests for Analytics API

mod common;

use common::*;
use whatsapp_cloud_api::analytics::Granularity;
use wiremock::matchers::{method, path, query_param};
use wiremock::{Mock, MockServer, ResponseTemplate};

#[tokio::test]
async fn test_get_conversation_analytics() {
    let mock_server = MockServer::start().await;
    let client = create_test_client(&mock_server);

    Mock::given(method("GET"))
        .and(path(format!("/v21.0/{}", TEST_WABA_ID)))
        .and(query_param("fields", "conversation_analytics"))
        .and(query_param("granularity", "DAILY"))
        .respond_with(ResponseTemplate::new(200).set_body_json(serde_json::json!({
            "conversation_analytics": {
                "data": [
                    {
                        "start": 1704067200,
                        "end": 1704153600,
                        "conversation": 150,
                        "cost": 12.50
                    },
                    {
                        "start": 1704153600,
                        "end": 1704240000,
                        "conversation": 175,
                        "cost": 14.25
                    }
                ]
            }
        })))
        .mount(&mock_server)
        .await;

    let response = client
        .analytics(TEST_WABA_ID)
        .get_conversation_analytics(1704067200, 1704240000, Granularity::Daily)
        .await
        .unwrap();

    assert_eq!(response.conversation_analytics.data.len(), 2);
    assert_eq!(response.conversation_analytics.data[0].conversation, 150);
}

#[tokio::test]
async fn test_get_conversation_analytics_half_hour() {
    let mock_server = MockServer::start().await;
    let client = create_test_client(&mock_server);

    Mock::given(method("GET"))
        .and(path(format!("/v21.0/{}", TEST_WABA_ID)))
        .and(query_param("granularity", "HALF_HOUR"))
        .respond_with(ResponseTemplate::new(200).set_body_json(serde_json::json!({
            "conversation_analytics": {
                "data": [
                    {
                        "start": 1704067200,
                        "end": 1704069000,
                        "conversation": 10
                    }
                ]
            }
        })))
        .mount(&mock_server)
        .await;

    let response = client
        .analytics(TEST_WABA_ID)
        .get_conversation_analytics(1704067200, 1704069000, Granularity::HalfHour)
        .await
        .unwrap();

    assert_eq!(response.conversation_analytics.data[0].conversation, 10);
}

#[tokio::test]
async fn test_get_conversation_analytics_monthly() {
    let mock_server = MockServer::start().await;
    let client = create_test_client(&mock_server);

    Mock::given(method("GET"))
        .and(path(format!("/v21.0/{}", TEST_WABA_ID)))
        .and(query_param("granularity", "MONTHLY"))
        .respond_with(ResponseTemplate::new(200).set_body_json(serde_json::json!({
            "conversation_analytics": {
                "data": [
                    {
                        "start": 1704067200,
                        "end": 1706745600,
                        "conversation": 5000
                    }
                ]
            }
        })))
        .mount(&mock_server)
        .await;

    let response = client
        .analytics(TEST_WABA_ID)
        .get_conversation_analytics(1704067200, 1706745600, Granularity::Monthly)
        .await
        .unwrap();

    assert_eq!(response.conversation_analytics.data[0].conversation, 5000);
}

#[tokio::test]
async fn test_get_template_analytics() {
    let mock_server = MockServer::start().await;
    let client = create_test_client(&mock_server);

    Mock::given(method("GET"))
        .and(path(format!("/v21.0/{}", TEST_WABA_ID)))
        .and(query_param("fields", "template_analytics"))
        .respond_with(ResponseTemplate::new(200).set_body_json(serde_json::json!({
            "template_analytics": {
                "data": [
                    {
                        "start": 1704067200,
                        "end": 1704153600,
                        "template_id": "template_001",
                        "sent": 1000,
                        "delivered": 950,
                        "read": 800,
                        "clicked": 200
                    }
                ]
            }
        })))
        .mount(&mock_server)
        .await;

    let response = client
        .analytics(TEST_WABA_ID)
        .get_template_analytics(1704067200, 1704153600, Granularity::Daily, None)
        .await
        .unwrap();

    assert_eq!(response.template_analytics.data[0].sent, 1000);
    assert_eq!(response.template_analytics.data[0].delivered, 950);
    assert_eq!(response.template_analytics.data[0].clicked, 200);
}

#[tokio::test]
async fn test_get_template_analytics_with_filter() {
    let mock_server = MockServer::start().await;
    let client = create_test_client(&mock_server);

    Mock::given(method("GET"))
        .and(path(format!("/v21.0/{}", TEST_WABA_ID)))
        .and(query_param("fields", "template_analytics"))
        .and(query_param("template_ids", "tpl_001,tpl_002"))
        .respond_with(ResponseTemplate::new(200).set_body_json(serde_json::json!({
            "template_analytics": {
                "data": [
                    {
                        "start": 1704067200,
                        "end": 1704153600,
                        "template_id": "tpl_001",
                        "sent": 500,
                        "delivered": 480,
                        "read": 400,
                        "clicked": 100
                    }
                ]
            }
        })))
        .mount(&mock_server)
        .await;

    let response = client
        .analytics(TEST_WABA_ID)
        .get_template_analytics(
            1704067200,
            1704153600,
            Granularity::Daily,
            Some(vec!["tpl_001", "tpl_002"]),
        )
        .await
        .unwrap();

    assert_eq!(response.template_analytics.data[0].template_id, "tpl_001");
}

#[tokio::test]
async fn test_get_phone_number_analytics() {
    let mock_server = MockServer::start().await;
    let client = create_test_client(&mock_server);

    Mock::given(method("GET"))
        .and(path(format!("/v21.0/{}", TEST_WABA_ID)))
        .and(query_param("fields", "analytics"))
        .respond_with(ResponseTemplate::new(200).set_body_json(serde_json::json!({
            "analytics": {
                "phone_number": "628123456789",
                "data": [
                    {
                        "start": 1704067200,
                        "end": 1704153600,
                        "sent": 500,
                        "delivered": 480
                    }
                ]
            }
        })))
        .mount(&mock_server)
        .await;

    let response = client
        .analytics(TEST_WABA_ID)
        .get_phone_number_analytics(1704067200, 1704153600, Granularity::Daily, None)
        .await
        .unwrap();

    assert_eq!(response.analytics.phone_number, "628123456789");
    assert_eq!(response.analytics.data[0].sent, 500);
}
