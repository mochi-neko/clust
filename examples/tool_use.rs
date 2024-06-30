//! This example demonstrates how to use the tool use.
//!
//! ```shell
//! $ cargo run --example tool_use --features macros
//! ```

#![cfg(feature = "macros")]

use clust::messages::ClaudeModel;
use clust::messages::MaxTokens;
use clust::messages::Message;
use clust::messages::MessagesRequestBody;
use clust::messages::ToolList;
use clust::Client;

use clust_macros::clust_tool;

/// Get the current weather in a given location
///
/// ## Arguments
/// - `location` - The city and state, e.g. San Francisco, CA
#[clust_tool]
fn get_weather(location: String) -> String {
    "15 degrees".to_string() // Dummy response
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    // 1. Create a new API client with the API key loaded from the environment variable: `ANTHROPIC_API_KEY`
    let client = Client::from_env()?;
    // or specify the API key directly
    // let client = Client::from_api_key(clust::ApiKey::new("your-api-key"));

    // 2. Create a tool instance and tool list.
    let tool_increment = ClustTool_get_weather {};
    let tools = ToolList::new(vec![Box::new(
        tool_increment,
    )]);

    // 3. Create a request body.
    let model = ClaudeModel::Claude35Sonnet20240620;
    let messages = vec![Message::user(
        "What is the weather like in San Francisco?",
    )];
    let max_tokens = MaxTokens::new(1024, model)?;
    let request_body = MessagesRequestBody {
        model,
        messages,
        max_tokens,
        tools: Some(tools.definitions()), // Specify tool definitions
        ..Default::default()
    };

    // 3. Call the API.
    let response = client
        .create_a_message(request_body)
        .await?;

    println!("Result:\n{}", response);

    assert_eq!(
        response.stop_reason,
        Some(clust::messages::StopReason::ToolUse)
    );

    // 4. Get the tool use.
    let tool_use = response
        .content
        .flatten_into_tool_use()?;

    println!("Tool use: {}", tool_use);

    // 5. Call the tool.
    let tool_result = tools.call(tool_use)?;

    println!("Tool result: {}", tool_result);

    Ok(())
}
