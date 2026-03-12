use std::path::Path;

use reqwest::Client;
use serde_json::{json, Value};

use crate::error::AgentGenError;
use crate::types::{
    BalanceResponse, CreateOriginResponse, GenerateImageRequest, GenerateImageResponse,
    GeneratePdfRequest, GeneratePdfResponse, UploadOriginPublicKeyResponse, UploadTempResponse,
};

const DEFAULT_BASE_URL: &str = "https://www.agent-gen.com/api";

/// Async AgentGen API client.
///
/// # Example
/// ```no_run
/// use agentgen::{AgentGenClient, types::{GenerateImageRequest, ImageFormat}};
///
/// #[tokio::main]
/// async fn main() -> Result<(), Box<dyn std::error::Error>> {
///     let client = AgentGenClient::new("agk_...");
///
///     let result = client
///         .generate_image(
///             GenerateImageRequest::new("<h1>Hello</h1>")
///                 .viewport_width(1200)
///                 .format(ImageFormat::Png),
///         )
///         .await?;
///
///     println!("{}", result.url);
///     Ok(())
/// }
/// ```
pub struct AgentGenClient {
    api_key: String,
    base_url: String,
    http: Client,
}

impl AgentGenClient {
    /// Create a client using the production API base URL.
    pub fn new(api_key: impl Into<String>) -> Self {
        Self::with_base_url(api_key, DEFAULT_BASE_URL)
    }

    /// Create a client with a custom base URL (useful for testing).
    pub fn with_base_url(api_key: impl Into<String>, base_url: impl Into<String>) -> Self {
        Self {
            api_key: api_key.into(),
            base_url: base_url.into().trim_end_matches('/').to_string(),
            http: Client::new(),
        }
    }

    async fn handle_response<T>(&self, response: reqwest::Response) -> Result<T, AgentGenError>
    where
        T: serde::de::DeserializeOwned,
    {
        let status = response.status();
        if status.is_success() {
            return Ok(response.json::<T>().await?);
        }

        let data: Value = response.json().await?;
        let message = data["error"]
            .as_str()
            .unwrap_or("Unknown error")
            .to_string();

        if status.as_u16() == 402 {
            return Err(AgentGenError::InsufficientTokens {
                balance: data["balance"].as_i64().unwrap_or(0),
                required: data["required"].as_i64().unwrap_or(0),
                buy_more_url: data["buy_more_url"].as_str().unwrap_or("").to_string(),
            });
        }

        Err(AgentGenError::Api {
            status: status.as_u16(),
            message,
            detail: data["detail"].as_str().map(String::from),
        })
    }

    /// Render HTML to an image (PNG / JPEG / WebP).
    /// Costs **1 token**.
    pub async fn generate_image(
        &self,
        request: GenerateImageRequest,
    ) -> Result<GenerateImageResponse, AgentGenError> {
        let response = self
            .http
            .post(format!("{}/v1/generate/image", self.base_url))
            .header("X-API-Key", &self.api_key)
            .json(&request)
            .send()
            .await?;
        self.handle_response(response).await
    }

    /// Render HTML to a PDF — single page or multi-page.
    /// Costs **2 tokens per page**.
    pub async fn generate_pdf(
        &self,
        request: GeneratePdfRequest,
    ) -> Result<GeneratePdfResponse, AgentGenError> {
        self.generate_pdf_with_optimize(request, None).await
    }

    /// Render HTML to a PDF and explicitly control post-processing optimization.
    /// Costs **2 tokens per page**.
    pub async fn generate_pdf_with_optimize(
        &self,
        request: GeneratePdfRequest,
        optimize: Option<bool>,
    ) -> Result<GeneratePdfResponse, AgentGenError> {
        let body = match request {
            GeneratePdfRequest::SinglePage(page) => {
                let mut value = serde_json::to_value(page)?;
                if let Some(optimize) = optimize {
                    value["optimize"] = json!(optimize);
                }
                value
            }
            GeneratePdfRequest::MultiPage { pages } => {
                let mut value = json!({ "pages": pages });
                if let Some(optimize) = optimize {
                    value["optimize"] = json!(optimize);
                }
                value
            }
        };

        let response = self
            .http
            .post(format!("{}/v1/generate/pdf", self.base_url))
            .header("X-API-Key", &self.api_key)
            .json(&body)
            .send()
            .await?;
        self.handle_response(response).await
    }

    /// Upload a file for use inside HTML templates.
    /// **Free** (no tokens). Auto-deleted after **24 hours**.
    pub async fn upload_temp(&self, file_path: &Path) -> Result<UploadTempResponse, AgentGenError> {
        let filename = file_path
            .file_name()
            .and_then(|n| n.to_str())
            .unwrap_or("upload")
            .to_string();

        let bytes = tokio::fs::read(file_path).await?;
        let part = reqwest::multipart::Part::bytes(bytes).file_name(filename);
        let form = reqwest::multipart::Form::new().part("file", part);

        let response = self
            .http
            .post(format!("{}/v1/upload/temp", self.base_url))
            .header("X-API-Key", &self.api_key)
            .multipart(form)
            .send()
            .await?;
        self.handle_response(response).await
    }

    /// Get the current token balance for the API key owner.
    pub async fn get_balance(&self) -> Result<BalanceResponse, AgentGenError> {
        let response = self
            .http
            .get(format!("{}/v1/balance", self.base_url))
            .header("X-API-Key", &self.api_key)
            .send()
            .await?;
        self.handle_response(response).await
    }

    /// Provision a new public origin subdomain (`<id>.agent-gen.com`).
    ///
    /// Use this to get a stable origin URL for third-party integrations that
    /// require a specific allowed origin (e.g. Tesla virtual key setup).
    pub async fn create_origin(&self) -> Result<CreateOriginResponse, AgentGenError> {
        let response = self
            .http
            .post(format!("{}/v1/origin", self.base_url))
            .header("X-API-Key", &self.api_key)
            .send()
            .await?;
        self.handle_response(response).await
    }

    /// Upload an EC public key (PEM format) to an origin subdomain.
    ///
    /// The key is stored at the standard Tesla virtual key path:
    /// `/.well-known/appspecific/com.tesla.3p.public-key.pem`.
    ///
    /// # Arguments
    /// * `origin_id` — The origin ID returned by [`create_origin`](Self::create_origin).
    /// * `pem` — Raw PEM text (must start with `-----BEGIN`).
    pub async fn upload_origin_public_key(
        &self,
        origin_id: &str,
        pem: &str,
    ) -> Result<UploadOriginPublicKeyResponse, AgentGenError> {
        let response = self
            .http
            .post(format!(
                "{}/v1/origin/{}/public-key",
                self.base_url, origin_id
            ))
            .header("X-API-Key", &self.api_key)
            .header("Content-Type", "text/plain")
            .body(pem.to_string())
            .send()
            .await?;
        self.handle_response(response).await
    }
}
