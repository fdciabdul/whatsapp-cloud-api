//! Messages API for sending WhatsApp messages

use crate::client::Client;
use crate::error::Result;
use crate::types::MessageResponse;
use serde::{Deserialize, Serialize};

/// Messages API client
pub struct MessagesApi {
    client: Client,
}

impl MessagesApi {
    pub(crate) fn new(client: Client) -> Self {
        Self { client }
    }

    /// Send a text message
    ///
    /// # Arguments
    ///
    /// * `to` - Recipient's phone number (with country code, no + or spaces)
    /// * `text` - Message text
    ///
    /// # Example
    ///
    /// ```rust,no_run
    /// # use wacloudapi::Client;
    /// # async fn example() -> Result<(), Box<dyn std::error::Error>> {
    /// let client = Client::new("token", "phone_id");
    /// let response = client.messages().send_text("628123456789", "Hello!").await?;
    /// # Ok(())
    /// # }
    /// ```
    pub async fn send_text(&self, to: &str, text: &str) -> Result<MessageResponse> {
        let body = SendTextRequest {
            messaging_product: "whatsapp".to_string(),
            recipient_type: "individual".to_string(),
            to: to.to_string(),
            message_type: "text".to_string(),
            text: TextContent {
                preview_url: false,
                body: text.to_string(),
            },
        };

        let url = format!("{}/messages", self.client.base_url());
        self.client.post(&url, &body).await
    }

    /// Send a text message with URL preview
    pub async fn send_text_with_preview(&self, to: &str, text: &str) -> Result<MessageResponse> {
        let body = SendTextRequest {
            messaging_product: "whatsapp".to_string(),
            recipient_type: "individual".to_string(),
            to: to.to_string(),
            message_type: "text".to_string(),
            text: TextContent {
                preview_url: true,
                body: text.to_string(),
            },
        };

        let url = format!("{}/messages", self.client.base_url());
        self.client.post(&url, &body).await
    }

    /// Send a reply to a message
    pub async fn send_reply(
        &self,
        to: &str,
        text: &str,
        message_id: &str,
    ) -> Result<MessageResponse> {
        let body = SendReplyRequest {
            messaging_product: "whatsapp".to_string(),
            recipient_type: "individual".to_string(),
            to: to.to_string(),
            context: Context {
                message_id: message_id.to_string(),
            },
            message_type: "text".to_string(),
            text: TextContent {
                preview_url: false,
                body: text.to_string(),
            },
        };

        let url = format!("{}/messages", self.client.base_url());
        self.client.post(&url, &body).await
    }

    /// Send a reaction to a message
    pub async fn send_reaction(
        &self,
        to: &str,
        message_id: &str,
        emoji: &str,
    ) -> Result<MessageResponse> {
        let body = SendReactionRequest {
            messaging_product: "whatsapp".to_string(),
            recipient_type: "individual".to_string(),
            to: to.to_string(),
            message_type: "reaction".to_string(),
            reaction: Reaction {
                message_id: message_id.to_string(),
                emoji: emoji.to_string(),
            },
        };

        let url = format!("{}/messages", self.client.base_url());
        self.client.post(&url, &body).await
    }

    /// Remove a reaction from a message (send empty emoji)
    pub async fn remove_reaction(&self, to: &str, message_id: &str) -> Result<MessageResponse> {
        self.send_reaction(to, message_id, "").await
    }

    /// Send an image by URL
    pub async fn send_image_url(
        &self,
        to: &str,
        url: &str,
        caption: Option<&str>,
    ) -> Result<MessageResponse> {
        let body = SendMediaRequest {
            messaging_product: "whatsapp".to_string(),
            recipient_type: "individual".to_string(),
            to: to.to_string(),
            message_type: "image".to_string(),
            image: Some(MediaContent {
                id: None,
                link: Some(url.to_string()),
                caption: caption.map(|s| s.to_string()),
                filename: None,
            }),
            video: None,
            audio: None,
            document: None,
            sticker: None,
        };

        let api_url = format!("{}/messages", self.client.base_url());
        self.client.post(&api_url, &body).await
    }

    /// Send an image by media ID
    pub async fn send_image_id(
        &self,
        to: &str,
        media_id: &str,
        caption: Option<&str>,
    ) -> Result<MessageResponse> {
        let body = SendMediaRequest {
            messaging_product: "whatsapp".to_string(),
            recipient_type: "individual".to_string(),
            to: to.to_string(),
            message_type: "image".to_string(),
            image: Some(MediaContent {
                id: Some(media_id.to_string()),
                link: None,
                caption: caption.map(|s| s.to_string()),
                filename: None,
            }),
            video: None,
            audio: None,
            document: None,
            sticker: None,
        };

        let url = format!("{}/messages", self.client.base_url());
        self.client.post(&url, &body).await
    }

    /// Send a video by URL
    pub async fn send_video_url(
        &self,
        to: &str,
        url: &str,
        caption: Option<&str>,
    ) -> Result<MessageResponse> {
        let body = SendMediaRequest {
            messaging_product: "whatsapp".to_string(),
            recipient_type: "individual".to_string(),
            to: to.to_string(),
            message_type: "video".to_string(),
            image: None,
            video: Some(MediaContent {
                id: None,
                link: Some(url.to_string()),
                caption: caption.map(|s| s.to_string()),
                filename: None,
            }),
            audio: None,
            document: None,
            sticker: None,
        };

        let api_url = format!("{}/messages", self.client.base_url());
        self.client.post(&api_url, &body).await
    }

    /// Send a video by media ID
    pub async fn send_video_id(
        &self,
        to: &str,
        media_id: &str,
        caption: Option<&str>,
    ) -> Result<MessageResponse> {
        let body = SendMediaRequest {
            messaging_product: "whatsapp".to_string(),
            recipient_type: "individual".to_string(),
            to: to.to_string(),
            message_type: "video".to_string(),
            image: None,
            video: Some(MediaContent {
                id: Some(media_id.to_string()),
                link: None,
                caption: caption.map(|s| s.to_string()),
                filename: None,
            }),
            audio: None,
            document: None,
            sticker: None,
        };

        let url = format!("{}/messages", self.client.base_url());
        self.client.post(&url, &body).await
    }

    /// Send an audio file by URL
    pub async fn send_audio_url(&self, to: &str, url: &str) -> Result<MessageResponse> {
        let body = SendMediaRequest {
            messaging_product: "whatsapp".to_string(),
            recipient_type: "individual".to_string(),
            to: to.to_string(),
            message_type: "audio".to_string(),
            image: None,
            video: None,
            audio: Some(MediaContent {
                id: None,
                link: Some(url.to_string()),
                caption: None,
                filename: None,
            }),
            document: None,
            sticker: None,
        };

        let api_url = format!("{}/messages", self.client.base_url());
        self.client.post(&api_url, &body).await
    }

    /// Send an audio file by media ID
    pub async fn send_audio_id(&self, to: &str, media_id: &str) -> Result<MessageResponse> {
        let body = SendMediaRequest {
            messaging_product: "whatsapp".to_string(),
            recipient_type: "individual".to_string(),
            to: to.to_string(),
            message_type: "audio".to_string(),
            image: None,
            video: None,
            audio: Some(MediaContent {
                id: Some(media_id.to_string()),
                link: None,
                caption: None,
                filename: None,
            }),
            document: None,
            sticker: None,
        };

        let url = format!("{}/messages", self.client.base_url());
        self.client.post(&url, &body).await
    }

    /// Send a document by URL
    pub async fn send_document_url(
        &self,
        to: &str,
        url: &str,
        filename: Option<&str>,
        caption: Option<&str>,
    ) -> Result<MessageResponse> {
        let body = SendMediaRequest {
            messaging_product: "whatsapp".to_string(),
            recipient_type: "individual".to_string(),
            to: to.to_string(),
            message_type: "document".to_string(),
            image: None,
            video: None,
            audio: None,
            document: Some(MediaContent {
                id: None,
                link: Some(url.to_string()),
                caption: caption.map(|s| s.to_string()),
                filename: filename.map(|s| s.to_string()),
            }),
            sticker: None,
        };

        let api_url = format!("{}/messages", self.client.base_url());
        self.client.post(&api_url, &body).await
    }

    /// Send a document by media ID
    pub async fn send_document_id(
        &self,
        to: &str,
        media_id: &str,
        filename: Option<&str>,
        caption: Option<&str>,
    ) -> Result<MessageResponse> {
        let body = SendMediaRequest {
            messaging_product: "whatsapp".to_string(),
            recipient_type: "individual".to_string(),
            to: to.to_string(),
            message_type: "document".to_string(),
            image: None,
            video: None,
            audio: None,
            document: Some(MediaContent {
                id: Some(media_id.to_string()),
                link: None,
                caption: caption.map(|s| s.to_string()),
                filename: filename.map(|s| s.to_string()),
            }),
            sticker: None,
        };

        let url = format!("{}/messages", self.client.base_url());
        self.client.post(&url, &body).await
    }

    /// Send a sticker by URL
    pub async fn send_sticker_url(&self, to: &str, url: &str) -> Result<MessageResponse> {
        let body = SendMediaRequest {
            messaging_product: "whatsapp".to_string(),
            recipient_type: "individual".to_string(),
            to: to.to_string(),
            message_type: "sticker".to_string(),
            image: None,
            video: None,
            audio: None,
            document: None,
            sticker: Some(MediaContent {
                id: None,
                link: Some(url.to_string()),
                caption: None,
                filename: None,
            }),
        };

        let api_url = format!("{}/messages", self.client.base_url());
        self.client.post(&api_url, &body).await
    }

    /// Send a sticker by media ID
    pub async fn send_sticker_id(&self, to: &str, media_id: &str) -> Result<MessageResponse> {
        let body = SendMediaRequest {
            messaging_product: "whatsapp".to_string(),
            recipient_type: "individual".to_string(),
            to: to.to_string(),
            message_type: "sticker".to_string(),
            image: None,
            video: None,
            audio: None,
            document: None,
            sticker: Some(MediaContent {
                id: Some(media_id.to_string()),
                link: None,
                caption: None,
                filename: None,
            }),
        };

        let url = format!("{}/messages", self.client.base_url());
        self.client.post(&url, &body).await
    }

    /// Send a location message
    pub async fn send_location(
        &self,
        to: &str,
        latitude: f64,
        longitude: f64,
        name: Option<&str>,
        address: Option<&str>,
    ) -> Result<MessageResponse> {
        let body = SendLocationRequest {
            messaging_product: "whatsapp".to_string(),
            recipient_type: "individual".to_string(),
            to: to.to_string(),
            message_type: "location".to_string(),
            location: Location {
                latitude,
                longitude,
                name: name.map(|s| s.to_string()),
                address: address.map(|s| s.to_string()),
            },
        };

        let url = format!("{}/messages", self.client.base_url());
        self.client.post(&url, &body).await
    }

    /// Send a contact message
    pub async fn send_contacts(&self, to: &str, contacts: Vec<Contact>) -> Result<MessageResponse> {
        let body = SendContactsRequest {
            messaging_product: "whatsapp".to_string(),
            recipient_type: "individual".to_string(),
            to: to.to_string(),
            message_type: "contacts".to_string(),
            contacts,
        };

        let url = format!("{}/messages", self.client.base_url());
        self.client.post(&url, &body).await
    }

    /// Send a template message
    pub async fn send_template(
        &self,
        to: &str,
        template_name: &str,
        language_code: &str,
        components: Option<Vec<TemplateComponent>>,
    ) -> Result<MessageResponse> {
        let body = SendTemplateRequest {
            messaging_product: "whatsapp".to_string(),
            recipient_type: "individual".to_string(),
            to: to.to_string(),
            message_type: "template".to_string(),
            template: Template {
                name: template_name.to_string(),
                language: Language {
                    code: language_code.to_string(),
                },
                components,
            },
        };

        let url = format!("{}/messages", self.client.base_url());
        self.client.post(&url, &body).await
    }

    /// Send an interactive list message
    pub async fn send_list(
        &self,
        to: &str,
        header: Option<&str>,
        body_text: &str,
        footer: Option<&str>,
        button_text: &str,
        sections: Vec<ListSection>,
    ) -> Result<MessageResponse> {
        let body = SendInteractiveRequest {
            messaging_product: "whatsapp".to_string(),
            recipient_type: "individual".to_string(),
            to: to.to_string(),
            message_type: "interactive".to_string(),
            interactive: Interactive {
                interactive_type: "list".to_string(),
                header: header.map(|h| InteractiveHeader {
                    header_type: "text".to_string(),
                    text: Some(h.to_string()),
                    image: None,
                    video: None,
                    document: None,
                }),
                body: InteractiveBody {
                    text: body_text.to_string(),
                },
                footer: footer.map(|f| InteractiveFooter {
                    text: f.to_string(),
                }),
                action: InteractiveAction {
                    button: Some(button_text.to_string()),
                    buttons: None,
                    sections: Some(sections),
                    catalog_id: None,
                    product_retailer_id: None,
                },
            },
        };

        let url = format!("{}/messages", self.client.base_url());
        self.client.post(&url, &body).await
    }

    /// Send an interactive button message
    pub async fn send_buttons(
        &self,
        to: &str,
        header: Option<&str>,
        body_text: &str,
        footer: Option<&str>,
        buttons: Vec<Button>,
    ) -> Result<MessageResponse> {
        let body = SendInteractiveRequest {
            messaging_product: "whatsapp".to_string(),
            recipient_type: "individual".to_string(),
            to: to.to_string(),
            message_type: "interactive".to_string(),
            interactive: Interactive {
                interactive_type: "button".to_string(),
                header: header.map(|h| InteractiveHeader {
                    header_type: "text".to_string(),
                    text: Some(h.to_string()),
                    image: None,
                    video: None,
                    document: None,
                }),
                body: InteractiveBody {
                    text: body_text.to_string(),
                },
                footer: footer.map(|f| InteractiveFooter {
                    text: f.to_string(),
                }),
                action: InteractiveAction {
                    button: None,
                    buttons: Some(buttons),
                    sections: None,
                    catalog_id: None,
                    product_retailer_id: None,
                },
            },
        };

        let url = format!("{}/messages", self.client.base_url());
        self.client.post(&url, &body).await
    }

    /// Mark a message as read
    pub async fn mark_as_read(&self, message_id: &str) -> Result<crate::types::SuccessResponse> {
        let body = MarkReadRequest {
            messaging_product: "whatsapp".to_string(),
            status: "read".to_string(),
            message_id: message_id.to_string(),
        };

        let url = format!("{}/messages", self.client.base_url());
        self.client.post(&url, &body).await
    }
}

// Request/Response types

#[derive(Debug, Serialize)]
struct SendTextRequest {
    messaging_product: String,
    recipient_type: String,
    to: String,
    #[serde(rename = "type")]
    message_type: String,
    text: TextContent,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TextContent {
    pub preview_url: bool,
    pub body: String,
}

#[derive(Debug, Serialize)]
struct SendReplyRequest {
    messaging_product: String,
    recipient_type: String,
    to: String,
    context: Context,
    #[serde(rename = "type")]
    message_type: String,
    text: TextContent,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Context {
    pub message_id: String,
}

#[derive(Debug, Serialize)]
struct SendReactionRequest {
    messaging_product: String,
    recipient_type: String,
    to: String,
    #[serde(rename = "type")]
    message_type: String,
    reaction: Reaction,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Reaction {
    pub message_id: String,
    pub emoji: String,
}

#[derive(Debug, Serialize)]
struct SendMediaRequest {
    messaging_product: String,
    recipient_type: String,
    to: String,
    #[serde(rename = "type")]
    message_type: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    image: Option<MediaContent>,
    #[serde(skip_serializing_if = "Option::is_none")]
    video: Option<MediaContent>,
    #[serde(skip_serializing_if = "Option::is_none")]
    audio: Option<MediaContent>,
    #[serde(skip_serializing_if = "Option::is_none")]
    document: Option<MediaContent>,
    #[serde(skip_serializing_if = "Option::is_none")]
    sticker: Option<MediaContent>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MediaContent {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub link: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub caption: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub filename: Option<String>,
}

#[derive(Debug, Serialize)]
struct SendLocationRequest {
    messaging_product: String,
    recipient_type: String,
    to: String,
    #[serde(rename = "type")]
    message_type: String,
    location: Location,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Location {
    pub latitude: f64,
    pub longitude: f64,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub address: Option<String>,
}

#[derive(Debug, Serialize)]
struct SendContactsRequest {
    messaging_product: String,
    recipient_type: String,
    to: String,
    #[serde(rename = "type")]
    message_type: String,
    contacts: Vec<Contact>,
}

/// Contact information for sending contact messages
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Contact {
    /// Contact name
    pub name: ContactName,
    /// Phone numbers
    #[serde(skip_serializing_if = "Option::is_none")]
    pub phones: Option<Vec<ContactPhone>>,
    /// Email addresses
    #[serde(skip_serializing_if = "Option::is_none")]
    pub emails: Option<Vec<ContactEmail>>,
    /// URLs
    #[serde(skip_serializing_if = "Option::is_none")]
    pub urls: Option<Vec<ContactUrl>>,
    /// Addresses
    #[serde(skip_serializing_if = "Option::is_none")]
    pub addresses: Option<Vec<ContactAddress>>,
    /// Organization info
    #[serde(skip_serializing_if = "Option::is_none")]
    pub org: Option<ContactOrg>,
    /// Birthday (YYYY-MM-DD)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub birthday: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ContactName {
    pub formatted_name: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub first_name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub middle_name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub suffix: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub prefix: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ContactPhone {
    pub phone: String,
    #[serde(rename = "type", skip_serializing_if = "Option::is_none")]
    pub phone_type: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub wa_id: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ContactEmail {
    pub email: String,
    #[serde(rename = "type", skip_serializing_if = "Option::is_none")]
    pub email_type: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ContactUrl {
    pub url: String,
    #[serde(rename = "type", skip_serializing_if = "Option::is_none")]
    pub url_type: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ContactAddress {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub street: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub city: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub state: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub zip: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub country: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub country_code: Option<String>,
    #[serde(rename = "type", skip_serializing_if = "Option::is_none")]
    pub address_type: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ContactOrg {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub company: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub department: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub title: Option<String>,
}

#[derive(Debug, Serialize)]
struct SendTemplateRequest {
    messaging_product: String,
    recipient_type: String,
    to: String,
    #[serde(rename = "type")]
    message_type: String,
    template: Template,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Template {
    pub name: String,
    pub language: Language,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub components: Option<Vec<TemplateComponent>>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Language {
    pub code: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TemplateComponent {
    #[serde(rename = "type")]
    pub component_type: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sub_type: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub index: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parameters: Option<Vec<TemplateParameter>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TemplateParameter {
    #[serde(rename = "type")]
    pub param_type: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub text: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub currency: Option<Currency>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub date_time: Option<DateTime>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub image: Option<MediaContent>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub document: Option<MediaContent>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub video: Option<MediaContent>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Currency {
    pub fallback_value: String,
    pub code: String,
    pub amount_1000: i64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DateTime {
    pub fallback_value: String,
}

#[derive(Debug, Serialize)]
struct SendInteractiveRequest {
    messaging_product: String,
    recipient_type: String,
    to: String,
    #[serde(rename = "type")]
    message_type: String,
    interactive: Interactive,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Interactive {
    #[serde(rename = "type")]
    pub interactive_type: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub header: Option<InteractiveHeader>,
    pub body: InteractiveBody,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub footer: Option<InteractiveFooter>,
    pub action: InteractiveAction,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct InteractiveHeader {
    #[serde(rename = "type")]
    pub header_type: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub text: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub image: Option<MediaContent>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub video: Option<MediaContent>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub document: Option<MediaContent>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct InteractiveBody {
    pub text: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct InteractiveFooter {
    pub text: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct InteractiveAction {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub button: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub buttons: Option<Vec<Button>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sections: Option<Vec<ListSection>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub catalog_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub product_retailer_id: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Button {
    #[serde(rename = "type")]
    pub button_type: String,
    pub reply: ButtonReply,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ButtonReply {
    pub id: String,
    pub title: String,
}

impl Button {
    /// Create a new reply button
    pub fn reply(id: impl Into<String>, title: impl Into<String>) -> Self {
        Self {
            button_type: "reply".to_string(),
            reply: ButtonReply {
                id: id.into(),
                title: title.into(),
            },
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ListSection {
    pub title: String,
    pub rows: Vec<ListRow>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ListRow {
    pub id: String,
    pub title: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
}

impl ListRow {
    /// Create a new list row
    pub fn new(id: impl Into<String>, title: impl Into<String>) -> Self {
        Self {
            id: id.into(),
            title: title.into(),
            description: None,
        }
    }

    /// Add description to the row
    pub fn with_description(mut self, description: impl Into<String>) -> Self {
        self.description = Some(description.into());
        self
    }
}

#[derive(Debug, Serialize)]
struct MarkReadRequest {
    messaging_product: String,
    status: String,
    message_id: String,
}
