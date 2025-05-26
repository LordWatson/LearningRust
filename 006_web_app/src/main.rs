use actix_web::{web, App, HttpServer, HttpResponse, Responder};
use actix_files::Files;
use serde::Serialize;

#[derive(Serialize)]
struct Message {
    text: String,
}

async fn hello() -> impl Responder {
    println!("Hello from Rust backend!");
    HttpResponse::Ok()
        .content_type("application/json")
        .json(Message {
            text: "Hello from Rust backend!".to_string(),
        })
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            // static files
            .service(Files::new("/", "./src/frontend").index_file("index.html"))
            // API endpoints
            .service(
                web::scope("/api")
                    .route("/hello", web::get().to(hello))
            )
    })
        .bind("127.0.0.1:8080")?
        .run()
        .await
}