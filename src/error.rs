use crate::macros::impl_display_for_serialize;
use reqwest::StatusCode;
use std::fmt::Display;

/// The error of the validation.
#[derive(Debug, Clone, thiserror::Error)]
pub struct ValidationError<T>
where
    T: Display,
{
    /// The type of the validation target.
    pub _type: String,
    /// The expected value message.
    pub expected: String,
    /// The actual value.
    pub actual: T,
}

impl<T> Display for ValidationError<T>
where
    T: Display,
{
    fn fmt(
        &self,
        f: &mut std::fmt::Formatter<'_>,
    ) -> std::fmt::Result {
        write!(
            f,
            "Validation error: ({}) {}, actual value: {}",
            self._type, self.expected, self.actual,
        )
    }
}

/// The error of the client API calling.
#[derive(Debug, thiserror::Error)]
pub enum ClientError {
    /// HTTP request error of an API calling.
    #[error("HTTP request error: {0:?}")]
    HttpRequestError(reqwest::Error),
    /// Reading response text failed of an API calling.
    #[error("Reading response text failed: {0:?}")]
    ReadResponseTextFailed(reqwest::Error),
    /// Failed to deserialize response of an API calling.
    #[error("Failed to deserialize response as JSON: {error:?}, {text:?}")]
    ResponseDeserializationFailed {
        error: serde_json::Error,
        text: String,
    },
    /// Failed to deserialize response of an API calling.
    #[error(
        "Failed to deserialize error response as JSON: {error:?}, {text:?}"
    )]
    ErrorResponseDeserializationFailed {
        error: serde_json::Error,
        text: String,
    },
}

/// The error of the API server.
#[derive(Debug, Clone, thiserror::Error)]
pub struct ApiError {
    /// The HTTP status code of the response.
    pub status: StatusCode,
    /// The type of the error.
    pub _type: ApiErrorType,
    /// The response body of the error.
    pub response: ApiErrorResponse,
}

impl Display for ApiError {
    fn fmt(
        &self,
        f: &mut std::fmt::Formatter<'_>,
    ) -> std::fmt::Result {
        write!(
            f,
            "API error: ({}) {}: {}",
            self.status, self._type, self.response,
        )
    }
}

impl ApiError {
    /// Creates a new API error.
    pub(crate) fn new(
        status: StatusCode,
        response: ApiErrorResponse,
    ) -> Self {
        let _type = ApiErrorType::from(status);
        Self {
            status,
            _type,
            response,
        }
    }
}

/// The response body of the API error defined at [the errors](https://docs.anthropic.com/claude/reference/errors).
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct ApiErrorResponse {
    /// The type of response. It will be `error`.
    #[serde(rename = "type")]
    pub _type: String,
    /// The error body.
    pub error: ApiErrorBody,
}

impl_display_for_serialize!(ApiErrorResponse);

/// The API error body defined at [the errors](https://docs.anthropic.com/claude/reference/errors).
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct ApiErrorBody {
    /// The type of the error.
    #[serde(rename = "type")]
    pub _type: String,
    /// The error message.
    pub message: String,
}

impl_display_for_serialize!(ApiErrorBody);

/// The API error type defined at [the errors](https://docs.anthropic.com/claude/reference/errors).
#[derive(Debug, Clone, PartialEq)]
pub enum ApiErrorType {
    /// 400 - invalid_request_error: There was an issue with the format or content of your request.
    InvalidRequestError,
    /// 401 - authentication_error: There's an issue with your API key.
    AuthenticationError,
    /// 403 - permission_error: Your API key does not have permission to use the specified resource.
    PermissionError,
    /// 404 - not_found_error: The requested resource was not found.
    NotFoundError,
    /// 429 - rate_limit_error: Your account has hit a rate limit.
    RateLimitError,
    /// 500 - api_error: An unexpected error has occurred internal to Anthropic's systems.
    ApiError,
    /// 529 - overloaded_error: Anthropic's API is temporarily overloaded.
    OverloadedError,
    /// Unknown error type.
    Unknown(StatusCode),
}

impl Display for ApiErrorType {
    fn fmt(
        &self,
        f: &mut std::fmt::Formatter<'_>,
    ) -> std::fmt::Result {
        match self {
            | ApiErrorType::InvalidRequestError => {
                write!(f, "invalid_request_error")
            },
            | ApiErrorType::AuthenticationError => {
                write!(f, "authentication_error")
            },
            | ApiErrorType::PermissionError => {
                write!(f, "permission_error")
            },
            | ApiErrorType::NotFoundError => {
                write!(f, "not_found_error")
            },
            | ApiErrorType::RateLimitError => {
                write!(f, "rate_limit_error")
            },
            | ApiErrorType::ApiError => {
                write!(f, "api_error")
            },
            | ApiErrorType::OverloadedError => {
                write!(f, "overloaded_error")
            },
            | ApiErrorType::Unknown(status) => {
                write!(f, "unknown_error({})", status)
            },
        }
    }
}

impl From<StatusCode> for ApiErrorType {
    fn from(status: StatusCode) -> Self {
        if status == StatusCode::from_u16(529).unwrap() {
            return Self::OverloadedError;
        }

        match status {
            | StatusCode::BAD_REQUEST => Self::InvalidRequestError,
            | StatusCode::UNAUTHORIZED => Self::AuthenticationError,
            | StatusCode::FORBIDDEN => Self::PermissionError,
            | StatusCode::NOT_FOUND => Self::NotFoundError,
            | StatusCode::TOO_MANY_REQUESTS => Self::RateLimitError,
            | StatusCode::INTERNAL_SERVER_ERROR => Self::ApiError,
            | unknown => Self::Unknown(unknown),
        }
    }
}
