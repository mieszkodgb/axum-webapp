use axum::{extract::{Path, Query},
    middleware,
    response::{Html, IntoResponse},
    routing::get, Router
    };
use model::ModelController;
use serde::Deserialize;
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
use web::mw_resp_map::mw_response_mapper;

#[tokio::main]
async fn main() -> Result<()> {
    tracing_subscriber::fmt()
        // .without_time()
        .with_env_filter(EnvFilter::from_default_env())
        .init();
    let mc = ModelController::new().await?;

    let routes_api = web::routes_tickets::routes(mc.clone())
        .route_layer(middleware::from_fn(web::mw_auth::mw_auth_check));

    let routes_all = Router::new()
            .nest("/api", routes_api)
            .layer(middleware::map_response(mw_response_mapper))
            .layer(middleware::from_fn_with_state(
                mc.clone(),
                web::mw_auth::mw_context_resolver
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
