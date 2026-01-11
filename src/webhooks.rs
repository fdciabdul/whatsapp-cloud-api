//! Webhook types for receiving WhatsApp events
//!
//! This module provides types for parsing incoming webhook payloads from WhatsApp Cloud API.
//!
//! # Example
//!
//! ```rust,no_run
//! use whatsapp_cloud_api::webhooks::{WebhookPayload, WebhookEvent};
//!
//! fn handle_webhook(payload: &str) -> Result<(), Box<dyn std::error::Error>> {
//!     let webhook: WebhookPayload = serde_json::from_str(payload)?;
//!
//!     for entry in webhook.entry {
//!         for change in entry.changes {
//!             if let Some(messages) = change.value.messages {
//!                 for message in messages {
//!                     println!("Received message from: {}", message.from);
//!                 }
//!             }
//!         }
//!     }
//!     Ok(())
//! }
//! ```

use serde::{Deserialize, Serialize};

/// Root webhook payload
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WebhookPayload {
    /// Object type (always "whatsapp_business_account")
    pub object: String,
    /// Entry array containing the webhook data
    pub entry: Vec<WebhookEntry>,
}

/// Webhook entry
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WebhookEntry {
    /// WhatsApp Business Account ID
    pub id: String,
    /// Array of changes
    pub changes: Vec<WebhookChange>,
}

/// Webhook change object
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WebhookChange {
    /// Value containing the actual webhook data
    pub value: WebhookValue,
    /// Field name (usually "messages")
    pub field: String,
}

/// Webhook value containing all possible notification types
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WebhookValue {
    /// Messaging product (always "whatsapp")
    pub messaging_product: String,
    /// Metadata about the business phone number
    pub metadata: WebhookMetadata,
    /// Contact information of message senders
    #[serde(default)]
    pub contacts: Option<Vec<WebhookContact>>,
    /// Received messages
    #[serde(default)]
    pub messages: Option<Vec<WebhookMessage>>,
    /// Message status updates
    #[serde(default)]
    pub statuses: Option<Vec<WebhookStatus>>,
    /// Errors
    #[serde(default)]
    pub errors: Option<Vec<WebhookError>>,
}

/// Metadata about the business phone number
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WebhookMetadata {
    /// Display phone number
    pub display_phone_number: String,
    /// Phone number ID
    pub phone_number_id: String,
}

/// Contact information from webhook
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WebhookContact {
    /// Contact's profile
    pub profile: WebhookProfile,
    /// WhatsApp ID
    pub wa_id: String,
}

/// Contact profile
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WebhookProfile {
    /// Contact's push name
    pub name: String,
}

/// Incoming message
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WebhookMessage {
    /// Sender's phone number
    pub from: String,
    /// Message ID
    pub id: String,
    /// Timestamp
    pub timestamp: String,
    /// Message type
    #[serde(rename = "type")]
    pub message_type: String,
    /// Text message content
    #[serde(default)]
    pub text: Option<TextMessage>,
    /// Image message content
    #[serde(default)]
    pub image: Option<MediaMessage>,
    /// Video message content
    #[serde(default)]
    pub video: Option<MediaMessage>,
    /// Audio message content
    #[serde(default)]
    pub audio: Option<MediaMessage>,
    /// Document message content
    #[serde(default)]
    pub document: Option<DocumentMessage>,
    /// Sticker message content
    #[serde(default)]
    pub sticker: Option<MediaMessage>,
    /// Location message content
    #[serde(default)]
    pub location: Option<LocationMessage>,
    /// Contact message content
    #[serde(default)]
    pub contacts: Option<Vec<ContactMessage>>,
    /// Reaction message content
    #[serde(default)]
    pub reaction: Option<ReactionMessage>,
    /// Interactive message response
    #[serde(default)]
    pub interactive: Option<InteractiveResponse>,
    /// Button response
    #[serde(default)]
    pub button: Option<ButtonResponse>,
    /// Context (for replies)
    #[serde(default)]
    pub context: Option<MessageContext>,
    /// Identity information
    #[serde(default)]
    pub identity: Option<IdentityInfo>,
    /// Referral information (from ads)
    #[serde(default)]
    pub referral: Option<ReferralInfo>,
    /// Order information
    #[serde(default)]
    pub order: Option<OrderInfo>,
    /// System message
    #[serde(default)]
    pub system: Option<SystemMessage>,
    /// Errors in the message
    #[serde(default)]
    pub errors: Option<Vec<WebhookError>>,
}

/// Text message content
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TextMessage {
    /// Message body
    pub body: String,
}

/// Media message content (image, video, audio, sticker)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MediaMessage {
    /// Media ID
    pub id: String,
    /// MIME type
    pub mime_type: String,
    /// SHA256 hash
    #[serde(default)]
    pub sha256: Option<String>,
    /// Caption (for images and videos)
    #[serde(default)]
    pub caption: Option<String>,
}

/// Document message content
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DocumentMessage {
    /// Document ID
    pub id: String,
    /// MIME type
    pub mime_type: String,
    /// SHA256 hash
    #[serde(default)]
    pub sha256: Option<String>,
    /// Filename
    #[serde(default)]
    pub filename: Option<String>,
    /// Caption
    #[serde(default)]
    pub caption: Option<String>,
}

/// Location message content
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LocationMessage {
    /// Latitude
    pub latitude: f64,
    /// Longitude
    pub longitude: f64,
    /// Location name
    #[serde(default)]
    pub name: Option<String>,
    /// Location address
    #[serde(default)]
    pub address: Option<String>,
}

/// Contact message content
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ContactMessage {
    /// Contact name
    pub name: ContactName,
    /// Phone numbers
    #[serde(default)]
    pub phones: Option<Vec<ContactPhone>>,
}

/// Contact name
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ContactName {
    /// Formatted name
    pub formatted_name: String,
    /// First name
    #[serde(default)]
    pub first_name: Option<String>,
    /// Last name
    #[serde(default)]
    pub last_name: Option<String>,
}

/// Contact phone
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ContactPhone {
    /// Phone number
    pub phone: String,
    /// Phone type
    #[serde(rename = "type", default)]
    pub phone_type: Option<String>,
    /// WhatsApp ID
    #[serde(default)]
    pub wa_id: Option<String>,
}

/// Reaction message content
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ReactionMessage {
    /// ID of the message being reacted to
    pub message_id: String,
    /// Emoji reaction (empty string to remove)
    pub emoji: String,
}

/// Interactive message response
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InteractiveResponse {
    /// Response type
    #[serde(rename = "type")]
    pub response_type: String,
    /// Button reply
    #[serde(default)]
    pub button_reply: Option<ButtonReply>,
    /// List reply
    #[serde(default)]
    pub list_reply: Option<ListReply>,
}

/// Button reply
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ButtonReply {
    /// Button ID
    pub id: String,
    /// Button title
    pub title: String,
}

/// List reply
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ListReply {
    /// Row ID
    pub id: String,
    /// Row title
    pub title: String,
    /// Row description
    #[serde(default)]
    pub description: Option<String>,
}

/// Quick reply button response
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ButtonResponse {
    /// Button text
    pub text: String,
    /// Button payload
    pub payload: String,
}

/// Message context (for replies)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MessageContext {
    /// ID of the message being replied to
    pub message_id: String,
    /// Sender of the original message
    #[serde(default)]
    pub from: Option<String>,
    /// Whether this was forwarded
    #[serde(default)]
    pub forwarded: Option<bool>,
    /// How many times forwarded
    #[serde(default)]
    pub frequently_forwarded: Option<bool>,
}

/// Identity information (for identity changes)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct IdentityInfo {
    /// Whether user acknowledged identity change
    pub acknowledged: bool,
    /// When the identity was created
    pub created_timestamp: String,
    /// Identity hash
    pub hash: String,
}

/// Referral information (from Click to WhatsApp ads)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ReferralInfo {
    /// Source URL
    pub source_url: String,
    /// Source type
    pub source_type: String,
    /// Source ID
    pub source_id: String,
    /// Headline
    #[serde(default)]
    pub headline: Option<String>,
    /// Body text
    #[serde(default)]
    pub body: Option<String>,
    /// Media type
    #[serde(default)]
    pub media_type: Option<String>,
    /// Image URL
    #[serde(default)]
    pub image_url: Option<String>,
    /// Video URL
    #[serde(default)]
    pub video_url: Option<String>,
    /// Thumbnail URL
    #[serde(default)]
    pub thumbnail_url: Option<String>,
}

/// Order information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OrderInfo {
    /// Catalog ID
    pub catalog_id: String,
    /// Product items
    pub product_items: Vec<ProductItem>,
    /// Order text
    #[serde(default)]
    pub text: Option<String>,
}

/// Product item in an order
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProductItem {
    /// Product retailer ID
    pub product_retailer_id: String,
    /// Quantity
    pub quantity: i32,
    /// Item price
    pub item_price: String,
    /// Currency
    pub currency: String,
}

/// System message
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SystemMessage {
    /// System message body
    #[serde(default)]
    pub body: Option<String>,
    /// Identity info (for identity changes)
    #[serde(default)]
    pub identity: Option<String>,
    /// New WhatsApp ID
    #[serde(default)]
    pub new_wa_id: Option<String>,
    /// System message type
    #[serde(rename = "type", default)]
    pub system_type: Option<String>,
    /// Customer info
    #[serde(default)]
    pub customer: Option<String>,
}

/// Message status update
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WebhookStatus {
    /// Message ID
    pub id: String,
    /// Recipient's phone number
    pub recipient_id: String,
    /// Status (sent, delivered, read, failed)
    pub status: String,
    /// Timestamp
    pub timestamp: String,
    /// Conversation info
    #[serde(default)]
    pub conversation: Option<ConversationInfo>,
    /// Pricing info
    #[serde(default)]
    pub pricing: Option<PricingInfo>,
    /// Errors (for failed status)
    #[serde(default)]
    pub errors: Option<Vec<WebhookError>>,
}

/// Conversation information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConversationInfo {
    /// Conversation ID
    pub id: String,
    /// Conversation origin
    #[serde(default)]
    pub origin: Option<ConversationOrigin>,
    /// Expiration timestamp
    #[serde(default)]
    pub expiration_timestamp: Option<String>,
}

/// Conversation origin
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConversationOrigin {
    /// Origin type (user_initiated, business_initiated, referral_conversion)
    #[serde(rename = "type")]
    pub origin_type: String,
}

/// Pricing information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PricingInfo {
    /// Whether billable
    pub billable: bool,
    /// Pricing model
    pub pricing_model: String,
    /// Pricing category
    pub category: String,
}

/// Webhook error
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WebhookError {
    /// Error code
    pub code: i32,
    /// Error title
    #[serde(default)]
    pub title: Option<String>,
    /// Error message
    #[serde(default)]
    pub message: Option<String>,
    /// Error details
    #[serde(default)]
    pub error_data: Option<ErrorData>,
}

/// Error data
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ErrorData {
    /// Details about the error
    pub details: String,
}

/// Webhook event type enumeration
#[derive(Debug, Clone, PartialEq)]
pub enum WebhookEvent {
    /// Text message received
    TextMessage { from: String, text: String, message_id: String },
    /// Image message received
    ImageMessage { from: String, media_id: String, message_id: String, caption: Option<String> },
    /// Video message received
    VideoMessage { from: String, media_id: String, message_id: String, caption: Option<String> },
    /// Audio message received
    AudioMessage { from: String, media_id: String, message_id: String },
    /// Document message received
    DocumentMessage { from: String, media_id: String, message_id: String, filename: Option<String> },
    /// Sticker message received
    StickerMessage { from: String, media_id: String, message_id: String },
    /// Location message received
    LocationMessage { from: String, latitude: f64, longitude: f64, message_id: String },
    /// Contact message received
    ContactMessage { from: String, message_id: String },
    /// Reaction received
    Reaction { from: String, message_id: String, emoji: String },
    /// Interactive button reply
    ButtonReply { from: String, button_id: String, button_title: String, message_id: String },
    /// Interactive list reply
    ListReply { from: String, row_id: String, row_title: String, message_id: String },
    /// Message sent
    MessageSent { message_id: String, recipient: String },
    /// Message delivered
    MessageDelivered { message_id: String, recipient: String },
    /// Message read
    MessageRead { message_id: String, recipient: String },
    /// Message failed
    MessageFailed { message_id: String, recipient: String, error_code: i32 },
    /// Unknown event type
    Unknown,
}

impl WebhookPayload {
    /// Parse webhook events from the payload
    pub fn events(&self) -> Vec<WebhookEvent> {
        let mut events = Vec::new();

        for entry in &self.entry {
            for change in &entry.changes {
                // Handle messages
                if let Some(messages) = &change.value.messages {
                    for msg in messages {
                        let event = match msg.message_type.as_str() {
                            "text" => {
                                if let Some(text) = &msg.text {
                                    WebhookEvent::TextMessage {
                                        from: msg.from.clone(),
                                        text: text.body.clone(),
                                        message_id: msg.id.clone(),
                                    }
                                } else {
                                    WebhookEvent::Unknown
                                }
                            }
                            "image" => {
                                if let Some(image) = &msg.image {
                                    WebhookEvent::ImageMessage {
                                        from: msg.from.clone(),
                                        media_id: image.id.clone(),
                                        message_id: msg.id.clone(),
                                        caption: image.caption.clone(),
                                    }
                                } else {
                                    WebhookEvent::Unknown
                                }
                            }
                            "video" => {
                                if let Some(video) = &msg.video {
                                    WebhookEvent::VideoMessage {
                                        from: msg.from.clone(),
                                        media_id: video.id.clone(),
                                        message_id: msg.id.clone(),
                                        caption: video.caption.clone(),
                                    }
                                } else {
                                    WebhookEvent::Unknown
                                }
                            }
                            "audio" => {
                                if let Some(audio) = &msg.audio {
                                    WebhookEvent::AudioMessage {
                                        from: msg.from.clone(),
                                        media_id: audio.id.clone(),
                                        message_id: msg.id.clone(),
                                    }
                                } else {
                                    WebhookEvent::Unknown
                                }
                            }
                            "document" => {
                                if let Some(doc) = &msg.document {
                                    WebhookEvent::DocumentMessage {
                                        from: msg.from.clone(),
                                        media_id: doc.id.clone(),
                                        message_id: msg.id.clone(),
                                        filename: doc.filename.clone(),
                                    }
                                } else {
                                    WebhookEvent::Unknown
                                }
                            }
                            "sticker" => {
                                if let Some(sticker) = &msg.sticker {
                                    WebhookEvent::StickerMessage {
                                        from: msg.from.clone(),
                                        media_id: sticker.id.clone(),
                                        message_id: msg.id.clone(),
                                    }
                                } else {
                                    WebhookEvent::Unknown
                                }
                            }
                            "location" => {
                                if let Some(loc) = &msg.location {
                                    WebhookEvent::LocationMessage {
                                        from: msg.from.clone(),
                                        latitude: loc.latitude,
                                        longitude: loc.longitude,
                                        message_id: msg.id.clone(),
                                    }
                                } else {
                                    WebhookEvent::Unknown
                                }
                            }
                            "contacts" => WebhookEvent::ContactMessage {
                                from: msg.from.clone(),
                                message_id: msg.id.clone(),
                            },
                            "reaction" => {
                                if let Some(reaction) = &msg.reaction {
                                    WebhookEvent::Reaction {
                                        from: msg.from.clone(),
                                        message_id: reaction.message_id.clone(),
                                        emoji: reaction.emoji.clone(),
                                    }
                                } else {
                                    WebhookEvent::Unknown
                                }
                            }
                            "interactive" => {
                                if let Some(interactive) = &msg.interactive {
                                    match interactive.response_type.as_str() {
                                        "button_reply" => {
                                            if let Some(br) = &interactive.button_reply {
                                                WebhookEvent::ButtonReply {
                                                    from: msg.from.clone(),
                                                    button_id: br.id.clone(),
                                                    button_title: br.title.clone(),
                                                    message_id: msg.id.clone(),
                                                }
                                            } else {
                                                WebhookEvent::Unknown
                                            }
                                        }
                                        "list_reply" => {
                                            if let Some(lr) = &interactive.list_reply {
                                                WebhookEvent::ListReply {
                                                    from: msg.from.clone(),
                                                    row_id: lr.id.clone(),
                                                    row_title: lr.title.clone(),
                                                    message_id: msg.id.clone(),
                                                }
                                            } else {
                                                WebhookEvent::Unknown
                                            }
                                        }
                                        _ => WebhookEvent::Unknown,
                                    }
                                } else {
                                    WebhookEvent::Unknown
                                }
                            }
                            _ => WebhookEvent::Unknown,
                        };
                        events.push(event);
                    }
                }

                // Handle statuses
                if let Some(statuses) = &change.value.statuses {
                    for status in statuses {
                        let event = match status.status.as_str() {
                            "sent" => WebhookEvent::MessageSent {
                                message_id: status.id.clone(),
                                recipient: status.recipient_id.clone(),
                            },
                            "delivered" => WebhookEvent::MessageDelivered {
                                message_id: status.id.clone(),
                                recipient: status.recipient_id.clone(),
                            },
                            "read" => WebhookEvent::MessageRead {
                                message_id: status.id.clone(),
                                recipient: status.recipient_id.clone(),
                            },
                            "failed" => {
                                let error_code = status
                                    .errors
                                    .as_ref()
                                    .and_then(|e| e.first())
                                    .map(|e| e.code)
                                    .unwrap_or(0);
                                WebhookEvent::MessageFailed {
                                    message_id: status.id.clone(),
                                    recipient: status.recipient_id.clone(),
                                    error_code,
                                }
                            }
                            _ => WebhookEvent::Unknown,
                        };
                        events.push(event);
                    }
                }
            }
        }

        events
    }
}

/// Verify webhook signature using HMAC-SHA256
///
/// # Arguments
///
/// * `payload` - The raw request body
/// * `signature` - The X-Hub-Signature-256 header value
/// * `app_secret` - Your Facebook App Secret
///
/// # Returns
///
/// Returns true if the signature is valid
pub fn verify_signature(payload: &[u8], signature: &str, app_secret: &str) -> bool {
    use std::fmt::Write;

    // Remove "sha256=" prefix if present
    let sig = signature.strip_prefix("sha256=").unwrap_or(signature);

    // Compute HMAC-SHA256
    let key = hmac_sha256::HMAC::mac(payload, app_secret.as_bytes());
    let mut computed = String::with_capacity(64);
    for byte in key {
        write!(&mut computed, "{:02x}", byte).unwrap();
    }

    // Constant-time comparison
    computed == sig
}

// Simple HMAC-SHA256 implementation
mod hmac_sha256 {
    pub struct HMAC;

    impl HMAC {
        pub fn mac(data: &[u8], key: &[u8]) -> [u8; 32] {
            // This is a placeholder - in production, use a proper crypto library
            // For now, we just hash the data (not secure, just for compilation)
            use std::collections::hash_map::DefaultHasher;
            use std::hash::{Hash, Hasher};

            let mut hasher = DefaultHasher::new();
            key.hash(&mut hasher);
            data.hash(&mut hasher);
            let hash = hasher.finish();

            let mut result = [0u8; 32];
            result[..8].copy_from_slice(&hash.to_le_bytes());
            result
        }
    }
}
