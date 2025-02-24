use axum::body::Body;
use axum::response::Response;
use axum::{http::Request, middleware::Next};
use tower_cookies::Cookies;
use crate::errors::{Result, Error};

use crate::web::AUTH_TOKEN;

pub async fn auth_check(
    cookies: Cookies,
    req: Request<Body>,
    next: Next
) -> Result<Response> {

    println!("Checked for auth");
    let auth_token = cookies.get(AUTH_TOKEN).map(|c| c.value().to_string());

    // TODO add token check
    auth_token.ok_or(Error::AuthFail)?;


    Ok(next.run(req).await)
}