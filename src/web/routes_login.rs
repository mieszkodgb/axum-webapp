use crate::errors::{Error, Result};

use axum::{routing::post, Json, Router};
use serde::Deserialize;
use serde_json::{json, Value};


pub fn routes() -> Router{
    Router::new().route("/api/login", post(api_login))
}
async fn api_login(paylod: Json<LoginPayload>) -> Result<Json<Value>>{
    println!("Login route");

    // TODO add real db/auth logic
    if paylod.username != "testuser" || paylod.password != "test123" {
        return Err(Error::LoginFail);
    }
    // TODO set cookies

    let body = Json(json!({
        "result": {
            "success": true
        }
    }));

    Ok(body)
}
 
#[derive(Debug, Deserialize)]
struct LoginPayload {
    username: String,
    password: String
}

