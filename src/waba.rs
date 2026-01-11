//! WhatsApp Business Account (WABA) Management API

use crate::client::Client;
use crate::error::Result;
use serde::{Deserialize, Serialize};

/// WABA Management API client
pub struct WabaApi {
    client: Client,
    waba_id: String,
}

impl WabaApi {
    pub(crate) fn new(client: Client, waba_id: String) -> Self {
        Self { client, waba_id }
    }

    /// Get WABA details
    pub async fn get(&self) -> Result<WabaDetails> {
        let url = self.client.endpoint_url(&self.waba_id);
        self.client.get(&url).await
    }

    /// Subscribe to webhooks for this WABA
    pub async fn subscribe_webhooks(&self) -> Result<crate::types::SuccessResponse> {
        let body = SubscribeRequest {
            subscribed_fields: vec![
                "messages".to_string(),
                "message_template_status_update".to_string(),
            ],
        };

        let url = format!(
            "{}/subscribed_apps",
            self.client.endpoint_url(&self.waba_id)
        );
        self.client.post(&url, &body).await
    }

    /// Subscribe to specific webhook fields
    ///
    /// # Arguments
    ///
    /// * `fields` - List of fields to subscribe to
    pub async fn subscribe_fields(
        &self,
        fields: Vec<WebhookField>,
    ) -> Result<crate::types::SuccessResponse> {
        let body = SubscribeRequest {
            subscribed_fields: fields.iter().map(|f| f.as_str().to_string()).collect(),
        };

        let url = format!(
            "{}/subscribed_apps",
            self.client.endpoint_url(&self.waba_id)
        );
        self.client.post(&url, &body).await
    }

    /// Unsubscribe from webhooks for this WABA
    pub async fn unsubscribe_webhooks(&self) -> Result<crate::types::SuccessResponse> {
        let url = format!(
            "{}/subscribed_apps",
            self.client.endpoint_url(&self.waba_id)
        );
        self.client.delete(&url).await
    }

    /// Get list of subscribed apps
    pub async fn get_subscribed_apps(&self) -> Result<SubscribedAppsResponse> {
        let url = format!(
            "{}/subscribed_apps",
            self.client.endpoint_url(&self.waba_id)
        );
        self.client.get(&url).await
    }

    /// Get phone numbers for this WABA
    pub async fn get_phone_numbers(&self) -> Result<PhoneNumbersResponse> {
        let url = format!(
            "{}/phone_numbers",
            self.client.endpoint_url(&self.waba_id)
        );
        self.client.get(&url).await
    }

    /// Get assigned users for this WABA
    pub async fn get_assigned_users(&self) -> Result<AssignedUsersResponse> {
        let url = format!(
            "{}/assigned_users",
            self.client.endpoint_url(&self.waba_id)
        );
        self.client.get(&url).await
    }

    /// Get system users for this WABA
    pub async fn get_system_users(&self) -> Result<SystemUsersResponse> {
        let url = format!(
            "{}/system_users",
            self.client.endpoint_url(&self.waba_id)
        );
        self.client.get(&url).await
    }

    /// Get message templates for this WABA
    pub async fn get_templates(&self) -> Result<WabaTemplatesResponse> {
        let url = format!(
            "{}/message_templates",
            self.client.endpoint_url(&self.waba_id)
        );
        self.client.get(&url).await
    }
}

/// Webhook fields that can be subscribed
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum WebhookField {
    /// Message events
    Messages,
    /// Template status updates
    MessageTemplateStatusUpdate,
    /// Template quality updates
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
}

impl WebhookField {
    fn as_str(&self) -> &'static str {
        match self {
            WebhookField::Messages => "messages",
            WebhookField::MessageTemplateStatusUpdate => "message_template_status_update",
            WebhookField::MessageTemplateQualityUpdate => "message_template_quality_update",
            WebhookField::AccountAlerts => "account_alerts",
            WebhookField::AccountReviewUpdate => "account_review_update",
            WebhookField::AccountUpdate => "account_update",
            WebhookField::BusinessCapabilityUpdate => "business_capability_update",
            WebhookField::PhoneNumberNameUpdate => "phone_number_name_update",
            WebhookField::PhoneNumberQualityUpdate => "phone_number_quality_update",
            WebhookField::Security => "security",
        }
    }
}

// Request types

#[derive(Debug, Serialize)]
struct SubscribeRequest {
    subscribed_fields: Vec<String>,
}

// Response types

/// WABA details response
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WabaDetails {
    /// WABA ID
    pub id: String,
    /// WABA name
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// Timezone ID
    #[serde(skip_serializing_if = "Option::is_none")]
    pub timezone_id: Option<String>,
    /// Message template namespace
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message_template_namespace: Option<String>,
    /// Account review status
    #[serde(skip_serializing_if = "Option::is_none")]
    pub account_review_status: Option<String>,
    /// Business verification status
    #[serde(skip_serializing_if = "Option::is_none")]
    pub business_verification_status: Option<String>,
    /// Primary funding ID
    #[serde(skip_serializing_if = "Option::is_none")]
    pub primary_funding_id: Option<String>,
    /// Purchase order number
    #[serde(skip_serializing_if = "Option::is_none")]
    pub purchase_order_number: Option<String>,
}

/// Subscribed apps response
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SubscribedAppsResponse {
    /// List of subscribed apps
    pub data: Vec<SubscribedApp>,
}

/// Subscribed app info
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SubscribedApp {
    /// App ID
    pub id: String,
    /// App name
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// Subscribed fields
    #[serde(default)]
    pub subscribed_fields: Vec<String>,
}

/// Phone numbers response
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PhoneNumbersResponse {
    /// List of phone numbers
    pub data: Vec<WabaPhoneNumber>,
}

/// Phone number info
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WabaPhoneNumber {
    /// Phone number ID
    pub id: String,
    /// Display phone number
    pub display_phone_number: String,
    /// Verified name
    #[serde(skip_serializing_if = "Option::is_none")]
    pub verified_name: Option<String>,
    /// Quality rating
    #[serde(skip_serializing_if = "Option::is_none")]
    pub quality_rating: Option<String>,
}

/// Assigned users response
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AssignedUsersResponse {
    /// List of assigned users
    pub data: Vec<AssignedUser>,
}

/// Assigned user info
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AssignedUser {
    /// User ID
    pub id: String,
    /// User name
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// User tasks
    #[serde(default)]
    pub tasks: Vec<String>,
}

/// System users response
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SystemUsersResponse {
    /// List of system users
    pub data: Vec<SystemUser>,
}

/// System user info
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SystemUser {
    /// User ID
    pub id: String,
    /// User name
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// User role
    #[serde(skip_serializing_if = "Option::is_none")]
    pub role: Option<String>,
}

/// WABA templates response
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WabaTemplatesResponse {
    /// List of templates
    pub data: Vec<WabaTemplate>,
    /// Paging info
    #[serde(skip_serializing_if = "Option::is_none")]
    pub paging: Option<Paging>,
}

/// WABA template info
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WabaTemplate {
    /// Template ID
    pub id: String,
    /// Template name
    pub name: String,
    /// Template status
    pub status: String,
    /// Template category
    #[serde(skip_serializing_if = "Option::is_none")]
    pub category: Option<String>,
    /// Template language
    #[serde(skip_serializing_if = "Option::is_none")]
    pub language: Option<String>,
}

/// Paging info
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Paging {
    /// Cursors
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cursors: Option<PagingCursors>,
    /// Next page URL
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next: Option<String>,
}

/// Paging cursors
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PagingCursors {
    /// Before cursor
    #[serde(skip_serializing_if = "Option::is_none")]
    pub before: Option<String>,
    /// After cursor
    #[serde(skip_serializing_if = "Option::is_none")]
    pub after: Option<String>,
}
