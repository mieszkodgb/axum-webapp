use axum::{extract::{Path, Query}, response::{Html, IntoResponse}, routing::{get}, Router};
use serde::Deserialize;
use tokio::net::TcpListener;

#[allow(unused)]

#[tokio::main]
async fn main() {
    let routes_all = Router::new().merge(routes_hello());
    let listener = TcpListener::bind("127.0.0.1:8080").await.unwrap();
	println!("LISTENING on {:?}\n", listener.local_addr());
	axum::serve(listener, routes_all.into_make_service())
        .await
        .unwrap();
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
