//! Block Users API for WhatsApp Business

use crate::client::Client;
use crate::error::Result;
use serde::{Deserialize, Serialize};

/// Block Users API client
pub struct BlockApi {
    client: Client,
}

impl BlockApi {
    pub(crate) fn new(client: Client) -> Self {
        Self { client }
    }

    /// Block a user
    ///
    /// # Arguments
    ///
    /// * `user_phone_number` - The phone number to block
    pub async fn block_user(&self, user_phone_number: &str) -> Result<BlockResponse> {
        let body = BlockUserRequest {
            messaging_product: "whatsapp".to_string(),
            block: vec![UserToBlock {
                user: user_phone_number.to_string(),
            }],
        };

        let url = format!("{}/block", self.client.base_url());
        self.client.post(&url, &body).await
    }

    /// Block multiple users
    ///
    /// # Arguments
    ///
    /// * `user_phone_numbers` - List of phone numbers to block
    pub async fn block_users(&self, user_phone_numbers: Vec<&str>) -> Result<BlockResponse> {
        let body = BlockUserRequest {
            messaging_product: "whatsapp".to_string(),
            block: user_phone_numbers
                .into_iter()
                .map(|u| UserToBlock {
                    user: u.to_string(),
                })
                .collect(),
        };

        let url = format!("{}/block", self.client.base_url());
        self.client.post(&url, &body).await
    }

    /// Unblock a user
    ///
    /// # Arguments
    ///
    /// * `user_phone_number` - The phone number to unblock
    pub async fn unblock_user(&self, user_phone_number: &str) -> Result<BlockResponse> {
        let body = UnblockUserRequest {
            messaging_product: "whatsapp".to_string(),
            unblock: vec![UserToBlock {
                user: user_phone_number.to_string(),
            }],
        };

        let url = format!("{}/block", self.client.base_url());
        self.client.post(&url, &body).await
    }

    /// Unblock multiple users
    ///
    /// # Arguments
    ///
    /// * `user_phone_numbers` - List of phone numbers to unblock
    pub async fn unblock_users(&self, user_phone_numbers: Vec<&str>) -> Result<BlockResponse> {
        let body = UnblockUserRequest {
            messaging_product: "whatsapp".to_string(),
            unblock: user_phone_numbers
                .into_iter()
                .map(|u| UserToBlock {
                    user: u.to_string(),
                })
                .collect(),
        };

        let url = format!("{}/block", self.client.base_url());
        self.client.post(&url, &body).await
    }

    /// Get list of blocked users
    pub async fn get_blocked_users(&self) -> Result<BlockedUsersResponse> {
        let url = format!("{}/block", self.client.base_url());
        self.client.get(&url).await
    }
}

// Request types

#[derive(Debug, Serialize)]
struct BlockUserRequest {
    messaging_product: String,
    block: Vec<UserToBlock>,
}

#[derive(Debug, Serialize)]
struct UnblockUserRequest {
    messaging_product: String,
    unblock: Vec<UserToBlock>,
}

#[derive(Debug, Serialize)]
struct UserToBlock {
    user: String,
}

// Response types

/// Block/Unblock response
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BlockResponse {
    /// Success status for each user
    #[serde(default)]
    pub data: Vec<BlockResult>,
}

/// Block result for a single user
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BlockResult {
    /// Input user phone number
    pub input: String,
    /// WhatsApp ID
    #[serde(skip_serializing_if = "Option::is_none")]
    pub wa_id: Option<String>,
    /// Success status
    pub success: bool,
}

/// Blocked users list response
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BlockedUsersResponse {
    /// List of blocked users
    #[serde(default)]
    pub data: Vec<BlockedUser>,
}

/// Blocked user info
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BlockedUser {
    /// WhatsApp ID of blocked user
    pub wa_id: String,
}
