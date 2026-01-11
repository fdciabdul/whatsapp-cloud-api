//! Tests for Block Users API

mod common;

use common::*;
use wiremock::matchers::{body_json, method, path};
use wiremock::{Mock, MockServer, ResponseTemplate};

#[tokio::test]
async fn test_block_user() {
    let mock_server = MockServer::start().await;
    let client = create_test_client(&mock_server);

    Mock::given(method("POST"))
        .and(path(format!("/v21.0/{}/block", TEST_PHONE_ID)))
        .and(body_json(serde_json::json!({
            "messaging_product": "whatsapp",
            "block": [
                { "user": "628111222333" }
            ]
        })))
        .respond_with(ResponseTemplate::new(200).set_body_json(serde_json::json!({
            "data": [
                {
                    "input": "628111222333",
                    "wa_id": "628111222333",
                    "success": true
                }
            ]
        })))
        .mount(&mock_server)
        .await;

    let response = client.block().block_user("628111222333").await.unwrap();

    assert_eq!(response.data.len(), 1);
    assert!(response.data[0].success);
    assert_eq!(response.data[0].input, "628111222333");
}

#[tokio::test]
async fn test_block_multiple_users() {
    let mock_server = MockServer::start().await;
    let client = create_test_client(&mock_server);

    Mock::given(method("POST"))
        .and(path(format!("/v21.0/{}/block", TEST_PHONE_ID)))
        .and(body_json(serde_json::json!({
            "messaging_product": "whatsapp",
            "block": [
                { "user": "628111111111" },
                { "user": "628222222222" },
                { "user": "628333333333" }
            ]
        })))
        .respond_with(ResponseTemplate::new(200).set_body_json(serde_json::json!({
            "data": [
                { "input": "628111111111", "wa_id": "628111111111", "success": true },
                { "input": "628222222222", "wa_id": "628222222222", "success": true },
                { "input": "628333333333", "wa_id": "628333333333", "success": true }
            ]
        })))
        .mount(&mock_server)
        .await;

    let response = client
        .block()
        .block_users(vec!["628111111111", "628222222222", "628333333333"])
        .await
        .unwrap();

    assert_eq!(response.data.len(), 3);
    assert!(response.data.iter().all(|r| r.success));
}

#[tokio::test]
async fn test_unblock_user() {
    let mock_server = MockServer::start().await;
    let client = create_test_client(&mock_server);

    Mock::given(method("POST"))
        .and(path(format!("/v21.0/{}/block", TEST_PHONE_ID)))
        .and(body_json(serde_json::json!({
            "messaging_product": "whatsapp",
            "unblock": [
                { "user": "628111222333" }
            ]
        })))
        .respond_with(ResponseTemplate::new(200).set_body_json(serde_json::json!({
            "data": [
                {
                    "input": "628111222333",
                    "wa_id": "628111222333",
                    "success": true
                }
            ]
        })))
        .mount(&mock_server)
        .await;

    let response = client.block().unblock_user("628111222333").await.unwrap();

    assert!(response.data[0].success);
}

#[tokio::test]
async fn test_unblock_multiple_users() {
    let mock_server = MockServer::start().await;
    let client = create_test_client(&mock_server);

    Mock::given(method("POST"))
        .and(path(format!("/v21.0/{}/block", TEST_PHONE_ID)))
        .and(body_json(serde_json::json!({
            "messaging_product": "whatsapp",
            "unblock": [
                { "user": "628111111111" },
                { "user": "628222222222" }
            ]
        })))
        .respond_with(ResponseTemplate::new(200).set_body_json(serde_json::json!({
            "data": [
                { "input": "628111111111", "wa_id": "628111111111", "success": true },
                { "input": "628222222222", "wa_id": "628222222222", "success": true }
            ]
        })))
        .mount(&mock_server)
        .await;

    let response = client
        .block()
        .unblock_users(vec!["628111111111", "628222222222"])
        .await
        .unwrap();

    assert_eq!(response.data.len(), 2);
}

#[tokio::test]
async fn test_get_blocked_users() {
    let mock_server = MockServer::start().await;
    let client = create_test_client(&mock_server);

    Mock::given(method("GET"))
        .and(path(format!("/v21.0/{}/block", TEST_PHONE_ID)))
        .respond_with(ResponseTemplate::new(200).set_body_json(serde_json::json!({
            "data": [
                { "wa_id": "628111111111" },
                { "wa_id": "628222222222" },
                { "wa_id": "628333333333" }
            ]
        })))
        .mount(&mock_server)
        .await;

    let response = client.block().get_blocked_users().await.unwrap();

    assert_eq!(response.data.len(), 3);
    assert_eq!(response.data[0].wa_id, "628111111111");
    assert_eq!(response.data[2].wa_id, "628333333333");
}

#[tokio::test]
async fn test_block_user_failure() {
    let mock_server = MockServer::start().await;
    let client = create_test_client(&mock_server);

    Mock::given(method("POST"))
        .and(path(format!("/v21.0/{}/block", TEST_PHONE_ID)))
        .respond_with(ResponseTemplate::new(200).set_body_json(serde_json::json!({
            "data": [
                {
                    "input": "invalid_number",
                    "success": false
                }
            ]
        })))
        .mount(&mock_server)
        .await;

    let response = client.block().block_user("invalid_number").await.unwrap();

    assert!(!response.data[0].success);
}
