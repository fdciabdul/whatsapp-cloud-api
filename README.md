# wacloudapi

[![Crates.io](https://img.shields.io/crates/v/wacloudapi.svg)](https://crates.io/crates/wacloudapi)
[![Documentation](https://docs.rs/wacloudapi/badge.svg)](https://docs.rs/wacloudapi)
[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)

A comprehensive Rust SDK for the [WhatsApp Cloud API](https://developers.facebook.com/docs/whatsapp/cloud-api) hosted by Meta. This library provides a type-safe, async interface for integrating WhatsApp Business messaging into your Rust applications.

## Features

- **Messages API** - Send text, media, templates, interactive messages, and more
- **Media API** - Upload, download, and manage media files
- **Templates API** - Create and manage message templates
- **Phone Numbers API** - Manage business phone numbers and profiles
- **Products/Catalog API** - Send product and catalog messages
- **Flows API** - Create and manage WhatsApp Flows
- **QR Codes API** - Generate and manage QR codes
- **Analytics API** - Get conversation and template analytics
- **Block Users API** - Block and unblock users
- **WABA Management API** - Manage WhatsApp Business Accounts
- **Webhooks** - Type-safe webhook payload parsing and subscription management
- **Typing Indicators** - Show typing status to users
- **Async/Await** - Built on Tokio for async operations
- **Type-Safe** - Strongly typed API with Serde serialization

## Installation

Add this to your `Cargo.toml`:

```toml
[dependencies]
wacloudapi = "0.1"
tokio = { version = "1", features = ["full"] }
```

Or use cargo:

```bash
cargo add wacloudapi
```

## Quick Start

```rust
use wacloudapi::Client;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Create a client
    let client = Client::new(
        "YOUR_ACCESS_TOKEN",
        "YOUR_PHONE_NUMBER_ID"
    );

    // Send a text message
    let response = client
        .messages()
        .send_text("628123456789", "Hello from Rust!")
        .await?;

    println!("Message sent: {:?}", response.messages[0].id);
    Ok(())
}
```

## Usage Examples

### Send Text Message

```rust
let response = client
    .messages()
    .send_text("628123456789", "Hello!")
    .await?;
```

### Send Text with URL Preview

```rust
let response = client
    .messages()
    .send_text_with_preview("628123456789", "Check this out: https://example.com")
    .await?;
```

### Send Image

```rust
// By URL
let response = client
    .messages()
    .send_image_url("628123456789", "https://example.com/image.jpg", Some("Caption"))
    .await?;

// By Media ID (after upload)
let response = client
    .messages()
    .send_image_id("628123456789", "media_id_here", Some("Caption"))
    .await?;
```

### Send Video

```rust
let response = client
    .messages()
    .send_video_url("628123456789", "https://example.com/video.mp4", Some("Watch this!"))
    .await?;
```

### Send Document

```rust
let response = client
    .messages()
    .send_document_url(
        "628123456789",
        "https://example.com/document.pdf",
        Some("report.pdf"),
        Some("Here's the report")
    )
    .await?;
```

### Send Location

```rust
let response = client
    .messages()
    .send_location(
        "628123456789",
        -6.2088,           // latitude
        106.8456,          // longitude
        Some("Jakarta"),   // name
        Some("Indonesia")  // address
    )
    .await?;
```

### Send Reaction

```rust
// Add reaction
let response = client
    .messages()
    .send_reaction("628123456789", "message_id_here", "👍")
    .await?;

// Remove reaction
let response = client
    .messages()
    .remove_reaction("628123456789", "message_id_here")
    .await?;
```

### Send Interactive Buttons

```rust
use wacloudapi::messages::Button;

let buttons = vec![
    Button::reply("btn_yes", "Yes"),
    Button::reply("btn_no", "No"),
    Button::reply("btn_maybe", "Maybe"),
];

let response = client
    .messages()
    .send_buttons(
        "628123456789",
        Some("Question"),         // header
        "Do you want to proceed?", // body
        Some("Tap a button"),     // footer
        buttons
    )
    .await?;
```

### Send Interactive List

```rust
use wacloudapi::messages::{ListSection, ListRow};

let sections = vec![
    ListSection {
        title: "Products".to_string(),
        rows: vec![
            ListRow::new("prod_1", "Product 1").with_description("$10.00"),
            ListRow::new("prod_2", "Product 2").with_description("$20.00"),
        ],
    },
];

let response = client
    .messages()
    .send_list(
        "628123456789",
        Some("Our Products"),   // header
        "Choose a product",     // body
        Some("Powered by WA"),  // footer
        "View Products",        // button text
        sections
    )
    .await?;
```

### Send Template Message

```rust
use wacloudapi::messages::{TemplateComponent, TemplateParameter};

let components = vec![
    TemplateComponent {
        component_type: "body".to_string(),
        sub_type: None,
        index: None,
        parameters: Some(vec![
            TemplateParameter {
                param_type: "text".to_string(),
                text: Some("John".to_string()),
                currency: None,
                date_time: None,
                image: None,
                document: None,
                video: None,
            },
        ]),
    },
];

let response = client
    .messages()
    .send_template("628123456789", "hello_world", "en_US", Some(components))
    .await?;
```

### Upload Media

```rust
// From file
let response = client
    .media()
    .upload_file("./image.jpg")
    .await?;
println!("Media ID: {}", response.id);

// From bytes
let response = client
    .media()
    .upload_bytes(&file_bytes, "image.jpg", "image/jpeg")
    .await?;
```

### Show Typing Indicator

```rust
client
    .typing()
    .show("628123456789")
    .await?;
```

### Send Product Message

```rust
let response = client
    .products()
    .send_product("628123456789", "catalog_id", "product_id", Some("Check this out!"))
    .await?;
```

### Send Flow Message

```rust
let response = client
    .flows()
    .send_flow(
        "628123456789",
        "flow_id",
        "flow_token",
        "Start Flow",
        None,
        None
    )
    .await?;
```

### Block/Unblock Users

```rust
// Block a user
client.block().block_user("628123456789").await?;

// Unblock a user
client.block().unblock_user("628123456789").await?;

// Get blocked users list
let blocked = client.block().get_blocked_users().await?;
```

### Create QR Code

```rust
let qr = client
    .qr_codes()
    .create("Hello! How can I help you?", "PNG")
    .await?;
println!("QR Code URL: {}", qr.qr_image_url);
```

### Get Analytics

```rust
let analytics = client
    .analytics("WABA_ID")
    .get_conversation_analytics(
        1704067200,  // start timestamp
        1706745600,  // end timestamp
        "DAILY"      // granularity
    )
    .await?;
```

### Handle Webhooks

```rust
use wacloudapi::webhooks::{WebhookPayload, WebhookEvent};

fn handle_webhook(payload: &str) -> Result<(), Box<dyn std::error::Error>> {
    let webhook: WebhookPayload = serde_json::from_str(payload)?;

    for event in webhook.events() {
        match event {
            WebhookEvent::TextMessage { from, text, message_id } => {
                println!("Text from {}: {}", from, text);
            }
            WebhookEvent::ImageMessage { from, media_id, .. } => {
                println!("Image from {}: {}", from, media_id);
            }
            WebhookEvent::ButtonReply { from, button_id, button_title, .. } => {
                println!("Button {} clicked by {}", button_title, from);
            }
            WebhookEvent::MessageDelivered { message_id, recipient } => {
                println!("Message {} delivered to {}", message_id, recipient);
            }
            _ => {}
        }
    }
    Ok(())
}
```

## API Reference

### Client

| Method | Description |
|--------|-------------|
| `Client::new(token, phone_id)` | Create a new client |
| `Client::with_version(token, phone_id, version)` | Create with custom API version |
| `client.messages()` | Access Messages API |
| `client.media()` | Access Media API |
| `client.phone_numbers()` | Access Phone Numbers API |
| `client.templates()` | Access Templates API |
| `client.products()` | Access Products/Catalog API |
| `client.flows()` | Access Flows API |
| `client.typing()` | Access Typing Indicator API |
| `client.qr_codes()` | Access QR Codes API |
| `client.block()` | Access Block Users API |
| `client.analytics(waba_id)` | Access Analytics API |
| `client.waba(waba_id)` | Access WABA Management API |
| `client.webhook_subscriptions(app_id)` | Access Webhook Subscriptions API |

### Messages API

| Method | Description |
|--------|-------------|
| `send_text(to, text)` | Send text message |
| `send_text_with_preview(to, text)` | Send text with URL preview |
| `send_reply(to, text, message_id)` | Reply to a message |
| `send_reaction(to, message_id, emoji)` | React to a message |
| `send_image_url(to, url, caption)` | Send image by URL |
| `send_image_id(to, media_id, caption)` | Send image by media ID |
| `send_video_url(to, url, caption)` | Send video by URL |
| `send_audio_url(to, url)` | Send audio by URL |
| `send_document_url(to, url, filename, caption)` | Send document by URL |
| `send_sticker_url(to, url)` | Send sticker by URL |
| `send_location(to, lat, lng, name, address)` | Send location |
| `send_contacts(to, contacts)` | Send contact card |
| `send_template(to, name, lang, components)` | Send template message |
| `send_list(to, header, body, footer, btn, sections)` | Send list message |
| `send_buttons(to, header, body, footer, buttons)` | Send button message |
| `mark_as_read(message_id)` | Mark message as read |

### Media API

| Method | Description |
|--------|-------------|
| `upload_file(path)` | Upload media from file |
| `upload_bytes(data, filename, mime)` | Upload media from bytes |
| `upload_base64(data, filename, mime)` | Upload media from base64 |
| `get_url(media_id)` | Get media download URL |
| `download(media_id)` | Download media content |
| `delete(media_id)` | Delete media |

## Environment Variables

You can set these environment variables:

```bash
WHATSAPP_ACCESS_TOKEN=your_access_token
WHATSAPP_PHONE_NUMBER_ID=your_phone_number_id
```

## Getting Started with WhatsApp Cloud API

1. Create a [Meta Developer Account](https://developers.facebook.com/)
2. Create a new app and add the WhatsApp product
3. Get your access token and phone number ID from the app dashboard
4. Set up webhooks to receive messages

For more information, see the [WhatsApp Cloud API documentation](https://developers.facebook.com/docs/whatsapp/cloud-api).

## License

MIT

## Author

Abdul Muttaqin - [@taqin](https://github.com/fdciabdul)
