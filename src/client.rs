//! HTTP client for the WhatsApp Cloud API

use crate::analytics::AnalyticsApi;
use crate::block::BlockApi;
use crate::error::{ApiErrorResponse, Error, Result};
use crate::flows::FlowsApi;
use crate::media::MediaApi;
use crate::messages::MessagesApi;
use crate::phone_numbers::PhoneNumbersApi;
use crate::products::ProductsApi;
use crate::qr_codes::QrCodesApi;
use crate::templates::TemplatesApi;
use crate::types::{DEFAULT_API_VERSION, GRAPH_API_URL};
use crate::typing::TypingApi;
use crate::waba::WabaApi;
use crate::webhooks_management::WebhookSubscriptionsApi;
use reqwest::header::{HeaderMap, HeaderValue, AUTHORIZATION, CONTENT_TYPE};
use serde::de::DeserializeOwned;
use serde::Serialize;
use std::sync::Arc;

/// WhatsApp Cloud API client
#[derive(Clone)]
pub struct Client {
    inner: Arc<ClientInner>,
}

struct ClientInner {
    http: reqwest::Client,
    access_token: String,
    phone_number_id: String,
    api_version: String,
    base_url: String,
}

impl Client {
    /// Create a new client with the given access token and phone number ID
    ///
    /// # Arguments
    ///
    /// * `access_token` - Your Facebook/Meta access token
    /// * `phone_number_id` - Your WhatsApp Business phone number ID
    ///
    /// # Example
    ///
    /// ```rust
    /// use whatsapp_cloud_api::Client;
    ///
    /// let client = Client::new("your_access_token", "your_phone_number_id");
    /// ```
    pub fn new(access_token: impl Into<String>, phone_number_id: impl Into<String>) -> Self {
        Self::with_config(access_token, phone_number_id, DEFAULT_API_VERSION, GRAPH_API_URL)
    }

    /// Create a new client with custom API version
    pub fn with_version(
        access_token: impl Into<String>,
        phone_number_id: impl Into<String>,
        api_version: impl Into<String>,
    ) -> Self {
        Self::with_config(access_token, phone_number_id, api_version, GRAPH_API_URL)
    }

    /// Create a new client with full configuration
    pub fn with_config(
        access_token: impl Into<String>,
        phone_number_id: impl Into<String>,
        api_version: impl Into<String>,
        base_url: impl Into<String>,
    ) -> Self {
        let http = reqwest::Client::builder()
            .timeout(std::time::Duration::from_secs(30))
            .build()
            .expect("Failed to create HTTP client");

        Self {
            inner: Arc::new(ClientInner {
                http,
                access_token: access_token.into(),
                phone_number_id: phone_number_id.into(),
                api_version: api_version.into(),
                base_url: base_url.into(),
            }),
        }
    }

    /// Get the phone number ID
    pub fn phone_number_id(&self) -> &str {
        &self.inner.phone_number_id
    }

    /// Get the API version
    pub fn api_version(&self) -> &str {
        &self.inner.api_version
    }

    /// Get the base URL for API calls
    pub fn base_url(&self) -> String {
        format!(
            "{}/{}/{}",
            self.inner.base_url, self.inner.api_version, self.inner.phone_number_id
        )
    }

    /// Get the base URL for a specific endpoint
    pub fn endpoint_url(&self, endpoint: &str) -> String {
        format!(
            "{}/{}/{}",
            self.inner.base_url, self.inner.api_version, endpoint
        )
    }

    /// Get default headers for requests
    fn default_headers(&self) -> HeaderMap {
        let mut headers = HeaderMap::new();
        headers.insert(
            AUTHORIZATION,
            HeaderValue::from_str(&format!("Bearer {}", self.inner.access_token))
                .expect("Invalid access token"),
        );
        headers.insert(
            CONTENT_TYPE,
            HeaderValue::from_static("application/json"),
        );
        headers
    }

    /// Make a GET request
    pub(crate) async fn get<T: DeserializeOwned>(&self, url: &str) -> Result<T> {
        let response = self
            .inner
            .http
            .get(url)
            .headers(self.default_headers())
            .send()
            .await?;

        self.handle_response(response).await
    }

    /// Make a POST request with JSON body
    pub(crate) async fn post<T: DeserializeOwned, B: Serialize>(
        &self,
        url: &str,
        body: &B,
    ) -> Result<T> {
        let response = self
            .inner
            .http
            .post(url)
            .headers(self.default_headers())
            .json(body)
            .send()
            .await?;

        self.handle_response(response).await
    }

    /// Make a POST request with form data
    pub(crate) async fn post_form<T: DeserializeOwned>(
        &self,
        url: &str,
        form: reqwest::multipart::Form,
    ) -> Result<T> {
        let mut headers = HeaderMap::new();
        headers.insert(
            AUTHORIZATION,
            HeaderValue::from_str(&format!("Bearer {}", self.inner.access_token))
                .expect("Invalid access token"),
        );

        let response = self
            .inner
            .http
            .post(url)
            .headers(headers)
            .multipart(form)
            .send()
            .await?;

        self.handle_response(response).await
    }

    /// Make a DELETE request
    pub(crate) async fn delete<T: DeserializeOwned>(&self, url: &str) -> Result<T> {
        let response = self
            .inner
            .http
            .delete(url)
            .headers(self.default_headers())
            .send()
            .await?;

        self.handle_response(response).await
    }

    /// Handle API response
    async fn handle_response<T: DeserializeOwned>(
        &self,
        response: reqwest::Response,
    ) -> Result<T> {
        let status = response.status();
        let body = response.text().await?;

        if status.is_success() {
            serde_json::from_str(&body).map_err(Error::from)
        } else {
            // Try to parse error response
            match serde_json::from_str::<ApiErrorResponse>(&body) {
                Ok(error_response) => Err(error_response.into()),
                Err(_) => Err(Error::Api {
                    code: status.as_u16() as i32,
                    message: body,
                    error_subcode: None,
                    error_data: None,
                }),
            }
        }
    }

    /// Get the raw HTTP client (for advanced use)
    pub fn http_client(&self) -> &reqwest::Client {
        &self.inner.http
    }

    /// Access the Messages API
    pub fn messages(&self) -> MessagesApi {
        MessagesApi::new(self.clone())
    }

    /// Access the Media API
    pub fn media(&self) -> MediaApi {
        MediaApi::new(self.clone())
    }

    /// Access the Phone Numbers API
    pub fn phone_numbers(&self) -> PhoneNumbersApi {
        PhoneNumbersApi::new(self.clone())
    }

    /// Access the Templates API
    pub fn templates(&self) -> TemplatesApi {
        TemplatesApi::new(self.clone())
    }

    /// Access the Products/Catalog API
    pub fn products(&self) -> ProductsApi {
        ProductsApi::new(self.clone())
    }

    /// Access the Flows API
    pub fn flows(&self) -> FlowsApi {
        FlowsApi::new(self.clone())
    }

    /// Access the Typing Indicator API
    pub fn typing(&self) -> TypingApi {
        TypingApi::new(self.clone())
    }

    /// Access the QR Codes API
    pub fn qr_codes(&self) -> QrCodesApi {
        QrCodesApi::new(self.clone())
    }

    /// Access the Block Users API
    pub fn block(&self) -> BlockApi {
        BlockApi::new(self.clone())
    }

    /// Access the Analytics API
    ///
    /// # Arguments
    ///
    /// * `waba_id` - WhatsApp Business Account ID
    pub fn analytics(&self, waba_id: impl Into<String>) -> AnalyticsApi {
        AnalyticsApi::new(self.clone(), waba_id.into())
    }

    /// Access the WABA Management API
    ///
    /// # Arguments
    ///
    /// * `waba_id` - WhatsApp Business Account ID
    pub fn waba(&self, waba_id: impl Into<String>) -> WabaApi {
        WabaApi::new(self.clone(), waba_id.into())
    }

    /// Access the Webhook Subscriptions API
    ///
    /// # Arguments
    ///
    /// * `app_id` - Facebook App ID
    pub fn webhook_subscriptions(&self, app_id: impl Into<String>) -> WebhookSubscriptionsApi {
        WebhookSubscriptionsApi::new(self.clone(), app_id.into())
    }
}

impl std::fmt::Debug for Client {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Client")
            .field("phone_number_id", &self.inner.phone_number_id)
            .field("api_version", &self.inner.api_version)
            .field("base_url", &self.inner.base_url)
            .finish()
    }
}
