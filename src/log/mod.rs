use crate::{context::Context, error::ClientError, Error, Result};
use chrono::Utc;
use serde::Serialize;
use serde_json::{json, Value};
use serde_with::skip_serializing_none;
use axum::http::{Method, Uri};
use tracing::debug;


// Not tracing but logger for services like prometheus
#[skip_serializing_none] // To avoid serializing Option::None
#[derive(Serialize)]
struct RequestLogLine {

    // uuid: String,
    timestamp: String,

    user_id: Option<u64>,

    req_path: String,
    req_method: String,

    client_error_type: Option<String>,
    error_type: Option<String>,
    error_data: Option<Value>,

}

pub async fn log_request(
    // uuid, Uuid,
    req_method: Method,
    uri: Uri,
    context: Option<Context>,
    service_error: Option<&Error>,
    client_error: Option<ClientError>,
) -> Result<()> {

    let timestamp = Utc::now();

    let error_type = service_error
        .map(|serv_error| (*serv_error).to_string());
    let error_data = serde_json::to_value(service_error)
        .ok()
        .and_then(|mut val| val.get_mut("data").map(|val| val.take()));


    let log_line = RequestLogLine {
        // uuid: uuid.to_string(),
        timestamp: timestamp.to_string(),
        user_id: context.map(|context| context.user_id()),
        req_path: uri.path().to_string(),
        req_method: req_method.to_string(),
        client_error_type: client_error.map(|err| err.to_string()),
        error_type: error_type,
        error_data: error_data

    };

    debug!("LogLine: {:?}", json!(log_line));

    //TODO Send logline to logger service (like cloudwtach or prometheus)

    Ok(())
    
}
