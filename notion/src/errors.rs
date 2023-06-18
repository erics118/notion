use serde::{Deserialize, Serialize};
use thiserror::Error;

#[derive(Error, Debug, Copy, Clone)]
pub enum Error {
    #[error("Invalid Notion API Token")]
    InvalidApiToken,
    #[error("Client Build Error")]
    ClientBuild,
}

#[derive(Serialize, Deserialize, Debug, Eq, PartialEq, Clone)]
pub struct ErrorInfo {
    pub status: u16,
    pub code: String,
    pub message: String,
}

#[derive(Error, Debug, Clone)]
pub enum NotionApiError {
    #[error("Invalid json (400): {0}")]
    InvalidJson(String),

    #[error("Invalid request url (400): {0}")]
    InvalidRequestUrl(String),

    #[error("Invalid request (400): {0}")]
    InvalidRequest(String),

    #[error("Validation error (400): {0}")]
    ValidationError(String),

    #[error("Missing version (400): {0}")]
    MissingVersion(String),

    #[error("Unauthorized (401): {0}")]
    Unauthorized(String),

    #[error("Restricted resource (403): {0}")]
    RestrictedResource(String),

    #[error("Object not found (404): {0}")]
    ObjectNotFound(String),

    #[error("Conflict error (409): {0}")]
    ConflictError(String),

    #[error("Rate limited (429): {0}")]
    RateLimited(String),

    #[error("Internal server error (500): {0}")]
    InternalServerError(String),

    #[error("Service unavailable (503): {0}")]
    ServiceUnavailable(String),

    #[error("Database connection unavailable (503): {0}")]
    DatabaseConnectionUnavailable(String),

    #[error("Gateway timeout (504): {0}")]
    GatewayTimeout(String),

    #[error("Unknown error")]
    Unknown,
}

impl From<ErrorInfo> for NotionApiError {
    fn from(value: ErrorInfo) -> Self {
        match value {
            ErrorInfo {
                status: 400,
                message,
                code,
            } => match code.as_str() {
                "invalid_json" => Self::InvalidJson(message),
                "invalid_request_url" => Self::InvalidRequestUrl(message),
                "invalid_request" => Self::InvalidRequest(message),
                "validation_error" => Self::ValidationError(message),
                "missing_version" => Self::MissingVersion(message),
                _ => Self::Unknown,
            },
            ErrorInfo {
                status: 401,
                message,
                ..
            } => Self::Unauthorized(message),
            ErrorInfo {
                status: 403,
                message,
                ..
            } => Self::RestrictedResource(message),
            ErrorInfo {
                status: 404,
                message,
                ..
            } => Self::ObjectNotFound(message),
            ErrorInfo {
                status: 409,
                message,
                ..
            } => Self::ConflictError(message),
            ErrorInfo {
                status: 429,
                message,
                ..
            } => Self::RateLimited(message),
            ErrorInfo {
                status: 500,
                message,
                ..
            } => Self::InternalServerError(message),
            ErrorInfo {
                status: 503,
                message,
                code,
            } => match code.as_str() {
                "service_unavailable" => Self::ServiceUnavailable(message),
                "database_connection_unavailable" => Self::DatabaseConnectionUnavailable(message),
                _ => Self::Unknown,
            },
            ErrorInfo {
                status: 504,
                message,
                ..
            } => Self::GatewayTimeout(message),
            _ => Self::Unknown,
        }
    }
}
