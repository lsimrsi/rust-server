use actix_web::{get, App, HttpServer, Responder};

static CRATES: &str = "https://play.rust-lang.org/meta/crates";

#[get("/crates")]
async fn get_crates() -> impl Responder {
    reqwest::get(CRATES).await.unwrap().text().await.unwrap()
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| App::new().service(get_crates))
        .bind(("127.0.0.1", 8080))?
        .run()
        .await
}
