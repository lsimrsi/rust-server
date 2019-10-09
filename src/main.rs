use actix_web::{web, App, HttpRequest, HttpResponse, Responder, HttpServer, middleware};
use futures::Future;
use reqwest::{Error};
use reqwest::r#async::Client as HttpClient;

fn hello() -> impl Responder {
    HttpResponse::Ok().body("Hello world!")
}

fn get_rust_posts() -> impl Future<Item = String, Error = String> {
    Box::new(HttpClient::new().get("http://www.reddit.com/r/rust.json").send()
    .and_then(|resp| resp.text())
    .map_err(|error| format!("Error: {:?}", error))



        // .and_then(|mut resp| resp)
        // .map(|_| {
        //     HttpResponse::Ok().body(actix_web::body::Body::from("asdf"))
        // })
        // .map_err(|error| format!("Error: {:?}", error))

    // .map_err(|error| format!("Error: {:?}", error))
    // .map(|resp| resp)
    // HttpResponse::Ok()



    // client
    //     .get("http://www.reddit.com/r/rust.json") // create request builder
    //     .header("User-Agent", "Actix-web")
    //     .send() // send http request
    //     .map_err(Error::from)
    //     .and_then(|resp| Ok::<HttpResponse, Error>(HttpResponse::Ok().streaming(resp)))
}

fn main() {
    // HttpServer::new(|| {
    //     App::new()
    //         .wrap(middleware::Logger::default())
    //         .service(web::resource("/get/rust/posts").route(web::get().to_async(get_rust_posts)))
    //     // App::new()
    //     //     .route("/", web::get().to(index))
    //     //     .route("/again", web::get().to(index2))
    // })
    HttpServer::new(|| {
        App::new()
            .route("/hello/", web::get().to(hello))
            .route("/get/rust/posts/", web::get().to_async(get_rust_posts))
    })
    .bind("127.0.0.1:8000")
    .unwrap()
    .run()
    .unwrap();
    println!("Hello, world!");
}
