use context::Context;
use axum::{extract::{Path, Query},
    http::{Method, Uri}, middleware,
    response::{Html, IntoResponse, Response},
    routing::{get, get_service}, Json, Router
    };
use log::log_request;
use model::ModelController;
use serde::Deserialize;
use serde_json::json;
use tokio::net::TcpListener;
use tower_cookies::CookieManagerLayer;
use error::{Result, Error};
use tracing::{debug, info};
use tracing_subscriber::EnvFilter;

#[allow(unused)]

mod error;
mod web;
mod log;
mod model;
mod context;
mod config;

pub use config::config;

#[tokio::main]
async fn main() -> Result<()> {
    tracing_subscriber::fmt()
        // .without_time()
        .with_env_filter(EnvFilter::from_default_env())
        .init();
    let mc = ModelController::new().await?;

    let routes_api = web::routes_tickets::routes(mc.clone())
        .route_layer(middleware::from_fn(web::middleware::auth_check));

    let routes_all = Router::new()
            .nest("/api", routes_api)
            .layer(middleware::map_response(main_response_mapper))
            .layer(middleware::from_fn_with_state(
                mc.clone(),
                web::middleware::context_resolver
            ))
            .merge(web::routes_login::routes())
            .layer(CookieManagerLayer::new())
            .merge(routes_hello())
            .fallback_service(web::routes_static::serve_dir())
            ;
    let listener = TcpListener::bind("127.0.0.1:8080").await.unwrap();
	info!("LISTENING on {:?}\n", listener.local_addr());
	axum::serve(listener, routes_all.into_make_service())
        .await
        .unwrap();

    Ok(())
}


fn routes_hello() -> Router{
    Router::new()
        .route("/hello", get(handler_hello))
        .route("/hello2/{name}", get(handler_hello2))
}

#[derive(Debug, Deserialize)]
struct HelloParams{
    name: Option<String>
}

// e.g. '/hello?params=Mike'
async fn handler_hello(Query(params): Query<HelloParams>) -> impl IntoResponse{
    debug!("Handler hello - {params:?}");

    let name = params.name.as_deref().unwrap_or("World!");
    Html(format!("Hello <strong>{name}</strong>"))
}

// e.g. '/hello2/Mike'
async fn handler_hello2(Path(name): Path<String>) -> impl IntoResponse{
    debug!("Handler hello2 - {name:?}");
    Html(format!("Hello <strong>{name}</strong>"))
}

async fn main_response_mapper(
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
