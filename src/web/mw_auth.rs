use axum::extract::{FromRequestParts, State};
use axum::http::request::Parts;
use chrono::{DateTime, Utc};
use regex::Regex;
use axum::body::Body;
use axum::response::Response;
use axum::{http::Request, middleware::Next};
use tower_cookies::{Cookie, Cookies};
use tracing::debug;
use crate::error::{Result, Error};
use crate::context::Context;
use crate::model::ModelController;
use crate::web::AUTH_TOKEN;

#[derive(Debug)]
struct Token{
    user_id: u64,
    expiration: String,//DateTime<Utc>,
    signature: String
}

pub async fn mw_auth_check(
    context: Result<Context>,
    req: Request<Body>,
    next: Next
) -> Result<Response> {

    debug!("Checked for auth");

    context?;
    
    Ok(next.run(req).await)
}

pub async fn mw_context_resolver(
    _mc: State<ModelController>, // AKA database connection
    cookies: Cookies,
    mut req: Request<Body>,
    next: Next,
) -> Result<Response>{

    debug!("Context resolver");
    let auth_token = cookies.get(AUTH_TOKEN).map(|c| c.value().to_string());
 
    let token = auth_token
            .ok_or(Error::AuthFailMissingToken)
            .and_then(parse_token)?;

    let user_id = token.user_id.clone();
    //TODO add token validation https://www.youtube.com/watch?v=-9K7zNgsbP0
    let valid_token = validate_token(token);
    let result_context = match valid_token {
        Ok(()) => Ok(Context::new(user_id)),
        Err(..) => Err(Error::AuthFailWrongTokenValue)
    };
    // Clean cookie if wrong cookie
    if result_context.is_err() && !matches!(result_context, Err(Error::AuthFailWrongTokenValue)){
        cookies.remove(Cookie::from(AUTH_TOKEN));
    }

    // Store app context into req
    req.extensions_mut().insert(result_context);
    Ok(next.run(req).await)
}

impl<S: Send + Sync> FromRequestParts<S> for Context {
    type Rejection = Error;

    async fn from_request_parts(parts: &mut Parts, _context: &S) -> Result<Self> {
        debug!("Extractor App context");

        let result_context = parts.extensions
            .get::<Result<Context>>()
            .ok_or(Error::AuthFailMissingContextInRequest)?
            .clone();

        result_context
    }
}

// Parse token as 'user-[user.id].[expiration].[signature]
fn parse_token(token: String) -> Result<Token>{
    let re = Regex::new(r"^user-(?P<user_id>\d+)\.(?P<expiration_date>.+)\.(?P<signature>.+)$").unwrap();
    debug!("Token is {:?}", token);
    let caps = re.captures(&token).ok_or(Error::AuthFailWrongTokenFormat)?;
    debug!("Token regex into {:?} - {:?} - {:?}",
            caps["user_id"].to_string(),
            caps["expiration_date"].to_string(),
            caps["signature"].to_string()
        );
    let auth_token = Token{
        user_id:caps["user_id"].to_string().parse::<u64>().map_err(|_| Error::AuthFailWrongTokenFormat)?,
        expiration:caps["expiration_date"].to_owned(),//parse::<DateTime<Utc>>().map_err(|_| Error::AuthFailWrongTokenFormat)?,
        signature:caps["signature"].to_owned()
    };
    debug!("Parse auth token");
    Ok(auth_token)
}

fn validate_token(token: Token) -> Result<()>{
    let expiration_date: DateTime<Utc> = token.expiration.parse::<DateTime<Utc>>()
        .map_err(|_| Error::AuthFailWrongTokenFormat)?;
    if expiration_date > Utc::now(){
        return Err(Error::AuthFailWrongTokenValue);
    }
    // todo!();
    return Ok(())
}
