use app_state::AppState;
use axum::{extract::{Path, Query}, http::{Method, Uri}, middleware, response::{Html, IntoResponse, Response}, routing::{get, get_service}, Json, Router};
use log::log_request;
use models::ModelController;
use serde::Deserialize;
use serde_json::json;
use tokio::net::TcpListener;
use tower_cookies::CookieManagerLayer;
use tower_http::services::ServeDir;
use errors::{Result, Error};

#[allow(unused)]

mod errors;
mod web;
mod log;
mod models;
mod app_state;

#[tokio::main]
async fn main() -> Result<()> {

    let mc = ModelController::new().await?;

    let routes_api = web::routes_tickets::routes(mc.clone())
        .route_layer(middleware::from_fn(web::middleware::auth_check));

    let routes_all = Router::new()
            .nest("/api", routes_api)
            .layer(middleware::map_response(main_response_mapper))
            .layer(middleware::from_fn_with_state(
                mc.clone(),
                web::middleware::state_resolver
            ))
            .merge(web::routes_login::routes())
            .layer(CookieManagerLayer::new())
            .merge(routes_hello())
            .fallback_service(routes_static())
            ;
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

async fn main_response_mapper(
    state: Result<AppState>,
    uri: Uri,
    req_method: Method,
    res: Response
    ) -> Response {
    println!("Response mapper");

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
            println!("Client error is {:?}", client_error);
            (*status_code, Json(client_error_body.to_string())).into_response()
        });

    // Server log
    println!("Error: {:?}", service_error);
    let client_error = client_status_error.unzip().1;

    // TOCheck if error what happens here?
    let state = state.ok();
    let _ = log_request(req_method, uri, state, service_error, client_error).await;

    error_response.unwrap_or(res)
}
