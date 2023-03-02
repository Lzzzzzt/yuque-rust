use reqwest::header::InvalidHeaderValue;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum YuqueError {
    #[error("Internal Error: {0}.")]
    Internal(String),
    #[error("Request Error: {0}.")]
    Request(#[from] reqwest::Error),
    #[error("Invalid Params: {0}. The requested parameters are incorrect, or the necessary information is missing, please compare the documentation.")]
    InvalidParams(String),
    #[error("Invalid User Info: {0}. Incorrect user information for the interface requiring user authentication.")]
    InvalidUserInfo(String),
    #[error("No Permission: {0}. Missing permission for the corresponding function.")]
    NoPermission(String),
    #[error("Not Found: Data does not exist, or is not open.")]
    NotFound(String),
    #[error("Server Exception: {0}")]
    ServerException(String),
    #[error("Not Support Format: {0}.")]
    NotSupportFormat(String),
}

impl From<InvalidHeaderValue> for YuqueError {
    fn from(value: InvalidHeaderValue) -> Self {
        Self::Internal(value.to_string())
    }
}

impl From<serde_json::Error> for YuqueError {
    fn from(value: serde_json::Error) -> Self {
        Self::Internal(value.to_string())
    }
}
