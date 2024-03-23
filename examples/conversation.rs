//! This example demonstrates how to converse with the assistant.
//!
//! ```shell
//! $ cargo run --example conversation -- -p <prompt> -f <first> -s <second>
//! ```
//!
//! e.g.
//! ```shell
//! $ cargo run --example conversation -- -p "You are a excellent AI assistant." -f "Where is the capital of Japan?" -s "What is the population of the city?"
//! ```

use clust::messages::ClaudeModel;
use clust::messages::MaxTokens;
use clust::messages::Message;
use clust::messages::MessagesRequestBody;
use clust::messages::SystemPrompt;
use clust::Client;

use clap::Parser;

#[derive(Parser)]
struct Arguments {
    #[arg(short, long)]
    prompt: String,
    #[arg(short, long)]
    first: String,
    #[arg(short, long)]
    second: String,
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    // 0. Parse the command-line arguments.
    let arguments = Arguments::parse();

    // 1. Create a new API client with the API key loaded from the environment variable: `ANTHROPIC_API_KEY`
    let client = Client::from_env()?;
    // or specify the API key directly
    // let client = Client::from_api_key(clust::ApiKey::new("your-api-key"));

    // 2. Create a request body with the first message.
    let model = ClaudeModel::Claude3Haiku20240307;
    let max_tokens = MaxTokens::new(1024, model)?;
    let prompt = SystemPrompt::new(arguments.prompt);
    let messages = vec![Message::user(
        arguments.first,
    )];
    let mut request_body = MessagesRequestBody {
        model,
        messages,
        max_tokens,
        system: Some(prompt),
        ..Default::default()
    };

    // 3. Call the API.
    let response = client
        .create_a_message(request_body.clone())
        .await?;

    println!("First result:\n{}", response);

    // 4. Use the first text content.
    println!("First content: {}", response.text()?);

    // 5. Store the assistant message of the first conversation.
    request_body
        .messages
        .push(response.crate_message());

    // 6. Add the second message.
    request_body
        .messages
        .push(Message::user(arguments.second));

    // 7. Re-call the API.
    let response = client
        .create_a_message(request_body.clone())
        .await?;

    println!("Second result:\n{}", response);

    // 8. Use the second text content.
    println!("Second content: {}", response.text()?);

    // 9. Store the assistant message of the second conversation.
    request_body
        .messages
        .push(response.crate_message());

    // Continue the conversation...

    Ok(())
}
