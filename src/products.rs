//! Products and Catalog Messages API

use crate::client::Client;
use crate::error::Result;
use crate::types::MessageResponse;
use serde::{Deserialize, Serialize};

/// Products API client for catalog and product messages
pub struct ProductsApi {
    client: Client,
}

impl ProductsApi {
    pub(crate) fn new(client: Client) -> Self {
        Self { client }
    }

    /// Send a single product message
    ///
    /// # Arguments
    ///
    /// * `to` - Recipient's phone number
    /// * `catalog_id` - The catalog ID
    /// * `product_retailer_id` - The product's retailer ID
    /// * `body_text` - Message body text
    /// * `footer` - Optional footer text
    pub async fn send_product(
        &self,
        to: &str,
        catalog_id: &str,
        product_retailer_id: &str,
        body_text: &str,
        footer: Option<&str>,
    ) -> Result<MessageResponse> {
        let body = SendProductRequest {
            messaging_product: "whatsapp".to_string(),
            recipient_type: "individual".to_string(),
            to: to.to_string(),
            message_type: "interactive".to_string(),
            interactive: ProductInteractive {
                interactive_type: "product".to_string(),
                body: ProductBody {
                    text: body_text.to_string(),
                },
                footer: footer.map(|f| ProductFooter {
                    text: f.to_string(),
                }),
                action: ProductAction {
                    catalog_id: catalog_id.to_string(),
                    product_retailer_id: Some(product_retailer_id.to_string()),
                    sections: None,
                },
            },
        };

        let url = format!("{}/messages", self.client.base_url());
        self.client.post(&url, &body).await
    }

    /// Send a multi-product message (product list)
    ///
    /// # Arguments
    ///
    /// * `to` - Recipient's phone number
    /// * `catalog_id` - The catalog ID
    /// * `header_text` - Header text
    /// * `body_text` - Message body text
    /// * `footer` - Optional footer text
    /// * `sections` - Product sections with items
    pub async fn send_product_list(
        &self,
        to: &str,
        catalog_id: &str,
        header_text: &str,
        body_text: &str,
        footer: Option<&str>,
        sections: Vec<ProductSection>,
    ) -> Result<MessageResponse> {
        let body = SendProductListRequest {
            messaging_product: "whatsapp".to_string(),
            recipient_type: "individual".to_string(),
            to: to.to_string(),
            message_type: "interactive".to_string(),
            interactive: ProductListInteractive {
                interactive_type: "product_list".to_string(),
                header: ProductHeader {
                    header_type: "text".to_string(),
                    text: header_text.to_string(),
                },
                body: ProductBody {
                    text: body_text.to_string(),
                },
                footer: footer.map(|f| ProductFooter {
                    text: f.to_string(),
                }),
                action: ProductListAction {
                    catalog_id: catalog_id.to_string(),
                    sections,
                },
            },
        };

        let url = format!("{}/messages", self.client.base_url());
        self.client.post(&url, &body).await
    }

    /// Send a catalog message
    ///
    /// # Arguments
    ///
    /// * `to` - Recipient's phone number
    /// * `body_text` - Message body text
    /// * `footer` - Optional footer text
    /// * `thumbnail_product_retailer_id` - Product ID to use as thumbnail
    pub async fn send_catalog(
        &self,
        to: &str,
        body_text: &str,
        footer: Option<&str>,
        thumbnail_product_retailer_id: Option<&str>,
    ) -> Result<MessageResponse> {
        let body = SendCatalogRequest {
            messaging_product: "whatsapp".to_string(),
            recipient_type: "individual".to_string(),
            to: to.to_string(),
            message_type: "interactive".to_string(),
            interactive: CatalogInteractive {
                interactive_type: "catalog_message".to_string(),
                body: ProductBody {
                    text: body_text.to_string(),
                },
                footer: footer.map(|f| ProductFooter {
                    text: f.to_string(),
                }),
                action: CatalogAction {
                    name: "catalog_message".to_string(),
                    parameters: thumbnail_product_retailer_id.map(|id| CatalogParameters {
                        thumbnail_product_retailer_id: id.to_string(),
                    }),
                },
            },
        };

        let url = format!("{}/messages", self.client.base_url());
        self.client.post(&url, &body).await
    }

    /// Get commerce settings
    pub async fn get_commerce_settings(&self) -> Result<CommerceSettings> {
        let url = format!(
            "{}/whatsapp_commerce_settings",
            self.client.base_url()
        );
        self.client.get(&url).await
    }

    /// Update commerce settings
    pub async fn update_commerce_settings(
        &self,
        is_catalog_visible: bool,
        is_cart_enabled: bool,
    ) -> Result<crate::types::SuccessResponse> {
        let body = UpdateCommerceSettingsRequest {
            is_catalog_visible,
            is_cart_enabled,
        };

        let url = format!(
            "{}/whatsapp_commerce_settings",
            self.client.base_url()
        );
        self.client.post(&url, &body).await
    }
}

// Request types

#[derive(Debug, Serialize)]
struct SendProductRequest {
    messaging_product: String,
    recipient_type: String,
    to: String,
    #[serde(rename = "type")]
    message_type: String,
    interactive: ProductInteractive,
}

#[derive(Debug, Serialize)]
struct ProductInteractive {
    #[serde(rename = "type")]
    interactive_type: String,
    body: ProductBody,
    #[serde(skip_serializing_if = "Option::is_none")]
    footer: Option<ProductFooter>,
    action: ProductAction,
}

#[derive(Debug, Serialize)]
struct ProductAction {
    catalog_id: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    product_retailer_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    sections: Option<Vec<ProductSection>>,
}

#[derive(Debug, Serialize)]
struct SendProductListRequest {
    messaging_product: String,
    recipient_type: String,
    to: String,
    #[serde(rename = "type")]
    message_type: String,
    interactive: ProductListInteractive,
}

#[derive(Debug, Serialize)]
struct ProductListInteractive {
    #[serde(rename = "type")]
    interactive_type: String,
    header: ProductHeader,
    body: ProductBody,
    #[serde(skip_serializing_if = "Option::is_none")]
    footer: Option<ProductFooter>,
    action: ProductListAction,
}

#[derive(Debug, Serialize)]
struct ProductHeader {
    #[serde(rename = "type")]
    header_type: String,
    text: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ProductBody {
    pub text: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ProductFooter {
    pub text: String,
}

#[derive(Debug, Serialize)]
struct ProductListAction {
    catalog_id: String,
    sections: Vec<ProductSection>,
}

/// Product section for multi-product messages
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProductSection {
    /// Section title
    pub title: String,
    /// Products in this section
    pub product_items: Vec<ProductItem>,
}

impl ProductSection {
    /// Create a new product section
    pub fn new(title: impl Into<String>, products: Vec<ProductItem>) -> Self {
        Self {
            title: title.into(),
            product_items: products,
        }
    }
}

/// Product item for catalog messages
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProductItem {
    /// Product retailer ID
    pub product_retailer_id: String,
}

impl ProductItem {
    /// Create a new product item
    pub fn new(product_retailer_id: impl Into<String>) -> Self {
        Self {
            product_retailer_id: product_retailer_id.into(),
        }
    }
}

#[derive(Debug, Serialize)]
struct SendCatalogRequest {
    messaging_product: String,
    recipient_type: String,
    to: String,
    #[serde(rename = "type")]
    message_type: String,
    interactive: CatalogInteractive,
}

#[derive(Debug, Serialize)]
struct CatalogInteractive {
    #[serde(rename = "type")]
    interactive_type: String,
    body: ProductBody,
    #[serde(skip_serializing_if = "Option::is_none")]
    footer: Option<ProductFooter>,
    action: CatalogAction,
}

#[derive(Debug, Serialize)]
struct CatalogAction {
    name: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    parameters: Option<CatalogParameters>,
}

#[derive(Debug, Serialize)]
struct CatalogParameters {
    thumbnail_product_retailer_id: String,
}

#[derive(Debug, Serialize)]
struct UpdateCommerceSettingsRequest {
    is_catalog_visible: bool,
    is_cart_enabled: bool,
}

// Response types

/// Commerce settings response
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CommerceSettings {
    /// Whether catalog is visible
    #[serde(default)]
    pub is_catalog_visible: bool,
    /// Whether cart is enabled
    #[serde(default)]
    pub is_cart_enabled: bool,
    /// Catalog ID
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
}
