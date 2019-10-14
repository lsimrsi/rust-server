// not used

use actix_web::{middleware, web, App, HttpResponse, HttpServer};
use futures::Future;
use reqwest;
use reqwest::r#async::{Client, Response};

static REDDIT: &str = "http://www.reddit.com/r/rust.json";

fn get_rust_posts(
    _req: actix_web::HttpRequest,
    client: web::Data<Client>,
) -> impl Future<Item = HttpResponse, Error = actix_web::Error> {
    client.get(REDDIT).send()
        .and_then(|mut res| {
            // res.text() now returns a Future instead of a Result
            HttpResponse::Ok()
                .content_type("application/json")
                .body(res.text())
        })
        .map_err(reqwest::Error::from)
}

fn main() {
    HttpServer::new(|| {
        App::new()
            .data(Client::new())
            .wrap(middleware::Logger::default())
            .service(web::resource("/get/rust/posts").route(web::get().to_async(get_rust_posts)))
    })
    .bind("127.0.0.1:8000")
    .unwrap()
    .run()
    .unwrap();
}
