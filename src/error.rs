use axum::{
    http::StatusCode,
    response::{IntoResponse, Response},
};
use oauth_axum::error::OauthError;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum ServerError {
    #[error("token request failed")]
    TokenRequestFailed,
    #[error("oauth url creation failed")]
    AuthUrlCreationFailed,
    #[error("no generated oauth url")]
    NoGeneratedOauthUrl,
    #[error("failed to generate oauth url")]
    GenerateOauthUrl,
    #[error("not found")]
    NotFound,
    #[error("conflict")]
    Conflict,
    #[error("multipart")]
    Mulipart(#[from] axum::extract::multipart::MultipartError),
    #[error(transparent)]
    Time(#[from] time::error::Format),
    #[error(transparent)]
    Sqlite(#[from] async_sqlite::Error),
}

// Implement `IntoResponse` for the error
impl IntoResponse for ServerError {
    fn into_response(self) -> Response {
        match self {
            Self::NotFound => (StatusCode::NOT_FOUND, "Not Found").into_response(),
            Self::Conflict => (StatusCode::CONFLICT, "Conflict").into_response(),
            _ => (StatusCode::INTERNAL_SERVER_ERROR, "Internal Server Error").into_response(),
        }
    }
}

impl From<OauthError> for ServerError {
    fn from(e: OauthError) -> Self {
        match e {
            OauthError::TokenRequestFailed => ServerError::TokenRequestFailed,
            OauthError::AuthUrlCreationFailed => ServerError::AuthUrlCreationFailed,
        }
    }
}
