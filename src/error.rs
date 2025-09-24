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
    #[error(transparent)]
    Time(#[from] time::error::Format),
    #[error(transparent)]
    Sqlite(#[from] async_sqlite::Error),
}

// Implement `IntoResponse` for the error
impl IntoResponse for ServerError {
    fn into_response(self) -> Response {
        tracing::error!(error = ?self);
        (StatusCode::INTERNAL_SERVER_ERROR, "Internal Server Error").into_response()
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
