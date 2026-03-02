use std::path::Path;

use reqwest::Client;
use serde_json::Value;

use crate::error::AgentGenError;
use crate::types::{
    BalanceResponse, GenerateImageRequest, GenerateImageResponse, GeneratePdfRequest,
    GeneratePdfResponse, UploadTempResponse,
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
///                 .width(1200)
///                 .height(630)
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
                buy_more_url: data["buy_more_url"]
                    .as_str()
                    .unwrap_or("")
                    .to_string(),
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
        let response = self
            .http
            .post(format!("{}/v1/generate/pdf", self.base_url))
            .header("X-API-Key", &self.api_key)
            .json(&request)
            .send()
            .await?;
        self.handle_response(response).await
    }

    /// Upload a file for use inside HTML templates.
    /// **Free** (no tokens). Auto-deleted after **24 hours**.
    pub async fn upload_temp(
        &self,
        file_path: &Path,
    ) -> Result<UploadTempResponse, AgentGenError> {
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
}
