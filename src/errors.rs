use axum::response::{IntoResponse, Response};
use axum::http::StatusCode;
pub type Result<T> = core::result::Result<T, Error>;

#[derive(Debug)]
pub enum Error{
    LoginFail,
    TicketNotFound {id: u64}
}

impl IntoResponse for Error {
    fn into_response(self) -> Response {
        println!("Error into Res");
        (StatusCode::INTERNAL_SERVER_ERROR, "UNHANDLE_CLIENT_ERROR").into_response()
        
    }
}
