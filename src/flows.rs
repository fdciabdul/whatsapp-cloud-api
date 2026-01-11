//! Flows API for WhatsApp Flows

use crate::client::Client;
use crate::error::Result;
use crate::types::MessageResponse;
use serde::{Deserialize, Serialize};
use serde_json::Value;

/// Flows API client
pub struct FlowsApi {
    client: Client,
}

impl FlowsApi {
    pub(crate) fn new(client: Client) -> Self {
        Self { client }
    }

    /// Send a flow message
    ///
    /// # Arguments
    ///
    /// * `to` - Recipient's phone number
    /// * `flow_token` - Flow token for the session
    /// * `flow_id` - The flow ID
    /// * `flow_cta` - Call to action button text
    /// * `flow_action` - Flow action (navigate or data_exchange)
    /// * `screen` - Initial screen name
    /// * `data` - Optional flow data
    /// * `header` - Optional header text
    /// * `body_text` - Body text
    /// * `footer` - Optional footer text
    pub async fn send_flow(
        &self,
        to: &str,
        flow_token: &str,
        flow_id: &str,
        flow_cta: &str,
        flow_action: FlowAction,
        screen: &str,
        data: Option<Value>,
        header: Option<&str>,
        body_text: &str,
        footer: Option<&str>,
    ) -> Result<MessageResponse> {
        let body = SendFlowRequest {
            messaging_product: "whatsapp".to_string(),
            recipient_type: "individual".to_string(),
            to: to.to_string(),
            message_type: "interactive".to_string(),
            interactive: FlowInteractive {
                interactive_type: "flow".to_string(),
                header: header.map(|h| FlowHeader {
                    header_type: "text".to_string(),
                    text: h.to_string(),
                }),
                body: FlowBody {
                    text: body_text.to_string(),
                },
                footer: footer.map(|f| FlowFooter {
                    text: f.to_string(),
                }),
                action: FlowActionPayload {
                    name: "flow".to_string(),
                    parameters: FlowParameters {
                        flow_message_version: "3".to_string(),
                        flow_token: flow_token.to_string(),
                        flow_id: flow_id.to_string(),
                        flow_cta: flow_cta.to_string(),
                        flow_action: flow_action.as_str().to_string(),
                        flow_action_payload: FlowActionPayloadData {
                            screen: screen.to_string(),
                            data,
                        },
                    },
                },
            },
        };

        let url = format!("{}/messages", self.client.base_url());
        self.client.post(&url, &body).await
    }

    /// List flows for the WABA
    ///
    /// # Arguments
    ///
    /// * `waba_id` - WhatsApp Business Account ID
    pub async fn list_flows(&self, waba_id: &str) -> Result<FlowsListResponse> {
        let url = self.client.endpoint_url(&format!("{}/flows", waba_id));
        self.client.get(&url).await
    }

    /// Get flow details
    ///
    /// # Arguments
    ///
    /// * `flow_id` - The flow ID
    pub async fn get_flow(&self, flow_id: &str) -> Result<Flow> {
        let url = self.client.endpoint_url(flow_id);
        self.client.get(&url).await
    }

    /// Create a new flow
    ///
    /// # Arguments
    ///
    /// * `waba_id` - WhatsApp Business Account ID
    /// * `name` - Flow name
    /// * `categories` - Flow categories
    pub async fn create_flow(
        &self,
        waba_id: &str,
        name: &str,
        categories: Vec<FlowCategory>,
    ) -> Result<CreateFlowResponse> {
        let body = CreateFlowRequest {
            name: name.to_string(),
            categories: categories.iter().map(|c| c.as_str().to_string()).collect(),
        };

        let url = self.client.endpoint_url(&format!("{}/flows", waba_id));
        self.client.post(&url, &body).await
    }

    /// Update flow JSON
    ///
    /// # Arguments
    ///
    /// * `flow_id` - The flow ID
    /// * `flow_json` - The flow JSON content
    pub async fn update_flow_json(
        &self,
        flow_id: &str,
        flow_json: &str,
    ) -> Result<UpdateFlowResponse> {
        let form = reqwest::multipart::Form::new()
            .text("name", "flow.json")
            .text("file", flow_json.to_string());

        let url = self.client.endpoint_url(&format!("{}/assets", flow_id));
        self.client.post_form(&url, form).await
    }

    /// Publish a flow
    ///
    /// # Arguments
    ///
    /// * `flow_id` - The flow ID
    pub async fn publish_flow(&self, flow_id: &str) -> Result<crate::types::SuccessResponse> {
        let body = PublishFlowRequest {
            status: "PUBLISHED".to_string(),
        };

        let url = self.client.endpoint_url(flow_id);
        self.client.post(&url, &body).await
    }

    /// Delete a flow
    ///
    /// # Arguments
    ///
    /// * `flow_id` - The flow ID
    pub async fn delete_flow(&self, flow_id: &str) -> Result<crate::types::SuccessResponse> {
        let url = self.client.endpoint_url(flow_id);
        self.client.delete(&url).await
    }

    /// Deprecate a flow
    ///
    /// # Arguments
    ///
    /// * `flow_id` - The flow ID
    pub async fn deprecate_flow(&self, flow_id: &str) -> Result<crate::types::SuccessResponse> {
        let body = DeprecateFlowRequest {
            status: "DEPRECATED".to_string(),
        };

        let url = self.client.endpoint_url(flow_id);
        self.client.post(&url, &body).await
    }

    /// Get flow preview URL
    ///
    /// # Arguments
    ///
    /// * `flow_id` - The flow ID
    pub async fn get_preview(&self, flow_id: &str) -> Result<FlowPreviewResponse> {
        let url = self.client.endpoint_url(&format!("{}/preview", flow_id));
        self.client.get(&url).await
    }
}

/// Flow action type
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum FlowAction {
    /// Navigate to a screen
    Navigate,
    /// Exchange data
    DataExchange,
}

impl FlowAction {
    fn as_str(&self) -> &'static str {
        match self {
            FlowAction::Navigate => "navigate",
            FlowAction::DataExchange => "data_exchange",
        }
    }
}

/// Flow category
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum FlowCategory {
    SignUp,
    SignIn,
    Appointment,
    LeadGeneration,
    ContactUs,
    CustomerSupport,
    Survey,
    Other,
}

impl FlowCategory {
    fn as_str(&self) -> &'static str {
        match self {
            FlowCategory::SignUp => "SIGN_UP",
            FlowCategory::SignIn => "SIGN_IN",
            FlowCategory::Appointment => "APPOINTMENT_BOOKING",
            FlowCategory::LeadGeneration => "LEAD_GENERATION",
            FlowCategory::ContactUs => "CONTACT_US",
            FlowCategory::CustomerSupport => "CUSTOMER_SUPPORT",
            FlowCategory::Survey => "SURVEY",
            FlowCategory::Other => "OTHER",
        }
    }
}

// Request types

#[derive(Debug, Serialize)]
struct SendFlowRequest {
    messaging_product: String,
    recipient_type: String,
    to: String,
    #[serde(rename = "type")]
    message_type: String,
    interactive: FlowInteractive,
}

#[derive(Debug, Serialize)]
struct FlowInteractive {
    #[serde(rename = "type")]
    interactive_type: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    header: Option<FlowHeader>,
    body: FlowBody,
    #[serde(skip_serializing_if = "Option::is_none")]
    footer: Option<FlowFooter>,
    action: FlowActionPayload,
}

#[derive(Debug, Serialize)]
struct FlowHeader {
    #[serde(rename = "type")]
    header_type: String,
    text: String,
}

#[derive(Debug, Serialize)]
struct FlowBody {
    text: String,
}

#[derive(Debug, Serialize)]
struct FlowFooter {
    text: String,
}

#[derive(Debug, Serialize)]
struct FlowActionPayload {
    name: String,
    parameters: FlowParameters,
}

#[derive(Debug, Serialize)]
struct FlowParameters {
    flow_message_version: String,
    flow_token: String,
    flow_id: String,
    flow_cta: String,
    flow_action: String,
    flow_action_payload: FlowActionPayloadData,
}

#[derive(Debug, Serialize)]
struct FlowActionPayloadData {
    screen: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    data: Option<Value>,
}

#[derive(Debug, Serialize)]
struct CreateFlowRequest {
    name: String,
    categories: Vec<String>,
}

#[derive(Debug, Serialize)]
struct PublishFlowRequest {
    status: String,
}

#[derive(Debug, Serialize)]
struct DeprecateFlowRequest {
    status: String,
}

// Response types

/// Flow list response
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FlowsListResponse {
    /// List of flows
    pub data: Vec<Flow>,
    /// Paging info
    #[serde(skip_serializing_if = "Option::is_none")]
    pub paging: Option<Paging>,
}

/// Flow details
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Flow {
    /// Flow ID
    pub id: String,
    /// Flow name
    pub name: String,
    /// Flow status
    pub status: String,
    /// Flow categories
    #[serde(default)]
    pub categories: Vec<String>,
    /// Validation errors
    #[serde(skip_serializing_if = "Option::is_none")]
    pub validation_errors: Option<Vec<FlowValidationError>>,
    /// JSON version
    #[serde(skip_serializing_if = "Option::is_none")]
    pub json_version: Option<String>,
    /// Data API version
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data_api_version: Option<String>,
    /// Endpoint URI
    #[serde(skip_serializing_if = "Option::is_none")]
    pub endpoint_uri: Option<String>,
    /// Preview info
    #[serde(skip_serializing_if = "Option::is_none")]
    pub preview: Option<FlowPreview>,
}

/// Flow validation error
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FlowValidationError {
    /// Error message
    pub error: String,
    /// Error type
    pub error_type: String,
    /// Line start
    #[serde(skip_serializing_if = "Option::is_none")]
    pub line_start: Option<i32>,
    /// Line end
    #[serde(skip_serializing_if = "Option::is_none")]
    pub line_end: Option<i32>,
    /// Column start
    #[serde(skip_serializing_if = "Option::is_none")]
    pub column_start: Option<i32>,
    /// Column end
    #[serde(skip_serializing_if = "Option::is_none")]
    pub column_end: Option<i32>,
}

/// Flow preview
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FlowPreview {
    /// Preview URL
    pub preview_url: String,
    /// Expires at
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expires_at: Option<String>,
}

/// Create flow response
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateFlowResponse {
    /// Created flow ID
    pub id: String,
}

/// Update flow response
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UpdateFlowResponse {
    /// Success status
    pub success: bool,
    /// Validation errors
    #[serde(skip_serializing_if = "Option::is_none")]
    pub validation_errors: Option<Vec<FlowValidationError>>,
}

/// Flow preview response
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FlowPreviewResponse {
    /// Preview URL
    pub preview_url: String,
    /// Expires at
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expires_at: Option<String>,
}

/// Paging info
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Paging {
    /// Cursors
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cursors: Option<PagingCursors>,
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
