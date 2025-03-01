#![allow(unused)]

use axum::Json;
use reqwest::cookie::Jar;
use serde_json::{json, Value};


#[tokio::test]
async fn client() -> Result<(),reqwest::Error>{
    let jar = Jar::default();
    let client = reqwest::Client::builder()
        .cookie_store(true)
        .build()?;
    
    // Hello
    println!("Request hello");
    let res = reqwest::get("http://localhost:8080/hello").await?;
    println!("Response: {:?} {}", res.version(), res.status());
    println!("Headers: {:#?}\n", res.headers());

    let body = res.text().await?;

    println!("{body}");

    //Hello2
    println!("Request hello2 Mike");
    let res = reqwest::get("http://localhost:8080/hello2/Mike").await?;
    println!("Response: {:?} {}", res.version(), res.status());
    println!("Headers: {:#?}\n", res.headers());

    let body = res.text().await?;

    println!("{body}");


    //Static page
    println!("Request Static page");
    let res = reqwest::get("http://localhost:8080/src/main.rs").await?;
    println!("Response: {:?} {}", res.version(), res.status());
    println!("Headers: {:#?}\n", res.headers());

    let body = res.text().await?;

    println!("{body}");

    //Wrong login
    println!("Post wrong login");
    let res_login = client.post("http://localhost:8080/api/login")
        .json(&json!({"username": "testuser", "password": "test12"}))
        .send().await?;
    println!("Response: {:?} {}", res_login.version(), res_login.status());
    println!("Headers: {:#?}\n", res_login.headers());

    let body = res_login.text().await?;

    println!("{body}");

    //Login
    println!("Post good login");
    let res_login = client.post("http://localhost:8080/api/login")
        .json(&json!({"username": "testuser", "password": "test123"}))
        .send().await?;

    println!("Response: {:?} {}", res_login.version(), res_login.status());
    println!("Headers: {:#?}\n", res_login.headers());

    let body = res_login.text().await?;

    println!("{body}");


    //Create
    println!("Create ticket");
    let res = client.post("http://localhost:8080/api/ticket")
        .json(&json!({"title": "Start ticket", "content": "This is a new ticket"}))
        .send().await?;
    println!("Response: {:?} {}", res.version(), res.status());
    println!("Headers: {:#?}\n", res.headers());

    let body = res.text().await?;

    println!("{body}");

    //Get
    println!("Get first ticket");
    let res = client.get("http://localhost:8080/api/ticket/0")
        .send().await?;
    println!("Response: {:?} {}", res.version(), res.status());
    println!("Headers: {:#?}\n", res.headers());

    let body = res.text().await?;

    //Update
    println!("Update first ticket");
    let res = client.patch("http://localhost:8080/api/ticket/0/update")
        .json(&json!({"title": "Start ticket", "content": "This is a new ticket (Update)"}))
        .send().await?;
    println!("Response: {:?} {}", res.version(), res.status());
    println!("Headers: {:#?}\n", res.headers());

    let body = res.text().await?;

    println!("{body}");


    //List
    println!("List all tickets");
    let res = client.get("http://localhost:8080/api/tickets")
        .send().await?;
    println!("Response: {:?} {}", res.version(), res.status());
    println!("Headers: {:#?}\n", res.headers());

    let body = res.text().await?;

    println!("{body}");

    //Delete
    println!("Delete first ticket");
    let res = client.delete("http://localhost:8080/api/ticket/0/delete")
        .send().await?;
    println!("Response: {:?} {}", res.version(), res.status());
    println!("Headers: {:#?}\n", res.headers());

    let body = res.text().await?;

    println!("{body}");


    Ok(())
}