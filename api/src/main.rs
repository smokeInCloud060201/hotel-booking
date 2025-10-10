mod env;

use actix_web::{delete, get, post, put, web, App, HttpResponse, HttpServer, Responder};
use config::Config;
use crate::env::config::Settings;

#[get("/")]
async fn hello() -> impl Responder {
    HttpResponse::Ok().body("Hello world!")
}

#[post("/echo")]
async fn echo(req_body: String) -> impl Responder {
    HttpResponse::Ok().body(req_body)
}

#[put("/users/{id}")]
async fn update_user(path: web::Path<u32>) -> impl Responder {
    let id = path.into_inner();
    HttpResponse::Ok().body(format!("Update user {id}"))
}

#[delete("/users/{id}")]
async fn delete_user(path: web::Path<u32>) -> impl Responder {
    let id = path.into_inner();
    HttpResponse::Ok().body(format!("Delete user {id}"))
}



#[actix_web::main]
async fn main() -> std::io::Result<()> {

    let settings = Settings::new().expect("Failed to load settings");

    println!("Starting server at {}:{}", settings.server.host, settings.server.port);

    HttpServer::new(|| {
        App::new()
            .service(hello)
            .service(echo)
            .service(update_user)
            .service(delete_user)
    })
        .bind((settings.server.host, settings.server.port))?
        .run()
        .await
}
