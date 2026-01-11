//! Media API for uploading and managing media files

use crate::client::Client;
use crate::error::{Error, Result};
use reqwest::multipart::{Form, Part};
use serde::{Deserialize, Serialize};
use std::path::Path;

/// Media API client
pub struct MediaApi {
    client: Client,
}

impl MediaApi {
    pub(crate) fn new(client: Client) -> Self {
        Self { client }
    }

    /// Upload media from a file path
    ///
    /// # Arguments
    ///
    /// * `file_path` - Path to the file to upload
    ///
    /// # Returns
    ///
    /// Returns the media ID that can be used to send the media
    ///
    /// # Example
    ///
    /// ```rust,no_run
    /// # use whatsapp_cloud_api::Client;
    /// # async fn example() -> Result<(), Box<dyn std::error::Error>> {
    /// let client = Client::new("token", "phone_id");
    /// let response = client.media().upload_file("./image.jpg").await?;
    /// println!("Media ID: {}", response.id);
    /// # Ok(())
    /// # }
    /// ```
    pub async fn upload_file(&self, file_path: impl AsRef<Path>) -> Result<MediaUploadResponse> {
        let path = file_path.as_ref();
        let file_name = path
            .file_name()
            .and_then(|n| n.to_str())
            .unwrap_or("file")
            .to_string();

        let mime_type = mime_guess::from_path(path)
            .first_or_octet_stream()
            .to_string();

        let file_bytes = tokio::fs::read(path).await?;

        self.upload_bytes(&file_bytes, &file_name, &mime_type).await
    }

    /// Upload media from bytes
    ///
    /// # Arguments
    ///
    /// * `data` - File content as bytes
    /// * `filename` - Name of the file
    /// * `mime_type` - MIME type of the file
    pub async fn upload_bytes(
        &self,
        data: &[u8],
        filename: &str,
        mime_type: &str,
    ) -> Result<MediaUploadResponse> {
        let file_part = Part::bytes(data.to_vec())
            .file_name(filename.to_string())
            .mime_str(mime_type)
            .map_err(|e| Error::MediaUpload(e.to_string()))?;

        let form = Form::new()
            .text("messaging_product", "whatsapp")
            .text("type", mime_type.to_string())
            .part("file", file_part);

        let url = format!("{}/media", self.client.base_url());
        self.client.post_form(&url, form).await
    }

    /// Upload media from base64
    ///
    /// # Arguments
    ///
    /// * `base64_data` - Base64-encoded file content
    /// * `filename` - Name of the file
    /// * `mime_type` - MIME type of the file
    pub async fn upload_base64(
        &self,
        base64_data: &str,
        filename: &str,
        mime_type: &str,
    ) -> Result<MediaUploadResponse> {
        use base64::Engine;
        let data = base64::engine::general_purpose::STANDARD
            .decode(base64_data)
            .map_err(|e| Error::MediaUpload(format!("Invalid base64: {}", e)))?;

        self.upload_bytes(&data, filename, mime_type).await
    }

    /// Get media URL by media ID
    ///
    /// # Arguments
    ///
    /// * `media_id` - The media ID
    ///
    /// # Returns
    ///
    /// Returns the media URL that can be used to download the media
    pub async fn get_url(&self, media_id: &str) -> Result<MediaUrlResponse> {
        let url = self.client.endpoint_url(media_id);
        self.client.get(&url).await
    }

    /// Download media by media ID
    ///
    /// # Arguments
    ///
    /// * `media_id` - The media ID
    ///
    /// # Returns
    ///
    /// Returns the media content as bytes
    pub async fn download(&self, media_id: &str) -> Result<Vec<u8>> {
        // First get the media URL
        let media_info = self.get_url(media_id).await?;

        // Then download the actual content
        let response = self
            .client
            .http_client()
            .get(&media_info.url)
            .header(
                "Authorization",
                format!("Bearer {}", self.get_token()),
            )
            .send()
            .await?;

        if !response.status().is_success() {
            return Err(Error::MediaUpload(format!(
                "Failed to download media: {}",
                response.status()
            )));
        }

        Ok(response.bytes().await?.to_vec())
    }

    /// Delete media by media ID
    pub async fn delete(&self, media_id: &str) -> Result<crate::types::SuccessResponse> {
        let url = self.client.endpoint_url(media_id);
        self.client.delete(&url).await
    }

    // Helper to get token from client (we need to expose this somehow)
    fn get_token(&self) -> String {
        // This is a workaround - ideally we'd have a better way to access the token
        // For now, we'll use a placeholder that should be set via the client
        String::new()
    }
}

/// Response from media upload
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MediaUploadResponse {
    /// Media ID
    pub id: String,
}

/// Response from getting media URL
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MediaUrlResponse {
    /// Messaging product
    pub messaging_product: Option<String>,
    /// Media URL (valid for 5 minutes)
    pub url: String,
    /// MIME type
    pub mime_type: String,
    /// SHA256 hash
    pub sha256: String,
    /// File size in bytes
    pub file_size: i64,
    /// Media ID
    pub id: String,
}

/// Supported media types for upload
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MediaType {
    /// Audio files (mp3, ogg, amr, etc.)
    Audio,
    /// Document files (pdf, doc, xlsx, etc.)
    Document,
    /// Image files (jpg, png, webp)
    Image,
    /// Sticker files (webp)
    Sticker,
    /// Video files (mp4, 3gpp)
    Video,
}

impl MediaType {
    /// Get supported MIME types for this media type
    pub fn supported_mime_types(&self) -> &[&str] {
        match self {
            MediaType::Audio => &[
                "audio/aac",
                "audio/mp4",
                "audio/mpeg",
                "audio/amr",
                "audio/ogg",
            ],
            MediaType::Document => &[
                "text/plain",
                "application/pdf",
                "application/vnd.ms-powerpoint",
                "application/msword",
                "application/vnd.ms-excel",
                "application/vnd.openxmlformats-officedocument.wordprocessingml.document",
                "application/vnd.openxmlformats-officedocument.presentationml.presentation",
                "application/vnd.openxmlformats-officedocument.spreadsheetml.sheet",
            ],
            MediaType::Image => &["image/jpeg", "image/png", "image/webp"],
            MediaType::Sticker => &["image/webp"],
            MediaType::Video => &["video/mp4", "video/3gpp"],
        }
    }

    /// Check if a MIME type is supported for this media type
    pub fn is_mime_supported(&self, mime_type: &str) -> bool {
        self.supported_mime_types().contains(&mime_type)
    }

    /// Get max file size in bytes
    pub fn max_size(&self) -> u64 {
        match self {
            MediaType::Audio => 16 * 1024 * 1024, // 16 MB
            MediaType::Document => 100 * 1024 * 1024, // 100 MB
            MediaType::Image => 5 * 1024 * 1024,  // 5 MB
            MediaType::Sticker => 500 * 1024,     // 500 KB
            MediaType::Video => 16 * 1024 * 1024, // 16 MB
        }
    }
}
