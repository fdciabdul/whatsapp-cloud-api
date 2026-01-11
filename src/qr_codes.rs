//! QR Codes API for WhatsApp Business

use crate::client::Client;
use crate::error::Result;
use serde::{Deserialize, Serialize};

/// QR Codes API client
pub struct QrCodesApi {
    client: Client,
}

impl QrCodesApi {
    pub(crate) fn new(client: Client) -> Self {
        Self { client }
    }

    /// Create a QR code
    ///
    /// # Arguments
    ///
    /// * `prefilled_message` - The message that will be pre-filled when user scans
    /// * `generate_qr_image` - Image format to generate (PNG or SVG)
    pub async fn create(
        &self,
        prefilled_message: &str,
        generate_qr_image: QrImageFormat,
    ) -> Result<QrCodeResponse> {
        let body = CreateQrCodeRequest {
            prefilled_message: prefilled_message.to_string(),
            generate_qr_image: generate_qr_image.as_str().to_string(),
        };

        let url = format!("{}/message_qrdls", self.client.base_url());
        self.client.post(&url, &body).await
    }

    /// List all QR codes
    pub async fn list(&self) -> Result<QrCodesListResponse> {
        let url = format!("{}/message_qrdls", self.client.base_url());
        self.client.get(&url).await
    }

    /// Get a specific QR code
    ///
    /// # Arguments
    ///
    /// * `qr_code_id` - The QR code ID
    pub async fn get(&self, qr_code_id: &str) -> Result<QrCode> {
        let url = format!("{}/message_qrdls/{}", self.client.base_url(), qr_code_id);
        self.client.get(&url).await
    }

    /// Update a QR code
    ///
    /// # Arguments
    ///
    /// * `qr_code_id` - The QR code ID
    /// * `prefilled_message` - The new pre-filled message
    pub async fn update(
        &self,
        qr_code_id: &str,
        prefilled_message: &str,
    ) -> Result<QrCodeResponse> {
        let body = UpdateQrCodeRequest {
            prefilled_message: prefilled_message.to_string(),
        };

        let url = format!("{}/message_qrdls/{}", self.client.base_url(), qr_code_id);
        self.client.post(&url, &body).await
    }

    /// Delete a QR code
    ///
    /// # Arguments
    ///
    /// * `qr_code_id` - The QR code ID
    pub async fn delete(&self, qr_code_id: &str) -> Result<crate::types::SuccessResponse> {
        let url = format!("{}/message_qrdls/{}", self.client.base_url(), qr_code_id);
        self.client.delete(&url).await
    }
}

/// QR code image format
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum QrImageFormat {
    /// PNG format
    Png,
    /// SVG format
    Svg,
}

impl QrImageFormat {
    fn as_str(&self) -> &'static str {
        match self {
            QrImageFormat::Png => "PNG",
            QrImageFormat::Svg => "SVG",
        }
    }
}

// Request types

#[derive(Debug, Serialize)]
struct CreateQrCodeRequest {
    prefilled_message: String,
    generate_qr_image: String,
}

#[derive(Debug, Serialize)]
struct UpdateQrCodeRequest {
    prefilled_message: String,
}

// Response types

/// QR code response
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct QrCodeResponse {
    /// QR code ID
    pub code: String,
    /// Pre-filled message
    pub prefilled_message: String,
    /// Deep link URL
    pub deep_link_url: String,
    /// QR image URL (if requested)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub qr_image_url: Option<String>,
}

/// QR codes list response
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct QrCodesListResponse {
    /// List of QR codes
    pub data: Vec<QrCode>,
}

/// QR code details
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct QrCode {
    /// QR code ID
    pub code: String,
    /// Pre-filled message
    pub prefilled_message: String,
    /// Deep link URL
    pub deep_link_url: String,
}
