//! Tests for Products/Catalog API

mod common;

use common::*;
use whatsapp_cloud_api::products::{ProductItem, ProductSection};
use wiremock::matchers::{method, path};
use wiremock::{Mock, MockServer, ResponseTemplate};

#[tokio::test]
async fn test_send_product() {
    let mock_server = MockServer::start().await;
    let client = create_test_client(&mock_server);

    Mock::given(method("POST"))
        .and(path(format!("/v21.0/{}/messages", TEST_PHONE_ID)))
        .respond_with(ResponseTemplate::new(200).set_body_json(message_response("wamid.product123")))
        .mount(&mock_server)
        .await;

    let response = client
        .products()
        .send_product(
            "628123456789",
            "catalog_123",
            "product_456",
            "Check out this product!",
            Some("Powered by WhatsApp"),
        )
        .await
        .unwrap();

    assert_eq!(response.messages[0].id, "wamid.product123");
}

#[tokio::test]
async fn test_send_product_list() {
    let mock_server = MockServer::start().await;
    let client = create_test_client(&mock_server);

    Mock::given(method("POST"))
        .and(path(format!("/v21.0/{}/messages", TEST_PHONE_ID)))
        .respond_with(ResponseTemplate::new(200).set_body_json(message_response("wamid.prodlist123")))
        .mount(&mock_server)
        .await;

    let sections = vec![
        ProductSection::new(
            "Electronics",
            vec![
                ProductItem::new("phone_001"),
                ProductItem::new("laptop_002"),
            ],
        ),
        ProductSection::new(
            "Accessories",
            vec![
                ProductItem::new("case_001"),
                ProductItem::new("charger_002"),
            ],
        ),
    ];

    let response = client
        .products()
        .send_product_list(
            "628123456789",
            "catalog_123",
            "Our Products",
            "Browse our catalog",
            Some("Free shipping!"),
            sections,
        )
        .await
        .unwrap();

    assert_eq!(response.messages[0].id, "wamid.prodlist123");
}

#[tokio::test]
async fn test_send_catalog() {
    let mock_server = MockServer::start().await;
    let client = create_test_client(&mock_server);

    Mock::given(method("POST"))
        .and(path(format!("/v21.0/{}/messages", TEST_PHONE_ID)))
        .respond_with(ResponseTemplate::new(200).set_body_json(message_response("wamid.catalog123")))
        .mount(&mock_server)
        .await;

    let response = client
        .products()
        .send_catalog(
            "628123456789",
            "View our full catalog",
            Some("Shop now!"),
            Some("featured_product_001"),
        )
        .await
        .unwrap();

    assert_eq!(response.messages[0].id, "wamid.catalog123");
}

#[tokio::test]
async fn test_get_commerce_settings() {
    let mock_server = MockServer::start().await;
    let client = create_test_client(&mock_server);

    Mock::given(method("GET"))
        .and(path(format!(
            "/v21.0/{}/whatsapp_commerce_settings",
            TEST_PHONE_ID
        )))
        .respond_with(ResponseTemplate::new(200).set_body_json(serde_json::json!({
            "is_catalog_visible": true,
            "is_cart_enabled": true,
            "id": "commerce_123"
        })))
        .mount(&mock_server)
        .await;

    let response = client.products().get_commerce_settings().await.unwrap();

    assert!(response.is_catalog_visible);
    assert!(response.is_cart_enabled);
}

#[tokio::test]
async fn test_update_commerce_settings() {
    let mock_server = MockServer::start().await;
    let client = create_test_client(&mock_server);

    Mock::given(method("POST"))
        .and(path(format!(
            "/v21.0/{}/whatsapp_commerce_settings",
            TEST_PHONE_ID
        )))
        .respond_with(ResponseTemplate::new(200).set_body_json(success_response()))
        .mount(&mock_server)
        .await;

    let response = client
        .products()
        .update_commerce_settings(true, false)
        .await
        .unwrap();

    assert!(response.success);
}
