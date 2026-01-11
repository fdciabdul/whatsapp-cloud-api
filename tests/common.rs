//! Common test utilities and mock server setup

use whatsapp_cloud_api::Client;
use wiremock::MockServer;

/// Test phone number ID
pub const TEST_PHONE_ID: &str = "123456789";
/// Test access token
pub const TEST_TOKEN: &str = "test_access_token";
/// Test WABA ID
pub const TEST_WABA_ID: &str = "987654321";
/// Test App ID
pub const TEST_APP_ID: &str = "app_123456";

/// Create a test client connected to mock server
pub fn create_test_client(mock_server: &MockServer) -> Client {
    Client::with_config(TEST_TOKEN, TEST_PHONE_ID, "v21.0", mock_server.uri())
}

/// Standard success response
pub fn success_response() -> serde_json::Value {
    serde_json::json!({
        "success": true
    })
}

/// Standard message response
pub fn message_response(message_id: &str) -> serde_json::Value {
    serde_json::json!({
        "messaging_product": "whatsapp",
        "contacts": [
            {
                "input": "628123456789",
                "wa_id": "628123456789"
            }
        ],
        "messages": [
            {
                "id": message_id
            }
        ]
    })
}
