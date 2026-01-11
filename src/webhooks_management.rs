//! Webhook Subscriptions Management API

use crate::client::Client;
use crate::error::Result;
use serde::{Deserialize, Serialize};

/// Webhook Subscriptions API client
pub struct WebhookSubscriptionsApi {
    client: Client,
    app_id: String,
}

impl WebhookSubscriptionsApi {
    pub(crate) fn new(client: Client, app_id: String) -> Self {
        Self { client, app_id }
    }

    /// Get current webhook subscriptions
    pub async fn get(&self) -> Result<WebhookSubscriptionsResponse> {
        let url = self
            .client
            .endpoint_url(&format!("{}/subscriptions", self.app_id));
        self.client.get(&url).await
    }

    /// Create or update webhook subscription
    ///
    /// # Arguments
    ///
    /// * `callback_url` - The webhook URL to receive events
    /// * `verify_token` - The verify token for webhook verification
    /// * `fields` - List of fields to subscribe to
    pub async fn subscribe(
        &self,
        callback_url: &str,
        verify_token: &str,
        fields: Vec<SubscriptionField>,
    ) -> Result<crate::types::SuccessResponse> {
        let body = SubscribeWebhookRequest {
            object: "whatsapp_business_account".to_string(),
            callback_url: callback_url.to_string(),
            verify_token: verify_token.to_string(),
            fields: fields.iter().map(|f| f.as_str().to_string()).collect(),
        };

        let url = self
            .client
            .endpoint_url(&format!("{}/subscriptions", self.app_id));
        self.client.post(&url, &body).await
    }

    /// Delete webhook subscription
    ///
    /// # Arguments
    ///
    /// * `object` - Object type to unsubscribe (e.g., "whatsapp_business_account")
    pub async fn unsubscribe(&self, object: &str) -> Result<crate::types::SuccessResponse> {
        let url = self
            .client
            .endpoint_url(&format!("{}/subscriptions?object={}", self.app_id, object));
        self.client.delete(&url).await
    }

    /// Delete all webhook subscriptions
    pub async fn unsubscribe_all(&self) -> Result<crate::types::SuccessResponse> {
        let url = self
            .client
            .endpoint_url(&format!("{}/subscriptions", self.app_id));
        self.client.delete(&url).await
    }
}

/// Subscription fields for webhooks
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SubscriptionField {
    /// Message events
    Messages,
    /// Message template status updates
    MessageTemplateStatusUpdate,
    /// Message template quality updates
    MessageTemplateQualityUpdate,
    /// Account alerts
    AccountAlerts,
    /// Account review updates
    AccountReviewUpdate,
    /// Account updates
    AccountUpdate,
    /// Business capability updates
    BusinessCapabilityUpdate,
    /// Phone number name updates
    PhoneNumberNameUpdate,
    /// Phone number quality updates
    PhoneNumberQualityUpdate,
    /// Security events
    Security,
    /// Flows events
    Flows,
}

impl SubscriptionField {
    fn as_str(&self) -> &'static str {
        match self {
            SubscriptionField::Messages => "messages",
            SubscriptionField::MessageTemplateStatusUpdate => "message_template_status_update",
            SubscriptionField::MessageTemplateQualityUpdate => "message_template_quality_update",
            SubscriptionField::AccountAlerts => "account_alerts",
            SubscriptionField::AccountReviewUpdate => "account_review_update",
            SubscriptionField::AccountUpdate => "account_update",
            SubscriptionField::BusinessCapabilityUpdate => "business_capability_update",
            SubscriptionField::PhoneNumberNameUpdate => "phone_number_name_update",
            SubscriptionField::PhoneNumberQualityUpdate => "phone_number_quality_update",
            SubscriptionField::Security => "security",
            SubscriptionField::Flows => "flows",
        }
    }
}

// Request types

#[derive(Debug, Serialize)]
struct SubscribeWebhookRequest {
    object: String,
    callback_url: String,
    verify_token: String,
    fields: Vec<String>,
}

// Response types

/// Webhook subscriptions response
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WebhookSubscriptionsResponse {
    /// List of subscriptions
    pub data: Vec<WebhookSubscription>,
}

/// Webhook subscription info
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WebhookSubscription {
    /// Object type (e.g., "whatsapp_business_account")
    pub object: String,
    /// Callback URL
    pub callback_url: String,
    /// Active status
    pub active: bool,
    /// Subscribed fields
    #[serde(default)]
    pub fields: Vec<SubscribedField>,
}

/// Subscribed field info
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SubscribedField {
    /// Field name
    pub name: String,
    /// API version
    #[serde(skip_serializing_if = "Option::is_none")]
    pub version: Option<String>,
}
