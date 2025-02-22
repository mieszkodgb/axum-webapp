#![allow(unused)]


#[tokio::test]
async fn client() -> Result<(),reqwest::Error>{
    let client = reqwest::Client::new();

    let res = reqwest::get("http://localhost:8080/hello").await?;
    println!("Response: {:?} {}", res.version(), res.status());
    println!("Headers: {:#?}\n", res.headers());

    let body = res.text().await?;

    println!("{body}");
    Ok(())
}