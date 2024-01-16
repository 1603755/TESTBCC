
use actix_web::{get, post, web, App, HttpResponse, HttpServer, Responder, http, web::Redirect};
use actix_cors::Cors;
use api::db;
use api::services::{
    process_request,
    process_change_door,
    process_get_door,
    get_rfid_table,
    process_get_login
};
use std::fs;

async fn get_html () -> impl Responder {
    let html = fs::read_to_string("./web/home/index.html").unwrap();
    HttpResponse::Ok()
        .content_type("text/html; charset=utf-8")
        .body(html)
}

async fn get_login() -> impl Responder {
    let html = fs::read_to_string("./web/index.html").unwrap();
    HttpResponse::Ok()
        .content_type("text/html; charset=utf-8")
        .body(html)
}

#[post("/echo")]
async fn echo(req_body: String) -> impl Responder {
    HttpResponse::Ok().body(req_body)
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    db::check_or_create_table().unwrap();
    println!("Hello, world!");
    HttpServer::new(|| {

        let cors = Cors::default()
            .allow_any_origin()
            .allow_any_method()
            .allow_any_header()
            .supports_credentials();
        App::new()
            .wrap(cors)
            .service(echo)
            .service(process_request)
            .service(process_change_door)
            .service(process_get_door)
            .service(get_rfid_table)
            .service(process_get_login)
            .route("/", web::get().to(get_html))
            .route("/home", web::get().to(get_html))
            .route("/index", web::get().to(get_login))
    })
    .bind(("0.0.0.0", 80))?
    .run()
    .await
}