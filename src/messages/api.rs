use crate::messages::chunk_stream::ChunkStream;
use crate::messages::{
    ChunkStreamResult, MessagesError, MessagesRequestBody,
    MessagesResponseBody, MessagesResult, StreamOption,
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

pub(crate) async fn create_a_message_stream(
    client: &Client,
    request_body: MessagesRequestBody,
) -> MessagesResult<impl futures_util::Stream<Item = ChunkStreamResult>> {
    // Validate stream option.
    if let None = &request_body.stream {
        return Err(MessagesError::StreamOptionMismatch);
    }
    if let Some(stream) = &request_body.stream {
        if *stream != StreamOption::ReturnStream {
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

    // Ok
    if status_code.is_success() {
        // Create a chunk stream from response bytes stream.
        let byte_stream = response.bytes_stream();
        let chunk_stream = ChunkStream::new(byte_stream);
        Ok(chunk_stream)
    }
    // Error
    else {
        // Read the response text.
        let response_text = response
            .text()
            .await
            .map_err(ClientError::ReadResponseTextFailed)?;

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

#[cfg(feature = "tokio_stream")]
pub(crate) async fn create_a_message_stream_tokio(
    client: &Client,
    request_body: MessagesRequestBody,
) -> MessagesResult<impl tokio_stream::Stream<Item = ChunkStreamResult>> {
    // Validate stream option.
    if let None = &request_body.stream {
        return Err(MessagesError::StreamOptionMismatch);
    }
    if let Some(stream) = &request_body.stream {
        if *stream != StreamOption::ReturnStream {
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

    // Ok
    if status_code.is_success() {
        // Create a chunk stream from response bytes stream.
        let byte_stream = response.bytes_stream();
        let chunk_stream =
            crate::messages::tokio_chunk_stream::TokioChunkStream::new(
                byte_stream,
            );
        Ok(chunk_stream)
    }
    // Error
    else {
        // Read the response text.
        let response_text = response
            .text()
            .await
            .map_err(ClientError::ReadResponseTextFailed)?;

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
