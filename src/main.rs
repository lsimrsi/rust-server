use actix_web::{middleware, web, App, HttpResponse, HttpServer};
use futures::Future;
use reqwest;
use reqwest::Client;

static REDDIT: &str = "http://www.reddit.com/r/rust.json";

fn get_rust_posts(
    _req: actix_web::HttpRequest,
    client: web::Data<Client>,
) -> impl Future<Item = HttpResponse, Error = actix_web::Error> {
    let builder = client.get(REDDIT);
    actix_web::web::block(move || builder.send())
        .from_err()
        .and_then(|mut res| match res.text() {
            Ok(body) => HttpResponse::Ok()
                .content_type("application/json")
                .body(body),
            Err(error) => {
                println!("get_request error: {}", error);
                HttpResponse::InternalServerError()
                    .content_type("application/json")
                    .body(format!("{{\"error\": \"Error getting response text.\"}}"))
            }
        })
}

fn main() {
    HttpServer::new(|| {
        App::new()
            .data(Client::new())
            .wrap(middleware::Logger::default())
            .service(web::resource("/get/rust/posts").route(web::get().to_async(get_rust_posts)))
    })
    .bind("127.0.0.1:5000")
    .unwrap()
    .run()
    .unwrap();
}
