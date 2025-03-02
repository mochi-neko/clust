//! This example demonstrates how to use the `streaming_messages` API with `futures_util` backend.
//!
//! ```shell
//! $ cargo run --example streaming_messages_tools
//! ```
//!
//! e.g.
//! ```shell
//! $ cargo run --example streaming_messages_tools
//! ```

use clust::messages::ClaudeModel;
use clust::messages::ContentBlockDelta;
use clust::messages::MaxTokens;
use clust::messages::Message;
use clust::messages::MessageChunk;
use clust::messages::MessagesRequestBody;
use clust::messages::StreamOption;
use clust::messages::SystemPrompt;
use clust::messages::ToolDefinition;
use clust::Client;

use clap::Parser;
use futures_util::StreamExt;

#[derive(Parser)]
struct Arguments {
    #[arg(short, long)]
    prompt: String,
    #[arg(short, long)]
    message: String,
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    // 1. Create a new API client with the API key loaded from the environment variable: `ANTHROPIC_API_KEY`
    let client = Client::from_env()?;
    // 2. Create a request body with stream option.
    let model = ClaudeModel::Claude3Sonnet20240229;
    let messages = vec![Message::user("how is the temperature like")];
    let max_tokens = MaxTokens::new(1024, model)?;
    let system_prompt = SystemPrompt::new("use get_weather tool and tell me the current weather");
    let request_body = MessagesRequestBody {
        model,
        messages,
        max_tokens,
        system: Some(system_prompt),
        tools: Some(vec![ToolDefinition {
            name: "get_weather".to_string(),
            description: Some("Gives back weather".to_string()),
            input_schema: serde_json::json!({
                "type": "object",
                "properties": {}
            }),
        }]), // Specify tool definitions
        stream: Some(StreamOption::ReturnStream), // Enable streaming
        ..Default::default()
    };

    // 3. Call the API.
    let mut stream = client.create_a_message_stream(request_body).await?;

    let mut buffer = String::new();

    // 4. Poll the stream.
    // NOTE: The `futures_util::StreamExt` run on the single thread.
    while let Some(chunk) = stream.next().await {
        match chunk {
            Ok(chunk) => {
                println!("Chunk:\n{}", chunk);
                match chunk {
                    MessageChunk::ContentBlockDelta(content_block_delta) => {
                        if let ContentBlockDelta::TextDeltaContentBlock(delta) =  content_block_delta.delta {
                            buffer.push_str(&delta.text);
                        }
                    }
                    _ => {}
                }
            }
            Err(error) => {
                eprintln!("Chunk error:\n{:?}", error);
            }
        }
    }

    // 5. Use the result buffer.
    println!("Result:\n{}", buffer);

    Ok(())
}
