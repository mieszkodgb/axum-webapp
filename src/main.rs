use axum::{extract::Query, response::{Html, IntoResponse}, routing::get, Router};
use serde::Deserialize;
use tokio::net::TcpListener;

#[allow(unused)]

#[tokio::main]
async fn main() {
    let routes_hello = Router::new().route(
        "/hello",
        get(handler_hello),
    );
    let listener = TcpListener::bind("127.0.0.1:8080").await.unwrap();
	println!("LISTENING on {:?}\n", listener.local_addr());
	axum::serve(listener, routes_hello.into_make_service())
        .await
        .unwrap();
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