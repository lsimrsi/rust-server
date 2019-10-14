use actix_web::{middleware, web, App, HttpResponse, HttpServer};
use futures::Future;
use reqwest;
use reqwest::Client as BlockingClient;

static REDDIT: &str = "http://www.reddit.com/r/rust.json";
static SLOWWLY: &str =
    "http://slowwly.robertomurray.co.uk/delay/5000/url/http://www.reddit.com/r/rust.json";

fn get_request(builder: reqwest::RequestBuilder) -> impl Future<Item = HttpResponse, Error = actix_web::Error> {
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

fn get_rust_posts(
    _req: actix_web::HttpRequest,
    client: web::Data<BlockingClient>,
) -> impl Future<Item = HttpResponse, Error = actix_web::Error> {
    let builder = client.get(REDDIT);
    get_request(builder)
}

fn get_rust_posts_slowwly(
    _req: actix_web::HttpRequest,
    client: web::Data<BlockingClient>,
) -> impl Future<Item = HttpResponse, Error = actix_web::Error> {
    let builder = client.get(SLOWWLY);
    get_request(builder)
}

fn main() {
    HttpServer::new(|| {
        App::new()
            .data(BlockingClient::new())
            .wrap(middleware::Logger::default())
            .service(web::resource("/get/rust/posts").route(web::get().to_async(get_rust_posts)))
            .service(
                web::resource("/get/rust/posts/slowwly")
                    .route(web::get().to_async(get_rust_posts_slowwly)),
            )
    })
    .bind("127.0.0.1:8000")
    .unwrap()
    .run()
    .unwrap();
}
