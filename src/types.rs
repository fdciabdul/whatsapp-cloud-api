//! Common types used across the WhatsApp Cloud API SDK

use serde::{Deserialize, Serialize};

/// API version for the Graph API
pub const DEFAULT_API_VERSION: &str = "v21.0";

/// Base URL for the Graph API
pub const GRAPH_API_URL: &str = "https://graph.facebook.com";

/// Message response from the API
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MessageResponse {
    /// Messaging product (always "whatsapp")
    pub messaging_product: String,
    /// Contact information
    pub contacts: Vec<ContactInfo>,
    /// Message IDs
    pub messages: Vec<MessageInfo>,
}

/// Contact info in message response
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ContactInfo {
    /// Input phone number
    pub input: String,
    /// WhatsApp ID
    pub wa_id: String,
}

/// Message info in response
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MessageInfo {
    /// Message ID
    pub id: String,
    /// Message status (optional, only in some responses)
    pub message_status: Option<String>,
}

/// Success response
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SuccessResponse {
    /// Success flag
    pub success: bool,
}

/// Phone number info
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PhoneNumber {
    /// Verified business name
    pub verified_name: String,
    /// Display phone number
    pub display_phone_number: String,
    /// Phone number ID
    pub id: String,
    /// Quality rating (GREEN, YELLOW, RED, NA)
    pub quality_rating: String,
    /// Code verification status
    #[serde(default)]
    pub code_verification_status: Option<String>,
    /// Platform type
    #[serde(default)]
    pub platform_type: Option<String>,
    /// Throughput level
    #[serde(default)]
    pub throughput: Option<Throughput>,
}

/// Throughput info
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Throughput {
    /// Throughput level
    pub level: String,
}

/// Phone numbers list response
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PhoneNumbersResponse {
    /// List of phone numbers
    pub data: Vec<PhoneNumber>,
    /// Paging info
    #[serde(default)]
    pub paging: Option<Paging>,
}

/// Paging info
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Paging {
    /// Cursors
    pub cursors: Option<Cursors>,
    /// Next page URL
    pub next: Option<String>,
    /// Previous page URL
    pub previous: Option<String>,
}

/// Cursors for pagination
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Cursors {
    /// Before cursor
    pub before: String,
    /// After cursor
    pub after: String,
}

/// WhatsApp Business Account (WABA)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WhatsAppBusinessAccount {
    /// WABA ID
    pub id: String,
    /// Account name
    pub name: String,
    /// Timezone ID
    pub timezone_id: String,
    /// Message template namespace
    pub message_template_namespace: String,
}

/// WABA list response
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WabaListResponse {
    /// List of WABAs
    pub data: Vec<WhatsAppBusinessAccount>,
    /// Paging info
    #[serde(default)]
    pub paging: Option<Paging>,
}

/// Quality rating enum
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "UPPERCASE")]
pub enum QualityRating {
    /// High quality
    Green,
    /// Medium quality
    Yellow,
    /// Low quality
    Red,
    /// Not available
    #[serde(rename = "NA")]
    Na,
}

impl std::fmt::Display for QualityRating {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            QualityRating::Green => write!(f, "GREEN"),
            QualityRating::Yellow => write!(f, "YELLOW"),
            QualityRating::Red => write!(f, "RED"),
            QualityRating::Na => write!(f, "NA"),
        }
    }
}
