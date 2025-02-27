use axum::extract::FromRequestParts;
use axum::http::request::Parts;
use axum::RequestPartsExt;
use chrono::{DateTime, Utc};
use regex::Regex;
use axum::body::Body;
use axum::response::Response;
use axum::{http::Request, middleware::Next};
use tower_cookies::Cookies;
use crate::errors::{Result, Error};
use crate::app_state::AppState;
use crate::web::AUTH_TOKEN;

#[derive(Debug)]
struct Token{
    user_id: u64,
    expiration: DateTime<Utc>,
    signature: String
}

pub async fn auth_check(
    state: Result<AppState>,
    req: Request<Body>,
    next: Next
) -> Result<Response> {

    println!("Checked for auth");

    state?;
    
    Ok(next.run(req).await)
}

impl<S: Send + Sync> FromRequestParts<S> for AppState {
    type Rejection = Error;

    async fn from_request_parts(parts: &mut Parts, _state: &S) -> std::result::Result<Self, Self::Rejection> {
        println!("Extractor App state");

        // Extract cookies
        let cookies = parts.extract::<Cookies>().await.unwrap();
        let auth_token = cookies.get(AUTH_TOKEN).map(|c| c.value().to_string());

        let token = auth_token
            .ok_or(Error::AuthFailMissingToken)
            .and_then(parse_token)?;

        let user_id = token.user_id.clone();
        //TODO add token validation https://www.youtube.com/watch?v=-9K7zNgsbP0
        let valid_token = validate_token(token);
        match valid_token {
            true => Ok(AppState::new(user_id)),
            false => Err(Error::AuthFailWrongTokenValue)
        } 
    }
}

// Parse token as 'user-[user.id].[expiration].[signature]
fn parse_token(token: String) -> Result<Token>{
    let re = Regex::new(r"^user-(?P<user_id>\d+)\.(?P<expiration_date>\d{2}-\d{2}-\d{2}\s\d{2}:\d{2}:\d{2})\.(?P<signature>.+)$").unwrap();
    let caps = re.captures(&token).ok_or(Error::AuthFailWrongTokenFormat)?;
    let auth_token = Token{
        user_id:caps["user_id"].to_string().parse::<u64>().map_err(|_| Error::AuthFailWrongTokenFormat)?,
        expiration:caps["expiration_date"].parse::<DateTime<Utc>>().map_err(|_| Error::AuthFailWrongTokenFormat)?,
        signature:caps["signature"].to_owned()
    };
    Ok(auth_token)
}

fn validate_token(token: Token) -> bool{
    if token.expiration > Utc::now(){
        return false
    }
    todo!();
    return true
}