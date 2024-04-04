# clust

An unofficial Rust client
for [the Anthropic/Claude API](https://docs.anthropic.com/claude/reference/getting-started-with-the-api).

## Installation

Run the following Cargo command in your project directory:

```shell
cargo add clust
```

or add the following line to your Cargo.toml:

```toml
[dependencies]
clust = "0.7.0"
```

## Supported APIs

- Messages
    - [x] [Create a Message](https://docs.anthropic.com/claude/reference/messages_post)
    - [x] [Streaming Messages](https://docs.anthropic.com/claude/reference/messages-streaming)

## Feature flags

- `macros`: Enable the `clust_tool` attribute macro for generating `clust::messages::Tool`
  or `clust::messages::AsyncTool` from a Rust function.

## Usages

### API key and client

First you need to create a new API client: `clust::Client` with your Anthropic API key from environment variable: "
ANTHROPIC_API_KEY"

```rust,no_run
use clust::Client;

let client = Client::from_env().unwrap();
```

or specify the API key directly:

```rust,no_run
use clust::Client;
use clust::ApiKey;

let client = Client::from_api_key(ApiKey::new("your-api-key"));
```

If you want to customize the client, you can use builder pattern by `clust::ClientBuilder`:

```rust,no_run
use clust::ClientBuilder;
use clust::ApiKey;
use clust::Version;

let client = ClientBuilder::new(ApiKey::new("your-api-key"))
    .version(Version::V2023_06_01)
    .client(reqwest::ClientBuilder::new().timeout(std::time::Duration::from_secs(10)).build().unwrap())
    .build();
```

### Models and max tokens

You can specify the model by `clust::messages::ClaudeModel`.

```rust,no_run
use clust::messages::ClaudeModel;
use clust::messages::MessagesRequestBody;

let model = ClaudeModel::Claude3Sonnet20240229;

let request_body = MessagesRequestBody {
    model,
    ..Default::default ()
};
```

Because max number of tokens of text generation: `clust::messages::MaxTokens` depends on the model,
you need to create `clust::messages::MaxTokens` with the model.

```rust,no_run
use clust::messages::ClaudeModel;
use clust::messages::MaxTokens;
use clust::messages::MessagesRequestBody;

let model = ClaudeModel::Claude3Sonnet20240229;
let max_tokens = MaxTokens::new(1024, model).unwrap();

let request_body = MessagesRequestBody {
    model,
    max_tokens,
    ..Default::default ()
};
```

### Prompt

You can specify the system prompt by `clust::messages::SystemPrompt` and there is no "system" role in the message.

```rust,no_run
use clust::messages::SystemPrompt;
use clust::messages::MessagesRequestBody;

let system_prompt = SystemPrompt::new("You are an excellent AI assistant.");

let request_body = MessagesRequestBody {
    system: Some(system_prompt),
    ..Default::default ()
};
```

### Messages and contents

Build messages by a vector of `clust::messages::Message`:

```rust,no_run
use clust::messages::Role;
use clust::messages::Content;

/// The message.
pub struct Message {
    /// The role of the message.
    pub role: Role,
    /// The content of the message.
    pub content: Content,
}
```

You can create each role message as follows:

```rust,no_run
use clust::messages::Message;

let message = Message::user("Hello, Claude!");
let message = Message::assistant("Hello, user!");
```

and a content: `clust::messages::Content`.

```rust,no_run
use clust::messages::ContentBlock;

/// The content of the message.
pub enum Content {
    /// The single text content.
    SingleText(String),
    /// The multiple content blocks.
    MultipleBlocks(Vec<ContentBlock>),
}
```

Multiple blocks is a vector of content block: `clust::messages::ContentBlock`:

```rust,no_run
use clust::messages::TextContentBlock;
use clust::messages::ImageContentBlock;

/// The content block of the message.
pub enum ContentBlock {
    /// The text content block.
    Text(TextContentBlock),
    /// The image content block.
    Image(ImageContentBlock),
}
```

You can create a content as follows:

```rust,no_run
use clust::messages::Content;
use clust::messages::ContentBlock;
use clust::messages::TextContentBlock;
use clust::messages::ImageContentBlock;
use clust::messages::ImageContentSource;
use clust::messages::ImageMediaType;

// Single text content
let content = Content::SingleText("Hello, Claude!".to_string());
// or use `From` trait
let content = Content::from("Hello, Claude!");

// Multiple content blocks
let content = Content::MultipleBlocks(vec![
    ContentBlock::Text(TextContentBlock::new("Hello, Claude!")),
    ContentBlock::Image(ImageContentBlock::new(ImageContentSource::base64(
        ImageMediaType::Png,
        "Base64 encoded image data",
    ))),
]);
// or use `From` trait for `String` or `ImageContentSource`
let content = Content::from(vec![
    ContentBlock::from("Hello, Claude!"),
    ContentBlock::from(ImageContentSource::base64(
        ImageMediaType::Png,
        "Base64 encoded image data",
    )),
]);

```

### Request body

The request body is defined by `clust::messages::MessagesRequestBody`.

See also `MessagesRequestBody` for other options.

```rust,no_run
use clust::messages::MessagesRequestBody;
use clust::messages::ClaudeModel;
use clust::messages::Message;
use clust::messages::MaxTokens;
use clust::messages::SystemPrompt;

let request_body = MessagesRequestBody {
    model: ClaudeModel::Claude3Sonnet20240229,
    messages: vec![Message::user("Hello, Claude!")],
    max_tokens: MaxTokens::new(1024, ClaudeModel::Claude3Sonnet20240229).unwrap(),
    system: Some(SystemPrompt::new("You are an excellent AI assistant.")),
    ..Default::default ()
};
```

You can also use the builder pattern with `clust::messages::MessagesRequestBuilder`:

```rust,no_run
use clust::messages::MessagesRequestBuilder;
use clust::messages::ClaudeModel;
use clust::messages::Message;
use clust::messages::SystemPrompt;

let request_body = MessagesRequestBuilder::new_with_max_tokens(
    ClaudeModel::Claude3Sonnet20240229,
    1024,
).unwrap()
.messages(vec![Message::user("Hello, Claude!")])
.system(SystemPrompt::new("You are an excellent AI assistant."))
.build();
```

### API calling

Call the API by `clust::Client::create_a_message` with the request body.

```rust,no_run
use clust::Client;
use clust::messages::MessagesRequestBody;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let client = Client::from_env()?;
    let request_body = MessagesRequestBody::default();

    // Call the async API.
    let response = client
        .create_a_message(request_body)
        .await?;

    // You can extract the text content from `clust::messages::MessagesResponseBody.content.flatten_into_text()`.
    println!("Content: {}", response.content.flatten_into_text()?);

    Ok(())
}
```

### Streaming

When you want to stream the response incrementally,
you can use `clust::Client::create_a_message_stream` with the stream option: `StreamOption::ReturnStream`.

```rust,no_run
use clust::Client;
use clust::messages::MessagesRequestBody;
use clust::messages::StreamOption;

use tokio_stream::StreamExt;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let client = Client::from_env()?;
    let request_body = MessagesRequestBody {
        stream: Some(StreamOption::ReturnStream),
        ..Default::default()
    };

    // Call the async API and get the stream.
    let mut stream = client
        .create_a_message_stream(request_body)
        .await?;

    // Poll the stream.
    while let Some(chunk) = stream.next().await {
         // Handle the chunk.
    }

    Ok(())
}
```

### Function calling

Function calling is not formally supported
as [the official guide](https://docs.anthropic.com/claude/docs/functions-external-tools).

This crate provides two method to use function calling:

1. Add the `clust::clust_macros::clust_tool` attribute macro to your Rust function and use
   generated `clust::messages::Tool` or `clust::messages::AsyncTool` with the `macros` feature flag.
2. Manually create `clust::messages::ToolDescription`, `clust::messages::FunctionCalls`,
   and `clust::messages::FunctionResults`.

## Examples

### Create a message

An example of creating a message with the API key loaded from the environment variable: `ANTHROPIC_API_KEY`

```env
ANTHROPIC_API_KEY={your-api-key}
```

is as follows:

```rust,no_run
use clust::messages::ClaudeModel;
use clust::messages::MaxTokens;
use clust::messages::Message;
use clust::messages::MessagesRequestBody;
use clust::messages::SystemPrompt;
use clust::Client;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    // 1. Create a new API client with the API key loaded from the environment variable: `ANTHROPIC_API_KEY`.
    let client = Client::from_env()?;

    // 2. Create a request body.
    let model = ClaudeModel::Claude3Sonnet20240229;
    let messages = vec![Message::user(
        "Where is the capital of France?",
    )];
    let max_tokens = MaxTokens::new(1024, model)?;
    let system_prompt = SystemPrompt::new("You are an excellent AI assistant.");
    let request_body = MessagesRequestBody {
        model,
        messages,
        max_tokens,
        system: Some(system_prompt),
        ..Default::default()
    };

    // 3. Call the API.
    let response = client
        .create_a_message(request_body)
        .await?;

    println!("Result:\n{}", response);

    Ok(())
}
```

### Streaming messages with `tokio` backend

An example of creating a message stream with the API key loaded from the environment variable: `ANTHROPIC_API_KEY`

```env
ANTHROPIC_API_KEY={your-api-key}
```

with [tokio-stream](https://docs.rs/tokio-stream/latest/tokio_stream/) is as follows:

```rust,no_run
use clust::messages::ClaudeModel;
use clust::messages::MaxTokens;
use clust::messages::Message;
use clust::messages::MessagesRequestBody;
use clust::messages::SystemPrompt;
use clust::messages::StreamOption;
use clust::messages::StreamChunk;
use clust::Client;

use tokio_stream::StreamExt;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    // 1. Create a new API client with the API key loaded from the environment variable: `ANTHROPIC_API_KEY`.
    let client = Client::from_env()?;

    // 2. Create a request body with `stream` option.
    let model = ClaudeModel::Claude3Sonnet20240229;
    let messages = vec![Message::user(
        "Where is the capital of France?",
    )];
    let max_tokens = MaxTokens::new(1024, model)?;
    let system_prompt = SystemPrompt::new("You are an excellent AI assistant.");
    let request_body = MessagesRequestBody {
        model,
        messages,
        max_tokens,
        system: Some(system_prompt),
        stream: Some(StreamOption::ReturnStream),
        ..Default::default()
    };

    // 3. Call the streaming API.
    let mut stream = client
        .create_a_message_stream(request_body)
        .await?;

    let mut buffer = String::new();

    // 4. Poll the stream.
    // NOTE: The `tokio_stream::StreamExt` run on the `tokio` runtime.
    while let Some(chunk) = stream.next().await {
        match chunk {
            | Ok(chunk) => {
                println!("Chunk:\n{}", chunk);
                match chunk {
                    | StreamChunk::ContentBlockDelta(content_block_delta) => {
                        // Buffer message delta.
                        buffer.push_str(&content_block_delta.delta.text);
                    }
                    | _ => {}
                }
            }
            | Err(error) => {
                eprintln!("Chunk error:\n{:?}", error);
            }
        }
    }

    println!("Result:\n{}", buffer);

    Ok(())
}
```

### Create a message with vision

See [an example with vision](./examples/create_a_message_with_vision.rs).

### Conversation

See [a conversation example](./examples/conversation.rs).

### Function calling

See [a function_calling example](./examples/function_calling.rs).

### Other examples

See also the [examples](./examples) directory for more examples.

## Changelog

See [CHANGELOG](./CHANGELOG.md).

## License

Licensed under either of the [Apache License, Version 2.0](./LICENSE-APACHE) or the [MIT](./LICENSE-MIT) license at your
option.
