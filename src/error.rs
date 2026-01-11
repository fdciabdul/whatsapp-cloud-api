//! Error types for the WhatsApp Cloud API SDK

use serde::{Deserialize, Serialize};
use thiserror::Error;

/// Result type alias for WhatsApp Cloud API operations
pub type Result<T> = std::result::Result<T, Error>;

/// Errors that can occur when using the WhatsApp Cloud API
#[derive(Error, Debug)]
pub enum Error {
    /// HTTP request failed
    #[error("HTTP request failed: {0}")]
    Request(#[from] reqwest::Error),

    /// JSON serialization/deserialization error
    #[error("JSON error: {0}")]
    Json(#[from] serde_json::Error),

    /// URL parsing error
    #[error("URL parse error: {0}")]
    UrlParse(#[from] url::ParseError),

    /// API error returned by WhatsApp Cloud API
    #[error("API error: {message} (code: {code})")]
    Api {
        code: i32,
        message: String,
        error_subcode: Option<i32>,
        error_data: Option<ApiErrorData>,
    },

    /// Rate limit exceeded
    #[error("Rate limit exceeded. Retry after {retry_after:?} seconds")]
    RateLimited { retry_after: Option<u64> },

    /// Invalid access token
    #[error("Invalid or expired access token")]
    InvalidToken,

    /// Media upload failed
    #[error("Media upload failed: {0}")]
    MediaUpload(String),

    /// Invalid phone number format
    #[error("Invalid phone number format: {0}")]
    InvalidPhoneNumber(String),

    /// Message not sent
    #[error("Message not sent: {0}")]
    MessageNotSent(String),

    /// IO error
    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),
}

/// Additional error data from the API
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ApiErrorData {
    /// Messaging product (always "whatsapp")
    pub messaging_product: Option<String>,
    /// Details about the error
    pub details: Option<String>,
}

/// Error response from the WhatsApp Cloud API
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ApiErrorResponse {
    /// Error object
    pub error: ApiError,
}

/// Error object from the API
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ApiError {
    /// Error message
    pub message: String,
    /// Error type
    #[serde(rename = "type")]
    pub error_type: Option<String>,
    /// Error code
    pub code: i32,
    /// Error subcode
    pub error_subcode: Option<i32>,
    /// Error user title
    pub error_user_title: Option<String>,
    /// Error user message
    pub error_user_msg: Option<String>,
    /// Facebook trace ID
    pub fbtrace_id: Option<String>,
    /// Additional error data
    pub error_data: Option<ApiErrorData>,
}

impl From<ApiErrorResponse> for Error {
    fn from(response: ApiErrorResponse) -> Self {
        let err = response.error;

        // Check for specific error codes
        match err.code {
            190 => Error::InvalidToken,
            4 | 17 | 32 | 613 => Error::RateLimited { retry_after: None },
            _ => Error::Api {
                code: err.code,
                message: err.message,
                error_subcode: err.error_subcode,
                error_data: err.error_data,
            },
        }
    }
}
