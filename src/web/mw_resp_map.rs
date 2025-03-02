use crate::{context::Context, log::log_request};
use axum::{http::{Method, Uri},
    response::{IntoResponse, Response},
    Json
    };
use crate::error::{Result, Error};
use serde_json::json;
use tracing::debug;

pub async fn mw_response_mapper(
    context: Result<Context>,
    uri: Uri,
    req_method: Method,
    res: Response
    ) -> Response {
    debug!("Response mapper");

    let service_error = res.extensions().get::<Error>();
    let client_status_error = service_error
        .map(|serv_err| serv_err.client_status_and_error());

    let error_response: Option<axum::http::Response<axum::body::Body>> = client_status_error
        .as_ref()
        .map(|(status_code, client_error)|{
            let client_error_body = json!({
                "error:":{
                    "type": client_error
                }
            });
            debug!("Client error is {:?}", client_error);
            (*status_code, Json(client_error_body.to_string())).into_response()
        });

    // Server log
    debug!("Error: {:?}", service_error);
    let client_error = client_status_error.unzip().1;

    // TOCheck if error what happens here?
    let context = context.ok();
    let _ = log_request(req_method, uri, context, service_error, client_error).await;

    error_response.unwrap_or(res)
}