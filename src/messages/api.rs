use crate::messages::{
    MessagesError, MessagesRequestBody, MessagesResponseBody, MessagesResult,
    StreamOption,
};
use crate::ApiError;
use crate::Client;
use crate::ClientError;

pub(crate) async fn create_a_message(
    client: &Client,
    request_body: MessagesRequestBody,
) -> MessagesResult<MessagesResponseBody> {
    // Validate stream option.
    if let Some(stream) = &request_body.stream {
        if *stream != StreamOption::ReturnOnce {
            return Err(MessagesError::StreamOptionMismatch);
        }
    }

    // Send the request.
    let response = client
        .post("https://api.anthropic.com/v1/messages")
        .json(&request_body)
        .send()
        .await
        .map_err(ClientError::HttpRequestError)?;

    // Check the response status code.
    let status_code = response.status();

    // Read the response text.
    let response_text = response
        .text()
        .await
        .map_err(ClientError::ReadResponseTextFailed)?;

    // Ok
    if status_code.is_success() {
        // Deserialize the response.
        serde_json::from_str(&response_text).map_err(|error| {
            {
                ClientError::ResponseDeserializationFailed {
                    error,
                    text: response_text,
                }
            }
            .into()
        })
    }
    // Error
    else {
        // Deserialize the error response.
        let error_response =
            serde_json::from_str(&response_text).map_err(|error| {
                ClientError::ErrorResponseDeserializationFailed {
                    error,
                    text: response_text,
                }
            })?;

        Err(ApiError::new(status_code, error_response).into())
    }
}
