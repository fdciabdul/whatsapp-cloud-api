//! # WhatsApp Cloud API SDK for Rust
//!
//! A Rust SDK for the [WhatsApp Cloud API](https://developers.facebook.com/docs/whatsapp/cloud-api)
//! hosted by Meta. This library provides a type-safe, async interface for integrating
//! WhatsApp Business messaging into your Rust applications.
//!
//! ## Features
//!
//! - **Messages**: Send text, media, templates, interactive messages, and more
//! - **Media**: Upload, download, and manage media files
//! - **Templates**: Work with message templates
//! - **Phone Numbers**: Manage business phone numbers
//! - **Products**: Catalog and product messages
//! - **Flows**: WhatsApp Flows support
//! - **Analytics**: Conversation and template analytics
//! - **QR Codes**: Generate and manage QR codes
//! - **Block Users**: Block/unblock users
//! - **WABA Management**: WhatsApp Business Account management
//! - **Webhooks**: Type-safe webhook payload parsing and subscription management
//!
//! ## Quick Start
//!
//! ```rust,no_run
//! use wacloudapi::Client;
//!
//! #[tokio::main]
//! async fn main() -> Result<(), Box<dyn std::error::Error>> {
//!     let client = Client::new("YOUR_ACCESS_TOKEN", "YOUR_PHONE_NUMBER_ID");
//!
//!     // Send a text message
//!     let response = client
//!         .messages()
//!         .send_text("628123456789", "Hello from Rust!")
//!         .await?;
//!
//!     println!("Message sent: {:?}", response.messages[0].id);
//!     Ok(())
//! }
//! ```

pub mod analytics;
pub mod block;
pub mod client;
pub mod error;
pub mod flows;
pub mod media;
pub mod messages;
pub mod phone_numbers;
pub mod products;
pub mod qr_codes;
pub mod templates;
pub mod types;
pub mod typing;
pub mod waba;
pub mod webhooks;
pub mod webhooks_management;

pub use client::Client;
pub use error::{Error, Result};
