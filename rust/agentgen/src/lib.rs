//! # agentgen
//!
//! Async Rust client for the [AgentGen API](https://www.agent-gen.com) —
//! HTML → PDF and HTML → Image for AI agents and developers.
//!
//! ## Quick start
//!
//! ```no_run
//! use agentgen::{AgentGenClient, types::{GenerateImageRequest, ImageFormat}};
//!
//! #[tokio::main]
//! async fn main() -> Result<(), Box<dyn std::error::Error>> {
//!     let client = AgentGenClient::new(std::env::var("AGENTGEN_API_KEY")?);
//!
//!     let img = client
//!         .generate_image(
//!             GenerateImageRequest::new("<h1>Hello, world!</h1>")
//!                 .viewport_width(1200)
//!                 .format(ImageFormat::Png),
//!         )
//!         .await?;
//!
//!     println!("Image URL: {}", img.url);
//!     Ok(())
//! }
//! ```

pub mod client;
pub mod error;
pub mod types;

pub use client::AgentGenClient;
pub use error::AgentGenError;
