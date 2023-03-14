use std::collections::HashMap;

use actix_web::{get, middleware, web, App, HttpServer, Responder};

static CRATES: &str = "http://play.rust-lang.org/meta/crates";

#[get("/crates")]
async fn get_crates() -> impl Responder {
    let resp = reqwest::get(CRATES).await.unwrap().bytes().await.unwrap();
    println!("{:#?}", resp);
    "asdf"
    // let res = client.
    //     .get(CRATES)
    //     .insert_header(("User-Agent", "Actix-web"))
    //     .send()
    //     .await;

    // match res {
    //     Ok(r) => "heyo".to_string(),
    //     Err(err) => err.to_string(),
    // }
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| App::new().service(get_crates))
        .bind(("127.0.0.1", 8080))?
        .run()
        .await
}
