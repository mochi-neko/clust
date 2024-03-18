//! This example demonstrates how to use the `streaming_messages` API with `tokio` backend.
//!
//! ```shell
//! $ cargo run --example streaming_messages_tokio -- -p <prompt> -m <message>
//! ```
//!
//! e.g.
//! ```shell
//! $ cargo run --example streaming_messages_tokio -- -p "You are a excellent AI assistant." -m "Where is the capital of Japan?"
//! ```

use clust::messages::Message;
use clust::messages::MessagesRequestBody;
use clust::messages::SystemPrompt;
use clust::messages::{ClaudeModel, StreamOption};
use clust::messages::{MaxTokens, StreamChunk};
use clust::Client;

use clap::Parser;
use tokio_stream::StreamExt;

#[derive(Parser)]
struct Arguments {
    #[arg(short, long)]
    prompt: String,
    #[arg(short, long)]
    message: String,
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    // 0. Parse the command-line arguments.
    let arguments = Arguments::parse();

    // 1. Create a new API client with the API key loaded from the environment variable: `ANTHROPIC_API_KEY`.
    let client = Client::from_env()?;

    // 2. Create a request body with stream option.
    let model = ClaudeModel::Claude3Sonnet20240229;
    let messages = vec![Message::user(
        arguments.message,
    )];
    let max_tokens = MaxTokens::new(1024, model)?;
    let system_prompt = SystemPrompt::new(arguments.prompt);
    let request_body = MessagesRequestBody {
        model,
        messages,
        max_tokens,
        system: Some(system_prompt),
        stream: Some(StreamOption::ReturnStream),
        ..Default::default()
    };

    // 3. Call the API.
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
                        buffer.push_str(&content_block_delta.delta.text);
                    },
                    | _ => {},
                }
            },
            | Err(error) => {
                eprintln!("Chunk error:\n{:?}", error);
            },
        }
    }

    println!("Result:\n{}", buffer);

    Ok(())
}
