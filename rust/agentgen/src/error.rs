use thiserror::Error;

/// Errors returned by the AgentGen client.
#[derive(Debug, Error)]
pub enum AgentGenError {
    /// The API returned a non-2xx response.
    #[error("API error ({status}): {message}")]
    Api {
        status: u16,
        message: String,
        detail: Option<String>,
    },

    /// The account has insufficient tokens for the requested operation.
    #[error("Insufficient tokens: have {balance}, need {required}. Buy more at {buy_more_url}")]
    InsufficientTokens {
        balance: i64,
        required: i64,
        buy_more_url: String,
    },

    /// An underlying HTTP transport error.
    #[error("HTTP error: {0}")]
    Http(#[from] reqwest::Error),

    /// An I/O error (e.g. reading a file for upload).
    #[error("I/O error: {0}")]
    Io(#[from] std::io::Error),

    /// A serialization error while building the request body.
    #[error("JSON error: {0}")]
    Json(#[from] serde_json::Error),
}
