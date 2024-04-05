//! An unofficial Rust client for [the Anthropic/Claude API](https://docs.anthropic.com/claude/reference/getting-started-with-the-api).
//!
//! ## Supported APIs
//! - [Messages](`crate::messages`)
//!     - [x] [Create a Message](https://docs.anthropic.com/claude/reference/messages_post)
//!     - [x] [Streaming Messages](https://docs.anthropic.com/claude/reference/messages-streaming)
//!
//! ## Feature flags
//! - `macros`: Enable the [`attributes::clust_tool`] attribute macro for generating [`messages::Tool`]
//!   or [`messages::AsyncTool`] from a Rust function.
//!
//! ## Usages
//!
//! ### API key and client
//! First you need to create a new API client: `clust::Client` with your Anthropic API key from environment variable: "ANTHROPIC_API_KEY"
//!
//! ```rust,no_run
//! use clust::Client;
//!
//! let client = Client::from_env().unwrap();
//! ```
//!
//! or specify the API key directly:
//!
//! ```rust
//! use clust::Client;
//! use clust::ApiKey;
//!
//! let client = Client::from_api_key(ApiKey::new("your-api-key"));
//! ```
//!
//! If you want to customize the client, you can use builder pattern by `clust::ClientBuilder`:
//! ```rust
//! use clust::ClientBuilder;
//! use clust::ApiKey;
//! use clust::Version;
//!
//! let client = ClientBuilder::new(ApiKey::new("your-api-key"))
//!     .version(Version::V2023_06_01)
//!     .client(reqwest::ClientBuilder::new().timeout(std::time::Duration::from_secs(10)).build().unwrap())
//!     .build();
//! ```
//!
//! ### Models and max tokens
//! You can specify the model by `clust::messages::ClaudeModel`.
//!
//! ```rust
//! use clust::messages::ClaudeModel;
//! use clust::messages::MessagesRequestBody;
//!
//! let model = ClaudeModel::Claude3Sonnet20240229;
//!
//! let request_body = MessagesRequestBody {
//!     model,
//!     ..Default::default()
//! };
//! ```
//!
//! Because max number of tokens of text generation: `clust::messages::MaxTokens` depends on the model,
//! you need to create `clust::messages::MaxTokens` with the model.
//!
//! ```rust
//! use clust::messages::ClaudeModel;
//! use clust::messages::MaxTokens;
//! use clust::messages::MessagesRequestBody;
//!
//! let model = ClaudeModel::Claude3Sonnet20240229;
//! let max_tokens = MaxTokens::new(1024, model).unwrap();
//!
//! let request_body = MessagesRequestBody {
//!     model,
//!     max_tokens,
//!     ..Default::default()
//! };
//! ```
//!
//! ### Prompt
//! You can specify the system prompt by `clust::messages::SystemPrompt` and there is no "system" role in the message.
//!
//! ```rust
//! use clust::messages::SystemPrompt;
//! use clust::messages::MessagesRequestBody;
//!
//! let system_prompt = SystemPrompt::new("You are an excellent AI assistant.");
//!
//! let request_body = MessagesRequestBody {
//!     system: Some(system_prompt),
//!     ..Default::default()
//! };
//! ```
//!
//! ### Messages and contents
//! Build messages by a vector of `clust::messages::Message`:
//!
//! ```rust
//! use clust::messages::Role;
//! use clust::messages::Content;
//!
//! /// The message.
//! pub struct Message {
//!     /// The role of the message.
//!     pub role: Role,
//!     /// The content of the message.
//!     pub content: Content,
//! }
//! ```
//!
//! You can create each role message as follows:
//!
//! ```rust
//! use clust::messages::Message;
//!
//! let message = Message::user("Hello, Claude!");
//! let message = Message::assistant("Hello, user!");
//! ```
//!
//! and a content: `clust::messages::Content`.
//!
//! ```rust
//! use clust::messages::ContentBlock;
//!
//! /// The content of the message.
//! pub enum Content {
//!     /// The single text content.
//!     SingleText(String),
//!     /// The multiple content blocks.
//!     MultipleBlocks(Vec<ContentBlock>),
//! }
//! ```
//!
//! Multiple blocks is a vector of content block: `clust::messages::ContentBlock`:
//!
//! ```rust
//! use clust::messages::TextContentBlock;
//! use clust::messages::ImageContentBlock;
//!
//! /// The content block of the message.
//! pub enum ContentBlock {
//!     /// The text content block.
//!     Text(TextContentBlock),
//!     /// The image content block.
//!     Image(ImageContentBlock),
//! }
//! ```
//!
//! You can create a content as follows:
//!
//! ```rust
//! use clust::messages::Content;
//! use clust::messages::ContentBlock;
//! use clust::messages::TextContentBlock;
//! use clust::messages::ImageContentBlock;
//! use clust::messages::ImageContentSource;
//! use clust::messages::ImageMediaType;
//!
//! // Single text content
//! let content = Content::SingleText("Hello, Claude!".to_string());
//! // or use `From` trait
//! let content = Content::from("Hello, Claude!");
//!
//! // Multiple content blocks
//! let content = Content::MultipleBlocks(vec![
//!     ContentBlock::Text(TextContentBlock::new("Hello, Claude!")),
//!     ContentBlock::Image(ImageContentBlock::new(ImageContentSource::base64(
//!          ImageMediaType::Png,
//!          "Base64 encoded image data",
//!     ))),
//! ]);
//! // or use `From` trait for `String` or `ImageContentSource`
//! let content = Content::from(vec![
//!     ContentBlock::from("Hello, Claude!"),
//!     ContentBlock::from(ImageContentSource::base64(
//!          ImageMediaType::Png,
//!          "Base64 encoded image data",
//!     )),
//! ]);
//!
//! ```
//!
//! ### Request body
//! The request body is defined by `clust::messages::MessagesRequestBody`.
//!
//! ```rust
//! use clust::messages::MessagesRequestBody;
//! use clust::messages::ClaudeModel;
//! use clust::messages::Message;
//! use clust::messages::MaxTokens;
//! use clust::messages::SystemPrompt;
//!
//! let request_body = MessagesRequestBody {
//!     model: ClaudeModel::Claude3Sonnet20240229,
//!     messages: vec![Message::user("Hello, Claude!")],
//!     max_tokens: MaxTokens::new(1024, ClaudeModel::Claude3Sonnet20240229).unwrap(),
//!     system: Some(SystemPrompt::new("You are an excellent AI assistant.")),
//!     ..Default::default()
//! };
//! ```
//!
//! You can also use the builder pattern with `clust::messages::MessagesRequestBuilder`:
//!
//! ```rust
//! use clust::messages::MessagesRequestBuilder;
//! use clust::messages::ClaudeModel;
//! use clust::messages::Message;
//! use clust::messages::SystemPrompt;
//!
//! let request_body = MessagesRequestBuilder::new_with_max_tokens(
//!     ClaudeModel::Claude3Sonnet20240229,
//!     1024,
//! ).unwrap()
//! .messages(vec![Message::user("Hello, Claude!")])
//! .system(SystemPrompt::new("You are an excellent AI assistant."))
//! .build();
//! ```
//!
//! ### API calling
//! Call the API by `clust::Client::create_a_message` with the request body.
//!
//! ```rust,no_run
//! use clust::Client;
//! use clust::messages::MessagesRequestBody;
//!
//! #[tokio::main]
//! async fn main() -> anyhow::Result<()> {
//!     let client = Client::from_env()?;
//!     let request_body = MessagesRequestBody::default();
//!
//!     // Call the async API.
//!     let response = client
//!         .create_a_message(request_body)
//!         .await?;
//!
//!     // You can extract the text content from `clust::messages::MessagesResponseBody.content.flatten_into_text()`.
//!     println!("Content: {}", response.content.flatten_into_text()?);
//!
//!     Ok(())
//! }
//! ```
//!
//! ### Streaming
//! When you want to stream the response incrementally,
//! you can use `clust::Client::create_a_message_stream` with the stream option: `StreamOption::ReturnStream`.
//!
//! ```rust,no_run
//! use clust::Client;
//! use clust::messages::MessagesRequestBody;
//! use clust::messages::StreamOption;
//!
//! use tokio_stream::StreamExt;
//!
//! #[tokio::main]
//! async fn main() -> anyhow::Result<()> {
//!     let client = Client::from_env()?;
//!     let request_body = MessagesRequestBody {
//!         stream: Some(StreamOption::ReturnStream),
//!         ..Default::default()
//!     };
//!
//!     // Call the async API and get the stream.
//!     let mut stream = client
//!         .create_a_message_stream(request_body)
//!         .await?;
//!
//!     // Poll the stream.
//!     while let Some(chunk) = stream.next().await {
//!          // Handle the chunk.
//!     }
//!
//!     Ok(())
//! }
//! ```
//!
//! ### Function calling
//!
//! #### Overview
//!
//! Function calling is not formally supported as [the official guide](https://docs.anthropic.com/claude/docs/functions-external-tools).
//!
//! 1. Define a tool (or tools) by `clust::messages::ToolDescription`.
//! 2. Embed tool description(s) in your system prompt.
//! 3. Call the API with tools.
//! 4. Extract the XML of the function calls by `clust::messages::FunctionCalls` from the response.
//! 5. Call the function with `clust::messages::FunctionCalls` and get the result as `clust::messages::FunctionResults`.
//! 6. Embed the function results in message as a user message.
//! 7. Call the API again with the function results.
//! 8. Continue the conversation.
//!
//! Structures for function calling are (de)serialized as XML string.
//! - `clust::messages::ToolDescription`
//! - `clust::messages::FunctionCalls`
//! - `clust::messages::FunctionResults`
//! - `clust::messages::ToolList`
//! - etc...
//!
//! This crate provides two methods to define function calling:
//!
//! 1. Add the `clust::attributes::clust_tool` attribute macro to your Rust function and use
//!    generated `clust::messages::Tool` or `clust::messages::AsyncTool` with the `macros` feature flag.
//! 2. Manually create `clust::messages::ToolDescription`.
//!
//! #### How to define and call a tool by the attribute macro
//!
//! Let's see how to define and call a tool by the attribute macro.
//!
//! By adding `#[clust_tool]` to your function with documentation for function description and arguments (parameters),
//! a structure `ClustTool_{function_name}` that implements a trait: `clust::messages::Tool` or `clust::messages::AsyncTool`
//! is automatically generated by macro.
//!
//! You can get `clust::messages::ToolDescription` by `ClustTool_{function_name}.description()`
//! and call the function by `ClustTool_{function_name}.call()` method with `clust::messages::FunctionCalls` generated by the assistant.
//!
//! ```rust,no_compile
//! use clust::attributes::clust_tool;
//! use clust::messages::Tool;
//! use clust::messages::FunctionCalls;
//! use clust::messages::Invoke;
//! use std::collections::BTreeMap;
//!
//! // Define a tool by attribute macro for your function.
//! // NOTE: Documentation for description and arguments is required.
//! /// Gets the current stock price for a company. Returns float: The current stock price. Raises ValueError: if the input symbol is invalid/unknown.
//! ///
//! /// ## Arguments
//! /// - `symbol` - The stock symbol of the company to get the price for.
//! #[clust_tool]
//! fn get_current_stock_price(symbol: String) -> f64 {
//!     // Call the API to get the current stock price.
//!     38.50 // NOTE: This is a dummy value.
//! }
//!
//! // Create a tool instance of generated struct.
//! let tool = ClustTool_get_current_stock_price {};
//!
//! // Get the tool description.
//! let tool_description = tool.description();
//!
//! // ToolDescription is displayed as XML string.
//! // e.g.
//! // <tool_description>
//! //   <tool_name>get_current_stock_price</tool_name>
//! //   <description>Gets the current stock price for a company. Returns float: The current stock price. Raises ValueError: if the input symbol is invalid/unknown.</description>
//! //   <parameters>
//! //     <parameter>
//! //       <name>symbol</name>
//! //       <type>String</type>
//! //       <description>The stock symbol of the company to get the price for.</description>
//! //     </parameter>
//! //   </parameters>
//! // </tool_description>
//! let prompt = format!("Your system prompt. {}", tool_description);
//!
//! // NOTE: Actually you can get the function calls from the response.
//! // MessagesResponseBody.content.extract_function_calls().unwrap();
//! // e.g.
//! // <function_calls>
//! //   <invoke>
//! //     <tool_name>get_current_stock_price</tool_name>
//! //     <parameters>
//! //       <symbol>GM</symbol>
//! //     </parameters>
//! //   </invoke>
//! // </function_calls>
//! let function_calls = FunctionCalls {
//!     invoke: Invoke {
//!         tool_name: "get_current_stock_price".to_string(),
//!         parameters: BTreeMap::from_iter(vec![(
//!             "symbol".to_string(),
//!             "GM".to_string(),
//!         )]),
//!     },
//! };
//!
//! // Call the function and get the function results.
//! // e.g.
//! // <function_results>
//! //   <result>
//! //     <tool_name>get_current_stock_price</tool_name>
//! //     <stdout>38.50</stdout>
//! //   </result>
//! // </function_results>
//! let function_results = tool.call(function_calls).unwrap();
//! ```
//!
//! When you have some tools, you can create a tool list by `clust::messages::ToolList`,
//! embed the tool list in your system prompt
//! and call the function with `clust::messages::FunctionCalls` via tool list.
//!
//! For more details, see [a function calling example](./examples/function_calling.rs).
//!
//! ## Examples
//!
//! ### Create a message
//! An example of creating a message with the API key loaded from the environment variable: `ANTHROPIC_API_KEY`
//!
//! ```env
//! ANTHROPIC_API_KEY={your-api-key}
//! ```
//!
//! is as follows:
//!
//! ```rust,no_run
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
//!
//! ### Streaming messages with `tokio` backend
//! An example of creating a message stream with the API key loaded from the environment variable: `ANTHROPIC_API_KEY`
//!
//! ```env
//! ANTHROPIC_API_KEY={your-api-key}
//! ```
//!
//! with [tokio-stream](https://docs.rs/tokio-stream/latest/tokio_stream/) is as follows:
//!
//! ```rust,no_run
//! use clust::messages::ClaudeModel;
//! use clust::messages::MaxTokens;
//! use clust::messages::Message;
//! use clust::messages::MessagesRequestBody;
//! use clust::messages::SystemPrompt;
//! use clust::messages::StreamOption;
//! use clust::messages::MessageChunk;
//! use clust::Client;
//!
//! use tokio_stream::StreamExt;
//!
//! #[tokio::main]
//! async fn main() -> anyhow::Result<()> {
//!     // 1. Create a new API client with the API key loaded from the environment variable: `ANTHROPIC_API_KEY`.
//!     let client = Client::from_env()?;
//!
//!     // 2. Create a request body with `stream` option.
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
//!         stream: Some(StreamOption::ReturnStream),
//!         ..Default::default()
//!     };
//!
//!     // 3. Call the streaming API.
//!     let mut stream = client
//!         .create_a_message_stream(request_body)
//!         .await?;
//!
//!     let mut buffer = String::new();
//!
//!     // 4. Poll the stream.
//!     // NOTE: The `futures_util::StreamExt` run on the single thread.
//!     while let Some(chunk) = stream.next().await {
//!         match chunk {
//!             | Ok(chunk) => {
//!                 println!("Chunk:\n{}", chunk);
//!                 match chunk {
//!                     | MessageChunk::ContentBlockDelta(content_block_delta) => {
//!                         // Buffer message delta.
//!                         buffer.push_str(&content_block_delta.delta.text);
//!                     }
//!                     | _ => {}
//!                 }
//!             }
//!             | Err(error) => {
//!                 eprintln!("Chunk error:\n{:?}", error);
//!             }
//!         }
//!     }
//!
//!     println!("Result:\n{}", buffer);
//!
//!     Ok(())
//! }
//! ```
//!
//! ### Create a message with vision
//!
//! See [an example with vision](./examples/create_a_message_with_vision.rs).
//!
//! ### Conversation
//!
//! See [a conversation example](./examples/conversation.rs).
//!
//! ### Function calling
//!
//! See [a function calling example](./examples/function_calling.rs).
//!
//! ### Other examples
//!
//! See also the [examples](./examples) directory for more examples.

mod api_key;
mod beta;
mod client;
mod error;
mod version;

pub(crate) mod macros;

pub mod messages;

#[cfg(feature = "macros")]
pub mod attributes;

pub use api_key::ApiKey;
pub use beta::Beta;
pub use client::Client;
pub use client::ClientBuilder;
pub use error::ApiError;
pub use error::ApiErrorBody;
pub use error::ApiErrorResponse;
pub use error::ApiErrorType;
pub use error::ClientError;
pub use error::ValidationError;
pub use version::Version;

pub use futures_core;
pub use quick_xml;
pub use reqwest;
pub use serde_json;

#[cfg(feature = "macros")]
pub use clust_macros;
