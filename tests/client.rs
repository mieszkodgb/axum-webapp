#![allow(unused)]

use axum::Json;
use serde_json::{json, Value};


#[tokio::test]
async fn client() -> Result<(),reqwest::Error>{
    let client = reqwest::Client::new();

    let res = reqwest::get("http://localhost:8080/hello").await?;
    println!("Response: {:?} {}", res.version(), res.status());
    println!("Headers: {:#?}\n", res.headers());

    let body = res.text().await?;

    println!("{body}");

    //Hello2
    let res = reqwest::get("http://localhost:8080/hello2/Mike").await?;
    println!("Response: {:?} {}", res.version(), res.status());
    println!("Headers: {:#?}\n", res.headers());

    let body = res.text().await?;

    println!("{body}");


    //Static page
    let res = reqwest::get("http://localhost:8080/src/main.rs").await?;
    println!("Response: {:?} {}", res.version(), res.status());
    println!("Headers: {:#?}\n", res.headers());

    let body = res.text().await?;

    println!("{body}");

    //Login
    let res_login = client.post("http://localhost:8080/api/login")
        .json(&json!({"username": "testuser", "password": "test123"}))
        .send().await?;
    println!("Response: {:?} {}", res_login.version(), res_login.status());
    println!("Headers: {:#?}\n", res_login.headers());

    let body = res_login.text().await?;

    println!("{body}");

    //Wrong login
    let res_login = client.post("http://localhost:8080/api/login")
        .json(&json!({"username": "testuser", "password": "test12"}))
        .send().await?;
    println!("Response: {:?} {}", res_login.version(), res_login.status());
    println!("Headers: {:#?}\n", res_login.headers());

    let body = res_login.text().await?;

    println!("{body}");

    Ok(())
}