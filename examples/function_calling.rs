//! This example demonstrates how to use the `create_a_message` API with function calling.
//!
//! ```shell
//! $ cargo run --example function_calling --features macros
//! ```

use clust::clust_macros::clust_tool;
use clust::messages::ClaudeModel;
use clust::messages::MaxTokens;
use clust::messages::Message;
use clust::messages::MessagesRequestBody;
use clust::messages::SystemPrompt;
use clust::messages::ToolList;
use clust::Client;

/// Gets the current stock price for a company. Returns float: The current stock price. Raises ValueError: if the input symbol is invalid/unknown.
///
/// ## Arguments
/// - `symbol` - The stock symbol of the company to get the price for.
#[clust_tool]
fn get_current_stock_price(symbol: String) -> f64 {
    // Call the API to get the current stock price.
    38.50
}

/// Gets the stock ticker symbol for a company searched by name. Returns str: The ticker symbol for the company stock. Raises TickerNotFound: if no matching ticker symbol is found.
///
/// ## Arguments
/// - `company_name` - The name of the company.
#[clust_tool]
fn get_ticker_symbol(company_name: String) -> String {
    // Call the API to get the ticker symbol.
    "GM".to_string()
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    // 0. Create a tool list and system prompt.
    let tool_list = ToolList::new(vec![
        Box::new(ClustTool_get_current_stock_price {}),
        Box::new(ClustTool_get_ticker_symbol {}),
    ]);

    let prompt = format!(
        r#"
In this environment you have access to a set of tools you can use to answer the user's question.

You may call them like this:
<function_calls>
<invoke>
<tool_name>$TOOL_NAME</tool_name>
<parameters>
<$PARAMETER_NAME>$PARAMETER_VALUE</$PARAMETER_NAME>
...
</parameters>
</invoke>
</function_calls>

Here are the tools available:
{}
"#,
        tool_list.tools(),
    );

    println!("{}", prompt);

    // 1. Create a new API client with the API key loaded from the environment variable: `ANTHROPIC_API_KEY`
    let client = Client::from_env()?;
    // or specify the API key directly
    // let client = Client::from_api_key(clust::ApiKey::new("your-api-key"));

    // 2. Create a request body.
    let model = ClaudeModel::Claude3Opus20240229;
    let messages = vec![Message::user(
        "What is the current stock price of General Motors?",
    )];
    let max_tokens = MaxTokens::new(4096, model)?;
    let system_prompt = SystemPrompt::new(prompt);
    let mut request_body = MessagesRequestBody {
        model,
        messages,
        max_tokens,
        system: Some(system_prompt),
        ..Default::default()
    };

    // 3. Call the API.
    let response = client
        .create_a_message(request_body.clone())
        .await?;

    println!("Result:\n{}", response);

    // 4. Exclude the function calls.
    let function_calls = response
        .content
        .exclude_function_calls()?;

    // 5. Call the function.
    let function_result = tool_list.call(function_calls)?;

    // 6. Stack a function call by assistant.
    request_body
        .messages
        .push(response.crate_message());

    // 7. Stack a function result by tool.
    request_body
        .messages
        .push(Message::user(
            function_result.to_string(),
        ));

    // 8. Re-call the API.
    let response = client
        .create_a_message(request_body.clone())
        .await?;

    println!("Result:\n{}", response);

    // 9. Exclude the function calls.
    let function_calls = response
        .content
        .exclude_function_calls()?;

    // 10. Call the function.
    let function_result = tool_list.call(function_calls)?;

    // 11. Stack a function call by assistant.
    request_body
        .messages
        .push(response.crate_message());

    // 12. Stack a function result by tool.
    request_body
        .messages
        .push(Message::user(
            function_result.to_string(),
        ));

    // 13. Re-call the API.
    let response = client
        .create_a_message(request_body.clone())
        .await?;

    println!("Result:\n{}", response);

    Ok(())
}
