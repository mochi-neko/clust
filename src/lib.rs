//! An unofficial Rust client for [the Anthropic/Claude API](https://docs.anthropic.com/claude/reference/getting-started-with-the-api).
//!
//! ## Supported APIs
//! - [Messages](`crate::messages`)
//!     - [x] [Create a Message](https://docs.anthropic.com/claude/reference/messages_post)
//!     - [ ] [Streaming Messages](https://docs.anthropic.com/claude/reference/messages-streaming)
//!
//! ## Usage
//! An example of creating a message with the API key loaded from the environment variable: `ANTHROPIC_API_KEY`
//!
//! ```env
//! ANTHROPIC_API_KEY={your-api-key}
//! ```
//!
//! is as follows:
//!
//! ```rust
//! use clust::messages::ClaudeModel;
//! use clust::messages::MaxTokens;
//! use clust::messages::Message;
//! use clust::messages::MessagesRequestBody;
//! use clust::messages::SystemPrompt;
//! use clust::Client;
//!
//! #[tokio::main]
//! async fn main() -> anyhow::Result<()> {
//!     // 1. Create a new API client with the API key loaded from the environment variable: `ANTHROPIC_API_KEY`.
//!     let client = Client::from_env()?;
//!
//!     // 2. Create a request body.
//!     let model = ClaudeModel::Claude3Sonnet20240229;
//!     let messages = vec![Message::user(
//!         "Where is the capital of France?",
//!     )];
//!     let max_tokens = MaxTokens::new(1024, model)?;
//!     let system_prompt = SystemPrompt::new("You are an excellent AI assistant.");
//!     let request_body = MessagesRequestBody {
//!         model,
//!         messages,
//!         max_tokens,
//!         system: Some(system_prompt),
//!         ..Default::default()
//!     };
//!
//!     // 3. Call the API.
//!     let response = client
//!         .create_a_message(request_body)
//!         .await?;
//!
//!     println!("Result:\n{}", response);
//!
//!     Ok(())
//! }
//! ```

mod api_key;
mod client;
mod error;
mod result;
mod version;

pub(crate) mod macros;

pub mod messages;

pub use api_key::ApiKey;
pub use client::Client;
pub use error::ApiError;
pub use error::ApiErrorBody;
pub use error::ApiErrorResponse;
pub use error::ApiErrorType;
pub use error::ClientError;
pub use error::ValidationError;
pub use result::ValidationResult;
pub use version::Version;

pub use reqwest;
