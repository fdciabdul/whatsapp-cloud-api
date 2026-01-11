//! Templates API for managing message templates

use crate::client::Client;
use crate::error::Result;
use crate::types::{Paging, SuccessResponse};
use serde::{Deserialize, Serialize};

/// Templates API client
pub struct TemplatesApi {
    client: Client,
}

impl TemplatesApi {
    pub(crate) fn new(client: Client) -> Self {
        Self { client }
    }

    /// Get all message templates for a WhatsApp Business Account
    ///
    /// # Arguments
    ///
    /// * `waba_id` - WhatsApp Business Account ID
    pub async fn list(&self, waba_id: &str) -> Result<TemplatesResponse> {
        let url = self.client.endpoint_url(&format!("{}/message_templates", waba_id));
        self.client.get(&url).await
    }

    /// Get templates with specific status
    pub async fn list_by_status(
        &self,
        waba_id: &str,
        status: TemplateStatus,
    ) -> Result<TemplatesResponse> {
        let url = self.client.endpoint_url(&format!(
            "{}/message_templates?status={}",
            waba_id,
            status.as_str()
        ));
        self.client.get(&url).await
    }

    /// Get a specific template by name
    pub async fn get_by_name(&self, waba_id: &str, name: &str) -> Result<TemplatesResponse> {
        let url = self.client.endpoint_url(&format!(
            "{}/message_templates?name={}",
            waba_id, name
        ));
        self.client.get(&url).await
    }

    /// Create a new message template
    pub async fn create(
        &self,
        waba_id: &str,
        template: &CreateTemplate,
    ) -> Result<CreateTemplateResponse> {
        let url = self.client.endpoint_url(&format!("{}/message_templates", waba_id));
        self.client.post(&url, template).await
    }

    /// Delete a message template
    ///
    /// # Arguments
    ///
    /// * `waba_id` - WhatsApp Business Account ID
    /// * `template_name` - Name of the template to delete
    pub async fn delete(&self, waba_id: &str, template_name: &str) -> Result<SuccessResponse> {
        let url = self.client.endpoint_url(&format!(
            "{}/message_templates?name={}",
            waba_id, template_name
        ));
        self.client.delete(&url).await
    }
}

/// Templates list response
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TemplatesResponse {
    /// List of templates
    pub data: Vec<MessageTemplate>,
    /// Paging info
    #[serde(default)]
    pub paging: Option<Paging>,
}

/// Message template
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MessageTemplate {
    /// Template name
    pub name: String,
    /// Template components
    pub components: Vec<TemplateComponentDef>,
    /// Template language
    pub language: String,
    /// Template status
    pub status: String,
    /// Template category
    pub category: String,
    /// Template ID
    #[serde(default)]
    pub id: Option<String>,
}

/// Template component definition
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TemplateComponentDef {
    /// Component type (HEADER, BODY, FOOTER, BUTTONS)
    #[serde(rename = "type")]
    pub component_type: String,
    /// Component format (for HEADER: TEXT, IMAGE, VIDEO, DOCUMENT)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub format: Option<String>,
    /// Text content
    #[serde(skip_serializing_if = "Option::is_none")]
    pub text: Option<String>,
    /// Buttons (for BUTTONS type)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub buttons: Option<Vec<TemplateButton>>,
    /// Example values for the template
    #[serde(skip_serializing_if = "Option::is_none")]
    pub example: Option<TemplateExample>,
}

/// Template button definition
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TemplateButton {
    /// Button type (QUICK_REPLY, URL, PHONE_NUMBER)
    #[serde(rename = "type")]
    pub button_type: String,
    /// Button text
    pub text: String,
    /// URL (for URL type)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub url: Option<String>,
    /// Phone number (for PHONE_NUMBER type)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub phone_number: Option<String>,
}

/// Template example values
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TemplateExample {
    /// Header handle (for media headers)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub header_handle: Option<Vec<String>>,
    /// Header text example
    #[serde(skip_serializing_if = "Option::is_none")]
    pub header_text: Option<Vec<String>>,
    /// Body text examples
    #[serde(skip_serializing_if = "Option::is_none")]
    pub body_text: Option<Vec<Vec<String>>>,
}

/// Create template request
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateTemplate {
    /// Template name (lowercase, underscores only)
    pub name: String,
    /// Template category (MARKETING, UTILITY, AUTHENTICATION)
    pub category: String,
    /// Language code (e.g., "en_US")
    pub language: String,
    /// Template components
    pub components: Vec<TemplateComponentDef>,
    /// Allow category change (optional)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub allow_category_change: Option<bool>,
}

impl CreateTemplate {
    /// Create a new template builder
    pub fn new(name: impl Into<String>, category: TemplateCategory, language: impl Into<String>) -> Self {
        Self {
            name: name.into(),
            category: category.as_str().to_string(),
            language: language.into(),
            components: Vec::new(),
            allow_category_change: None,
        }
    }

    /// Add a header component
    pub fn with_header(mut self, format: HeaderFormat, text: Option<String>) -> Self {
        self.components.push(TemplateComponentDef {
            component_type: "HEADER".to_string(),
            format: Some(format.as_str().to_string()),
            text,
            buttons: None,
            example: None,
        });
        self
    }

    /// Add a body component
    pub fn with_body(mut self, text: impl Into<String>) -> Self {
        self.components.push(TemplateComponentDef {
            component_type: "BODY".to_string(),
            format: None,
            text: Some(text.into()),
            buttons: None,
            example: None,
        });
        self
    }

    /// Add a footer component
    pub fn with_footer(mut self, text: impl Into<String>) -> Self {
        self.components.push(TemplateComponentDef {
            component_type: "FOOTER".to_string(),
            format: None,
            text: Some(text.into()),
            buttons: None,
            example: None,
        });
        self
    }

    /// Add buttons component
    pub fn with_buttons(mut self, buttons: Vec<TemplateButton>) -> Self {
        self.components.push(TemplateComponentDef {
            component_type: "BUTTONS".to_string(),
            format: None,
            text: None,
            buttons: Some(buttons),
            example: None,
        });
        self
    }
}

/// Create template response
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateTemplateResponse {
    /// Template ID
    pub id: String,
    /// Template status
    pub status: String,
    /// Template category
    pub category: String,
}

/// Template status
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TemplateStatus {
    /// Template is approved
    Approved,
    /// Template is pending review
    Pending,
    /// Template was rejected
    Rejected,
    /// Template is paused
    Paused,
    /// Template is disabled
    Disabled,
}

impl TemplateStatus {
    /// Get the string representation
    pub fn as_str(&self) -> &str {
        match self {
            TemplateStatus::Approved => "APPROVED",
            TemplateStatus::Pending => "PENDING",
            TemplateStatus::Rejected => "REJECTED",
            TemplateStatus::Paused => "PAUSED",
            TemplateStatus::Disabled => "DISABLED",
        }
    }
}

/// Template category
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TemplateCategory {
    /// Marketing messages
    Marketing,
    /// Utility/transactional messages
    Utility,
    /// Authentication messages (OTP)
    Authentication,
}

impl TemplateCategory {
    /// Get the string representation
    pub fn as_str(&self) -> &str {
        match self {
            TemplateCategory::Marketing => "MARKETING",
            TemplateCategory::Utility => "UTILITY",
            TemplateCategory::Authentication => "AUTHENTICATION",
        }
    }
}

/// Header format
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum HeaderFormat {
    /// Text header
    Text,
    /// Image header
    Image,
    /// Video header
    Video,
    /// Document header
    Document,
    /// Location header
    Location,
}

impl HeaderFormat {
    /// Get the string representation
    pub fn as_str(&self) -> &str {
        match self {
            HeaderFormat::Text => "TEXT",
            HeaderFormat::Image => "IMAGE",
            HeaderFormat::Video => "VIDEO",
            HeaderFormat::Document => "DOCUMENT",
            HeaderFormat::Location => "LOCATION",
        }
    }
}
