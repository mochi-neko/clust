//! This example demonstrates how to use the `create_a_message` API with builder pattern.
//!
//! ```shell
//! $ cargo run --example create_a_message_with_builder -- -p <prompt> -m <message>
//! ```
//!
//! e.g.
//! ```shell
//! $ cargo run --example create_a_message_with_builder -- -p "You are a excellent AI assistant." -m "Where is the capital of Japan?"
//! ```

use std::time::Duration;

use clap::Parser;

use clust::messages::ClaudeModel;
use clust::messages::Message;
use clust::messages::MessagesRequestBuilder;
use clust::messages::SystemPrompt;
use clust::ApiKey;
use clust::ClientBuilder;

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

    // 1. Create a new API client with builder pattern.
    let client = ClientBuilder::new(ApiKey::from_env()?)
        .version(clust::Version::V2023_06_01) // Custom API version
        .client(
            reqwest::ClientBuilder::new()
                .timeout(Duration::from_secs(30))
                .build()?,
        ) // Custom reqwest client
        .build();

    // 2. Create a request body with builder pattern.
    let request_body = MessagesRequestBuilder::new_with_max_tokens(
        ClaudeModel::Claude3Haiku20240307,
        1024,
    )?
    .messages(vec![Message::user(
        arguments.message,
    )])
    .system(SystemPrompt::new(arguments.prompt))
    .build();

    // 3. Call the API.
    let response = client
        .create_a_message(request_body)
        .await?;

    println!("Result:\n{}", response);

    // 4. Use the text content.
    println!(
        "Content: {}",
        response
            .content
            .flatten_into_text()?
    );

    Ok(())
}
