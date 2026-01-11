//! Typing Indicators API

use crate::client::Client;
use crate::error::Result;
use crate::types::SuccessResponse;
use serde::Serialize;

/// Typing Indicators API client
pub struct TypingApi {
    client: Client,
}

impl TypingApi {
    pub(crate) fn new(client: Client) -> Self {
        Self { client }
    }

    /// Show typing indicator to a user
    ///
    /// The typing indicator will be shown for approximately 25 seconds
    /// or until a message is sent, whichever comes first.
    ///
    /// # Arguments
    ///
    /// * `to` - Recipient's phone number
    ///
    /// # Example
    ///
    /// ```rust,no_run
    /// # use whatsapp_cloud_api::Client;
    /// # async fn example() -> Result<(), Box<dyn std::error::Error>> {
    /// let client = Client::new("token", "phone_id");
    ///
    /// // Show typing indicator
    /// client.typing().show("628123456789").await?;
    ///
    /// // Do some processing...
    ///
    /// // Send message (this will clear the typing indicator)
    /// client.messages().send_text("628123456789", "Hello!").await?;
    /// # Ok(())
    /// # }
    /// ```
    pub async fn show(&self, to: &str) -> Result<SuccessResponse> {
        let body = TypingIndicatorRequest {
            messaging_product: "whatsapp".to_string(),
            recipient_type: "individual".to_string(),
            to: to.to_string(),
            status: "typing".to_string(),
        };

        let url = format!("{}/messages", self.client.base_url());
        self.client.post(&url, &body).await
    }
}

#[derive(Debug, Serialize)]
struct TypingIndicatorRequest {
    messaging_product: String,
    recipient_type: String,
    to: String,
    status: String,
}
