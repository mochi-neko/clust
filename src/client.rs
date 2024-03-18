use futures_core::Stream;
use reqwest::RequestBuilder;

use crate::messages::{
    ChunkStreamResult, MessagesRequestBody, MessagesResponseBody,
    MessagesResult,
};
use crate::{ApiKey, Version};

/// The API client.
#[derive(Clone)]
pub struct Client {
    /// The API key.
    api_key: ApiKey,
    /// The API version.
    version: Version,
    /// An HTTP client.
    client: reqwest::Client,
}

impl Client {
    /// Create a new API client.
    ///
    /// ## Arguments
    /// - `api_key` - The API key.
    /// - `version` - The API version.
    /// - `client` - A HTTP client.
    ///
    /// ## Example
    /// ```
    /// use clust::Client;
    ///
    /// let api_key = clust::ApiKey::new("api-key");
    /// let version = clust::Version::V2023_06_01;
    /// let client = reqwest::Client::new();
    ///
    /// let client = Client::new(api_key, version, client);
    /// ```
    pub fn new(
        api_key: ApiKey,
        version: Version,
        client: reqwest::Client,
    ) -> Self {
        Self {
            api_key,
            version,
            client,
        }
    }

    /// Create a new API client with the API key loaded from the environment variable: `ANTHROPIC_API_KEY` and default options.
    ///
    /// ## Example
    /// ```no_run
    /// use clust::Client;
    ///
    /// let client = Client::from_env().unwrap();
    /// ```
    pub fn from_env() -> Result<Self, std::env::VarError> {
        let api_key = ApiKey::from_env()?;
        let version = Version::default();
        let client = reqwest::Client::new();

        Ok(Self::new(api_key, version, client))
    }

    /// Create a new API client with the API key and default options.
    ///
    /// ## Arguments
    /// - `api_key` - The API key.
    ///
    /// ## Example
    /// ```
    /// use clust::Client;
    ///
    /// let api_key = clust::ApiKey::new("api-key");
    ///
    /// let client = Client::from_api_key(api_key);
    /// ```
    pub fn from_api_key(api_key: ApiKey) -> Self {
        let version = Version::default();
        let client = reqwest::Client::new();

        Self::new(api_key, version, client)
    }

    /// Create a request builder for the `POST` method.
    pub(crate) fn post(
        &self,
        endpoint: &str,
    ) -> RequestBuilder {
        self.client
            .post(endpoint)
            .header("x-api-key", self.api_key.value())
            .header(
                "anthropic-version",
                self.version.to_string(),
            )
    }
}

impl Client {
    /// Create a Message.
    ///
    /// Send a structured list of input messages with text and/or image content, and the model will generate the next message in the conversation.
    ///
    /// The Messages API can be used for either single queries or stateless multi-turn conversations.
    ///
    /// See also [Create a Message](https://docs.anthropic.com/claude/reference/messages_post).
    ///
    /// ## Arguments
    /// - `request_body` - The request body.
    ///
    /// ## NOTE
    /// The `stream` option must be `None` or `StreamOption::ReturnOnce`.
    ///
    /// ## Example
    /// ```no_run
    /// use clust::Client;
    /// use clust::messages::{MessagesRequestBody, ClaudeModel, Message, Role, MaxTokens};
    ///
    /// #[tokio::main]
    /// async fn main() -> anyhow::Result<()> {
    ///     let client = Client::from_env()?;
    ///     let model = ClaudeModel::Claude3Sonnet20240229;
    ///     let max_tokens = MaxTokens::new(1024, model)?;
    ///     let request_body = MessagesRequestBody {
    ///         model,
    ///         max_tokens,
    ///         messages: vec![
    ///             Message::user("Hello, Claude!"),
    ///         ],
    ///         ..Default::default()
    ///     };
    ///
    ///     let response = client
    ///         .create_a_message(request_body)
    ///         .await?;
    ///
    ///     Ok(())
    /// }
    /// ```
    pub async fn create_a_message(
        &self,
        request_body: MessagesRequestBody,
    ) -> MessagesResult<MessagesResponseBody> {
        crate::messages::api::create_a_message(self, request_body).await
    }

    /// Create a Message with incrementally streaming the response using server-sent events (SSE).
    ///
    /// See also [Streaming Messages](https://docs.anthropic.com/claude/reference/messages-streaming).
    ///
    /// ## Arguments
    /// - `request_body` - The request body.
    ///
    /// ## NOTE
    /// The `stream` option must be `StreamOption::ReturnStream`.
    ///
    /// ## Example
    /// ```no_run
    /// use clust::Client;
    /// use clust::messages::{MessagesRequestBody, ClaudeModel, Message, Role, MaxTokens, StreamOption};
    /// use tokio_stream::StreamExt; // or futures_util::StreamExt to `stream.next().await`.
    ///
    /// #[tokio::main]
    /// async fn main() -> anyhow::Result<()> {
    ///     let client = Client::from_env()?;
    ///     let model = ClaudeModel::Claude3Sonnet20240229;
    ///     let max_tokens = MaxTokens::new(1024, model)?;
    ///     let request_body = MessagesRequestBody {
    ///         model,
    ///         max_tokens,
    ///         messages: vec![
    ///             Message::user("Hello, Claude!"),
    ///         ],
    ///         stream: Some(StreamOption::ReturnStream),
    ///         ..Default::default()
    ///     };
    ///
    ///     let mut stream = client
    ///         .create_a_message_stream(request_body)
    ///         .await?;
    ///
    ///     while let Some(chunk) = stream.next().await {
    ///         match chunk {
    ///             Ok(chunk) => {
    ///                 // Process the chunk.
    ///             }
    ///             Err(error) => {
    ///                 // Handle the error.
    ///             }
    ///         }
    ///     }
    ///
    ///     Ok(())
    /// }
    /// ```
    pub async fn create_a_message_stream(
        &self,
        request_body: MessagesRequestBody,
    ) -> MessagesResult<impl Stream<Item = ChunkStreamResult>> {
        crate::messages::api::create_a_message_stream(self, request_body).await
    }
}
