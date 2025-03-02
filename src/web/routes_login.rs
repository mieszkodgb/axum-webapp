use crate::{error::{Error, Result}, web};

use axum::{routing::post, Json, Router};
use chrono::{Duration, Utc};
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
    let expiration_date = Utc::now() + Duration::days(1);
    let new_token = format!("user-{user_id}.{expiration}.{signature}",
        user_id=1,
        expiration=expiration_date.to_string(),
        signature="sign"
    );
    cookies.add(Cookie::new(web::AUTH_TOKEN, new_token)); // format as user id, expiration date, signature

    let body = Json(json!({
        "result": {
            "success": true
        }
    }));
    println!("Login successful");
    Ok(body)
}
 
#[derive(Debug, Deserialize)]
struct LoginPayload {
    username: String,
    password: String,
}

