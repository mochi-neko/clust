use futures_core::Stream;
use reqwest::RequestBuilder;

use crate::messages::{
    ChunkStreamResult, MessagesRequestBody, MessagesResponseBody,
    MessagesResult,
};
use crate::{ApiKey, Version};

/// The builder of the API client.
///
/// ## Example
/// ```
/// use clust::ClientBuilder;
/// use clust::ApiKey;
/// use clust::Version;
///
/// let client = ClientBuilder::new(ApiKey::new("api-key"))
///     .set_version(Version::V2023_06_01)
///     .set_client(reqwest::Client::new())
///     .build();
/// ```
#[derive(Clone)]
pub struct ClientBuilder {
    /// The API key.
    api_key: ApiKey,
    /// The API version.
    version: Option<Version>,
    /// Internal HTTP client.
    client: Option<reqwest::Client>,
}

impl ClientBuilder {
    /// Create a new API client builder with the API key.
    pub fn new(api_key: ApiKey) -> Self {
        Self {
            api_key,
            version: None,
            client: None,
        }
    }

    /// Create a new API client builder with the API key loaded from the environment variable: `ANTHROPIC_API_KEY`.
    pub fn from_env() -> Result<Self, std::env::VarError> {
        let api_key = ApiKey::from_env()?;

        Ok(Self::new(api_key))
    }

    /// Set the API version.
    pub fn set_version(
        mut self,
        version: Version,
    ) -> Self {
        self.version = Some(version);
        self
    }

    /// Set the HTTP client.
    pub fn set_client(
        mut self,
        client: reqwest::Client,
    ) -> Self {
        self.client = Some(client);
        self
    }

    /// Build the API client.
    pub fn build(self) -> Client {
        let version = self
            .version
            .unwrap_or_default();
        let client = self
            .client
            .unwrap_or_else(|| reqwest::Client::new());

        Client {
            api_key: self.api_key,
            version,
            client,
        }
    }
}

/// The API client.
#[derive(Clone)]
pub struct Client {
    /// The API key.
    api_key: ApiKey,
    /// The API version.
    version: Version,
    /// Internal HTTP client.
    client: reqwest::Client,
}

impl Client {
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

        Ok(Self {
            api_key,
            version,
            client,
        })
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

        Self {
            api_key,
            version,
            client,
        }
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn builder() {
        let client = ClientBuilder::new(ApiKey::new("api-key")).build();
        assert_eq!(client.api_key.value(), "api-key");
        assert_eq!(client.version, Version::default());

        let client = ClientBuilder::new(ApiKey::new("api-key"))
            .set_version(Version::V2023_01_01)
            .build();
        assert_eq!(client.api_key.value(), "api-key");
        assert_eq!(client.version, Version::V2023_01_01);

        let client = ClientBuilder::new(ApiKey::new("api-key"))
            .set_client(
                reqwest::ClientBuilder::new()
                    .timeout(std::time::Duration::from_secs(10))
                    .build()
                    .unwrap(),
            )
            .build();
        assert_eq!(client.api_key.value(), "api-key");
        assert_eq!(client.version, Version::default());
    }
}
