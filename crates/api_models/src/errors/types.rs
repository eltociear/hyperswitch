use reqwest::StatusCode;

pub enum ErrorType {
    InvalidRequestError,
    RouterError,
    ConnectorError,
}

#[derive(Debug, serde::Serialize)]
pub struct ApiError {
    pub sub_code: String,
    pub error_identifier: u16,
    pub error_message: String,
}

#[derive(Debug)]
pub enum ApiErrorResponse {
    Unauthorized(ApiError),
    ForbiddenCommonResource(ApiError),
    ForbiddenPrivateResource(ApiError),
    Conflict(ApiError),
    Gone(ApiError),
    Unprocessable(ApiError),
    InternalServerError(ApiError),
    NotImplemented(ApiError),
    ConnectorError(ApiError, StatusCode),
    NotFound(ApiError),
    MethodNotAllowed(ApiError),
    BadRequest(ApiError),
}

impl ::core::fmt::Display for ApiErrorResponse {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            r#"{{"error":{}}}"#,
            serde_json::to_string(self.get_internal_error())
                .unwrap_or_else(|_| "API error response".to_string())
        )
    }
}

impl ApiErrorResponse {
    pub(crate) fn get_internal_error(&self) -> &ApiError {
        match self {
            Self::Unauthorized(i)
            | Self::ForbiddenCommonResource(i)
            | Self::ForbiddenPrivateResource(i)
            | Self::Conflict(i)
            | Self::Gone(i)
            | Self::Unprocessable(i)
            | Self::InternalServerError(i)
            | Self::NotImplemented(i)
            | Self::NotFound(i)
            | Self::MethodNotAllowed(i)
            | Self::BadRequest(i)
            | Self::ConnectorError(i, _) => i,
        }
    }

    pub(crate) fn error_type(&self) -> &str {
        match self {
            Self::Unauthorized(_) => "invalid_request",
            Self::ForbiddenCommonResource(_) => "invalid_request",
            Self::ForbiddenPrivateResource(_) => "invalid_request",
            Self::Conflict(_) => "invalid_request",
            Self::Gone(_) => "invalid_request",
            Self::Unprocessable(_) => "invalid_request",
            Self::InternalServerError(_) => "api",
            Self::NotImplemented(_) => "invalid_request",
            Self::ConnectorError(_, _) => "connector",
            Self::MethodNotAllowed(_) => "invalid_request",
            Self::NotFound(_) => "invalid_request",
            Self::BadRequest(_) => "bad_request",
        }
    }
    // pub fn new() -> Self {

    // }
}
