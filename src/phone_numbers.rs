//! Phone Numbers API for managing business phone numbers

use crate::client::Client;
use crate::error::Result;
use crate::types::{PhoneNumber, PhoneNumbersResponse, SuccessResponse};
use serde::{Deserialize, Serialize};

/// Phone Numbers API client
pub struct PhoneNumbersApi {
    client: Client,
}

impl PhoneNumbersApi {
    pub(crate) fn new(client: Client) -> Self {
        Self { client }
    }

    /// Get all phone numbers for a WhatsApp Business Account
    ///
    /// # Arguments
    ///
    /// * `waba_id` - WhatsApp Business Account ID
    pub async fn list(&self, waba_id: &str) -> Result<PhoneNumbersResponse> {
        let url = self
            .client
            .endpoint_url(&format!("{}/phone_numbers", waba_id));
        self.client.get(&url).await
    }

    /// Get a specific phone number by ID
    pub async fn get(&self, phone_number_id: &str) -> Result<PhoneNumber> {
        let url = self.client.endpoint_url(phone_number_id);
        self.client.get(&url).await
    }

    /// Register a phone number
    ///
    /// # Arguments
    ///
    /// * `pin` - 6-digit PIN for two-step verification
    pub async fn register(&self, pin: &str) -> Result<SuccessResponse> {
        let body = RegisterRequest {
            messaging_product: "whatsapp".to_string(),
            pin: pin.to_string(),
        };

        let url = format!("{}/register", self.client.base_url());
        self.client.post(&url, &body).await
    }

    /// Deregister a phone number
    pub async fn deregister(&self) -> Result<SuccessResponse> {
        let url = format!("{}/deregister", self.client.base_url());
        self.client.post(&url, &serde_json::json!({})).await
    }

    /// Request a verification code
    ///
    /// # Arguments
    ///
    /// * `code_method` - Method to receive code: "SMS" or "VOICE"
    /// * `language` - Language code (e.g., "en_US")
    pub async fn request_verification_code(
        &self,
        code_method: &str,
        language: &str,
    ) -> Result<SuccessResponse> {
        let body = RequestCodeRequest {
            code_method: code_method.to_string(),
            language: language.to_string(),
        };

        let url = format!("{}/request_code", self.client.base_url());
        self.client.post(&url, &body).await
    }

    /// Verify a phone number with the code received
    ///
    /// # Arguments
    ///
    /// * `code` - Verification code received via SMS or voice
    pub async fn verify_code(&self, code: &str) -> Result<SuccessResponse> {
        let body = VerifyCodeRequest {
            code: code.to_string(),
        };

        let url = format!("{}/verify_code", self.client.base_url());
        self.client.post(&url, &body).await
    }

    /// Set two-step verification PIN
    ///
    /// # Arguments
    ///
    /// * `pin` - 6-digit PIN
    pub async fn set_two_step_verification(&self, pin: &str) -> Result<SuccessResponse> {
        let body = TwoStepRequest {
            pin: pin.to_string(),
        };

        let url = self.client.base_url();
        self.client.post(&url, &body).await
    }

    /// Get business profile
    pub async fn get_business_profile(&self) -> Result<BusinessProfileResponse> {
        let url = format!(
            "{}/whatsapp_business_profile?fields=about,address,description,email,profile_picture_url,websites,vertical",
            self.client.base_url()
        );
        self.client.get(&url).await
    }

    /// Update business profile
    pub async fn update_business_profile(
        &self,
        profile: &BusinessProfileUpdate,
    ) -> Result<SuccessResponse> {
        let url = format!("{}/whatsapp_business_profile", self.client.base_url());
        self.client.post(&url, profile).await
    }
}

#[derive(Debug, Serialize)]
struct RegisterRequest {
    messaging_product: String,
    pin: String,
}

#[derive(Debug, Serialize)]
struct RequestCodeRequest {
    code_method: String,
    language: String,
}

#[derive(Debug, Serialize)]
struct VerifyCodeRequest {
    code: String,
}

#[derive(Debug, Serialize)]
struct TwoStepRequest {
    pin: String,
}

/// Business profile response
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BusinessProfileResponse {
    /// Profile data
    pub data: Vec<BusinessProfile>,
}

/// Business profile
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BusinessProfile {
    /// Messaging product
    #[serde(default)]
    pub messaging_product: Option<String>,
    /// About text
    #[serde(default)]
    pub about: Option<String>,
    /// Business address
    #[serde(default)]
    pub address: Option<String>,
    /// Business description
    #[serde(default)]
    pub description: Option<String>,
    /// Business email
    #[serde(default)]
    pub email: Option<String>,
    /// Profile picture URL
    #[serde(default)]
    pub profile_picture_url: Option<String>,
    /// Website URLs
    #[serde(default)]
    pub websites: Option<Vec<String>>,
    /// Business vertical/category
    #[serde(default)]
    pub vertical: Option<String>,
}

/// Business profile update request
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BusinessProfileUpdate {
    /// Messaging product (required)
    pub messaging_product: String,
    /// About text (max 139 characters)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub about: Option<String>,
    /// Business address (max 256 characters)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub address: Option<String>,
    /// Business description (max 512 characters)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// Business email (max 128 characters)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub email: Option<String>,
    /// Profile picture handle (from resumable upload)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub profile_picture_handle: Option<String>,
    /// Website URLs (max 2)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub websites: Option<Vec<String>>,
    /// Business vertical
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vertical: Option<String>,
}

impl Default for BusinessProfileUpdate {
    fn default() -> Self {
        Self {
            messaging_product: "whatsapp".to_string(),
            about: None,
            address: None,
            description: None,
            email: None,
            profile_picture_handle: None,
            websites: None,
            vertical: None,
        }
    }
}

impl BusinessProfileUpdate {
    /// Create a new business profile update
    pub fn new() -> Self {
        Self::default()
    }

    /// Set the about text
    pub fn about(mut self, about: impl Into<String>) -> Self {
        self.about = Some(about.into());
        self
    }

    /// Set the address
    pub fn address(mut self, address: impl Into<String>) -> Self {
        self.address = Some(address.into());
        self
    }

    /// Set the description
    pub fn description(mut self, description: impl Into<String>) -> Self {
        self.description = Some(description.into());
        self
    }

    /// Set the email
    pub fn email(mut self, email: impl Into<String>) -> Self {
        self.email = Some(email.into());
        self
    }

    /// Set the websites
    pub fn websites(mut self, websites: Vec<String>) -> Self {
        self.websites = Some(websites);
        self
    }

    /// Set the vertical
    pub fn vertical(mut self, vertical: impl Into<String>) -> Self {
        self.vertical = Some(vertical.into());
        self
    }
}
