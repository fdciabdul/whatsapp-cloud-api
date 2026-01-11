//! Analytics API for WhatsApp Business

use crate::client::Client;
use crate::error::Result;
use serde::{Deserialize, Serialize};

/// Analytics API client
pub struct AnalyticsApi {
    client: Client,
    waba_id: String,
}

impl AnalyticsApi {
    pub(crate) fn new(client: Client, waba_id: String) -> Self {
        Self { client, waba_id }
    }

    /// Get conversation analytics
    ///
    /// # Arguments
    ///
    /// * `start` - Start timestamp (Unix timestamp in seconds)
    /// * `end` - End timestamp (Unix timestamp in seconds)
    /// * `granularity` - Time granularity (HALF_HOUR, DAILY, MONTHLY)
    pub async fn get_conversation_analytics(
        &self,
        start: i64,
        end: i64,
        granularity: Granularity,
    ) -> Result<ConversationAnalyticsResponse> {
        let url = format!(
            "{}?start={}&end={}&granularity={}&fields=conversation_analytics",
            self.client.endpoint_url(&self.waba_id),
            start,
            end,
            granularity.as_str()
        );
        self.client.get(&url).await
    }

    /// Get template analytics
    ///
    /// # Arguments
    ///
    /// * `start` - Start timestamp (Unix timestamp in seconds)
    /// * `end` - End timestamp (Unix timestamp in seconds)
    /// * `granularity` - Time granularity
    /// * `template_ids` - Optional list of template IDs to filter
    pub async fn get_template_analytics(
        &self,
        start: i64,
        end: i64,
        granularity: Granularity,
        template_ids: Option<Vec<&str>>,
    ) -> Result<TemplateAnalyticsResponse> {
        let mut url = format!(
            "{}?start={}&end={}&granularity={}&fields=template_analytics",
            self.client.endpoint_url(&self.waba_id),
            start,
            end,
            granularity.as_str()
        );

        if let Some(ids) = template_ids {
            let ids_param = ids.join(",");
            url.push_str(&format!("&template_ids={}", ids_param));
        }

        self.client.get(&url).await
    }

    /// Get phone number analytics
    ///
    /// # Arguments
    ///
    /// * `start` - Start timestamp (Unix timestamp in seconds)
    /// * `end` - End timestamp (Unix timestamp in seconds)
    /// * `granularity` - Time granularity
    /// * `phone_numbers` - Optional list of phone numbers to filter
    pub async fn get_phone_number_analytics(
        &self,
        start: i64,
        end: i64,
        granularity: Granularity,
        phone_numbers: Option<Vec<&str>>,
    ) -> Result<PhoneNumberAnalyticsResponse> {
        let mut url = format!(
            "{}?start={}&end={}&granularity={}&fields=analytics",
            self.client.endpoint_url(&self.waba_id),
            start,
            end,
            granularity.as_str()
        );

        if let Some(numbers) = phone_numbers {
            let numbers_param = numbers.join(",");
            url.push_str(&format!("&phone_numbers={}", numbers_param));
        }

        self.client.get(&url).await
    }
}

/// Time granularity for analytics
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Granularity {
    /// 30-minute intervals
    HalfHour,
    /// Daily intervals
    Daily,
    /// Monthly intervals
    Monthly,
}

impl Granularity {
    fn as_str(&self) -> &'static str {
        match self {
            Granularity::HalfHour => "HALF_HOUR",
            Granularity::Daily => "DAILY",
            Granularity::Monthly => "MONTHLY",
        }
    }
}

// Response types

/// Conversation analytics response
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConversationAnalyticsResponse {
    /// Analytics data
    pub conversation_analytics: ConversationAnalytics,
}

/// Conversation analytics
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConversationAnalytics {
    /// Data points
    pub data: Vec<ConversationDataPoint>,
}

/// Conversation data point
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConversationDataPoint {
    /// Data point start timestamp
    pub start: i64,
    /// Data point end timestamp
    pub end: i64,
    /// Conversation count
    pub conversation: i64,
    /// Cost in credits
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cost: Option<f64>,
    /// Phone number ID
    #[serde(skip_serializing_if = "Option::is_none")]
    pub phone_number: Option<String>,
    /// Conversation direction
    #[serde(skip_serializing_if = "Option::is_none")]
    pub conversation_direction: Option<String>,
    /// Conversation type
    #[serde(skip_serializing_if = "Option::is_none")]
    pub conversation_type: Option<String>,
    /// Conversation category
    #[serde(skip_serializing_if = "Option::is_none")]
    pub conversation_category: Option<String>,
}

/// Template analytics response
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TemplateAnalyticsResponse {
    /// Analytics data
    pub template_analytics: TemplateAnalytics,
}

/// Template analytics
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TemplateAnalytics {
    /// Data points
    pub data: Vec<TemplateDataPoint>,
}

/// Template data point
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TemplateDataPoint {
    /// Data point start timestamp
    pub start: i64,
    /// Data point end timestamp
    pub end: i64,
    /// Template ID
    pub template_id: String,
    /// Sent count
    #[serde(default)]
    pub sent: i64,
    /// Delivered count
    #[serde(default)]
    pub delivered: i64,
    /// Read count
    #[serde(default)]
    pub read: i64,
    /// Clicked count
    #[serde(default)]
    pub clicked: i64,
}

/// Phone number analytics response
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PhoneNumberAnalyticsResponse {
    /// Analytics data
    pub analytics: PhoneNumberAnalytics,
}

/// Phone number analytics
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PhoneNumberAnalytics {
    /// Phone number
    pub phone_number: String,
    /// Data points
    pub data: Vec<PhoneNumberDataPoint>,
}

/// Phone number data point
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PhoneNumberDataPoint {
    /// Data point start timestamp
    pub start: i64,
    /// Data point end timestamp
    pub end: i64,
    /// Sent count
    #[serde(default)]
    pub sent: i64,
    /// Delivered count
    #[serde(default)]
    pub delivered: i64,
}
