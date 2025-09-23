use axum::{
    http::StatusCode,
    response::{IntoResponse, Response},
};
use oauth_axum::error::OauthError;

#[derive(Debug)]
pub enum ServerError {
    TokenRequestFailed,
    AuthUrlCreationFailed,
    NoGeneratedOauthUrl,
    GenerateOauthUrl,
}

// Implement `IntoResponse` for the error
impl IntoResponse for ServerError {
    fn into_response(self) -> Response {
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
