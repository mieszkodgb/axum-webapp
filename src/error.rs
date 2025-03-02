use std::fmt;

use axum::response::{IntoResponse, Response};
use axum::http::StatusCode;
use serde::Serialize;
use tracing::debug;
pub type Result<T> = core::result::Result<T, Error>;

#[derive(Clone, Debug, Serialize)]
#[serde(tag = "type", content = "data")] //To specify how to serialize
pub enum Error{
    LoginFail,
    TicketNotFound {id: u64},
    AuthFailMissingToken,
    AuthFailWrongTokenFormat,
    AuthFailWrongTokenValue,
    AuthFailMissingContextInRequest,
}

// To make Error enum to string
impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

impl IntoResponse for Error {
    fn into_response(self) -> Response {
        debug!("Error into Res: {:?}", self);
        
        let mut response = StatusCode::INTERNAL_SERVER_ERROR.into_response();

        response.extensions_mut().insert(self);
        return response
    }
}

#[derive(Debug, Serialize)]
#[allow(non_camel_case_types)]
pub enum ClientError {
    LOGIN_FAIL,
    NO_AUTH,
    INVALID_PARAMS,
    SERVICE_ERROR
}

// To make ClientError enum to string
impl fmt::Display for ClientError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}


impl Error {
    pub fn client_status_and_error(&self) -> (StatusCode, ClientError){
        match self {
            Self::TicketNotFound { .. } => (StatusCode::BAD_REQUEST, ClientError::INVALID_PARAMS),
            Self::LoginFail => (StatusCode::FORBIDDEN, ClientError::LOGIN_FAIL),
            Self::AuthFailMissingToken
            | Self::AuthFailWrongTokenFormat
            | Self::AuthFailWrongTokenValue => (StatusCode::FORBIDDEN, ClientError::NO_AUTH),
            _ => (StatusCode::INTERNAL_SERVER_ERROR, ClientError::SERVICE_ERROR)
            
        }
    }

}

