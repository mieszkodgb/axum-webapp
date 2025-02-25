use crate::{errors::{Error, Result}, web};

use axum::{routing::post, Json, Router};
use serde::Deserialize;
use serde_json::{json, Value};
use tower_cookies::{Cookie, Cookies};


pub fn routes() -> Router{
    Router::new().route("/api/login", post(api_login))
}
async fn api_login(cookies: Cookies, paylod: Json<LoginPayload>) -> Result<Json<Value>>{
    println!("Login route");

    // TODO add real db/auth logic
    if paylod.username != "testuser" || paylod.password != "test123" {
        return Err(Error::LoginFail);
    }
    
    // TODO: implement real auth-token generation/signature
    cookies.add(Cookie::new(web::AUTH_TOKEN, "user-1.exp.sign")); // format as user id, expiration date, signature

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

