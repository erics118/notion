use thiserror::Error;

#[derive(Error, Debug, Copy, Clone)]
pub enum Error {
    #[error("Invalid Notion API Token")]
    InvalidApiToken,
    #[error("Client Build Error")]
    ClientBuild,
}

#[derive(Error, Debug, Clone)]
pub enum NotionApiError {
    #[error("Invalid JSON: {0}")]
    InvalidJson(String),

    #[error("Invalid request URL: {0}")]
    InvalidRequestUrl(String),

    #[error("Invalid request: {0}")]
    InvalidRequest(String),

    #[error("Validation error: {0}")]
    ValidationError(String),

    #[error("Missing Notion-Version header")]
    MissingVersion,

    #[error("Unauthorized")]
    Unauthorized,

    #[error("Restricted resource")]
    RestrictedResource,

    #[error("Object not found")]
    ObjectNotFound,

    #[error("Conflict error")]
    ConflictError,

    #[error("Rate limited")]
    RateLimited,

    #[error("Internal server error")]
    InternalServerError,

    #[error("Service unavailable")]
    ServiceUnavailable,

    #[error("Database connection unavailable")]
    DatabaseConnectionUnavailable,

    #[error("Gateway timeout")]
    GatewayTimeout,
}
