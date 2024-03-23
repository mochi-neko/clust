//! This example demonstrates how to use the `create_a_message` API with vision.
//!
//! ```shell
//! $ cargo run --example create_a_message_with_vision -- -p <prompt> -i <image-path>
//! ```
//!
//! e.g.
//! ```shell
//! $ cargo run --example create_a_message_with_vision -- -p "What animal is in this image?" -i "path/to/image.png"
//! ```

use base64::Engine;
use clust::messages::MaxTokens;
use clust::messages::Message;
use clust::messages::MessagesRequestBody;
use clust::messages::SystemPrompt;
use clust::messages::{ClaudeModel, ImageContentSource, ImageMediaType};
use clust::Client;
use std::path::PathBuf;

use clap::Parser;

#[derive(Parser)]
struct Arguments {
    #[arg(short, long)]
    prompt: String,
    #[arg(short, long)]
    image_path: String,
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    // 0. Parse the command-line arguments.
    let arguments = Arguments::parse();

    // 1. Create a new API client with the API key loaded from the environment variable: `ANTHROPIC_API_KEY`
    let client = Client::from_env()?;
    // or specify the API key directly
    // let client = Client::from_api_key(clust::ApiKey::new("your-api-key"));

    // 2. Read image file and encode it to Base64.
    let image_file = tokio::fs::read(&arguments.image_path).await?;
    let image_base64 = base64::prelude::BASE64_STANDARD.encode(&image_file);
    let image_source = ImageContentSource::new(
        ImageMediaType::from_path(PathBuf::from(&arguments.image_path))?,
        image_base64,
    );

    // 3. Create a request body.
    let model = ClaudeModel::Claude3Sonnet20240229;
    let messages = vec![Message::user(
        image_source,
    )];
    let max_tokens = MaxTokens::new(1024, model)?;
    let system_prompt = SystemPrompt::new(arguments.prompt);
    let request_body = MessagesRequestBody {
        model,
        messages,
        max_tokens,
        system: Some(system_prompt),
        ..Default::default()
    };

    // 4. Call the API.
    let response = client
        .create_a_message(request_body)
        .await?;

    println!("Result:\n{}", response);

    // 5. Use the text content.
    println!("Content: {}", response.text()?);

    Ok(())
}
