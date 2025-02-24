use axum::{extract::{Path, Query}, middleware, response::{Html, IntoResponse, Response}, routing::{get, get_service}, Router};
use models::ModelController;
use serde::Deserialize;
use tokio::net::TcpListener;
use tower_cookies::CookieManagerLayer;
use tower_http::services::ServeDir;
use errors::Result;

#[allow(unused)]

mod errors;
mod web;
mod models;

#[tokio::main]
async fn main() -> Result<()> {

    let mc = ModelController::new().await?;

    let routes_api = web::routes_tickets::routes(mc.clone())
        .route_layer(middleware::from_fn(web::middleware::auth_check));

    let routes_all = Router::new()
            .merge(routes_hello())
            .merge(web::routes_login::routes())
            .nest("/api", routes_api)
            .layer(middleware::map_response(main_response_mapper))
            .layer(CookieManagerLayer::new())
            .fallback_service(routes_static());
    let listener = TcpListener::bind("127.0.0.1:8080").await.unwrap();
	println!("LISTENING on {:?}\n", listener.local_addr());
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
    println!("Handler hello - {params:?}");

    let name = params.name.as_deref().unwrap_or("World!");
    Html(format!("Hello <strong>{name}</strong>"))
}

// e.g. '/hello2/Mike'
async fn handler_hello2(Path(name): Path<String>) -> impl IntoResponse{
    println!("Handler hello2 - {name:?}");
    Html(format!("Hello <strong>{name}</strong>"))
}

fn routes_static() -> Router {
    Router::new().fallback_service(get_service(ServeDir::new("./")))
}

async fn main_response_mapper(res: Response) -> Response {
    println!("Response mapper");
    res
}
