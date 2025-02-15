//! This example demonstrates how to use the `create_a_message` API.
//!
//! ```shell
//! $ cargo run --example create_a_message -- -p <prompt> -m <message>
//! ```
//!
//! e.g.
//! ```shell
//! $ cargo run --example create_a_message -- -p "You are a excellent AI assistant." -m "Where is the capital of Japan?"
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
    message: String,
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    #[cfg(feature = "tracing")]
    init_tracing_subscriber();

    // 0. Parse the command-line arguments.
    let arguments = Arguments::parse();

    // 1. Create a new API client with the API key loaded from the environment variable: `ANTHROPIC_API_KEY`
    let client = Client::from_env()?;
    // or specify the API key directly
    // let client = Client::from_api_key(clust::ApiKey::new("your-api-key"));

    // 2. Create a request body.
    let model = ClaudeModel::Claude3Haiku20240307;
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
        ..Default::default()
    };

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

#[cfg(feature = "tracing")]
fn init_tracing_subscriber() {
    use tracing_subscriber::util::SubscriberInitExt;
    let subscriber = tracing_subscriber::fmt()
        .with_env_filter(tracing_subscriber::EnvFilter::from_default_env())
        .finish();
    subscriber.init();
}
