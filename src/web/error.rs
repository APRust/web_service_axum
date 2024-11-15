use crate::web;
use axum::http::StatusCode;
use axum::response::{IntoResponse, Response};
use serde::Serialize;
use std::fmt::Formatter;

pub type Result<T> = core::result::Result<T, Error>;

#[derive(Debug, Serialize, strum_macros::AsRefStr)]
#[serde(tag = "type", content = "data")]
pub enum Error {
    // -- Login
    LoginFail,

    // -- CtxExtError
    CtxExt(web::mw_auth::CtxExtError),
}

// region:    --- Axum IntoResponse
impl IntoResponse for Error {
    fn into_response(self) -> Response {
        println!("->> {:<12} - into_response", "HANDLER");

        // Create a placeholder Axum response
        let mut response = StatusCode::INTERNAL_SERVER_ERROR.into_response();

        // Insert the Error into the response
        response.extensions_mut().insert(self);

        response
    }
}

// endregion: --- Axum IntoResponse

// region:    --- Error Boilerplate
impl core::fmt::Display for Error {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}

impl std::error::Error for Error {}

// endregion: --- Error Boilerplate

// region:    --- Client Error
impl Error {
    pub fn client_status_and_error(&self) -> (StatusCode, ClientError) {
        use web::Error::*;

        #[allow(unreachable_patterns)]
        match self {
            // -- Login/Auth
            CtxExt(_) => (StatusCode::FORBIDDEN, ClientError::NO_AUTH),

            // -- Fallback.
            _ => (StatusCode::INTERNAL_SERVER_ERROR, ClientError::SERVER_ERROR),
        }
    }
}

#[derive(Debug, strum_macros::AsRefStr)]
#[allow(non_camel_case_types)]
pub enum ClientError {
    LOGIN_FAIL,
    NO_AUTH,
    SERVER_ERROR,
}

// endregion: --- Client Error
