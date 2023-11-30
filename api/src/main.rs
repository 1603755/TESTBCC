
use actix_web::{get, post, web, App, HttpResponse, HttpServer, Responder, http};
use actix_cors::Cors;
use api::db;
use api::services::{
    process_request,
    process_change_door,
    process_get_door
};

#[get("/")]
async fn hello() -> impl Responder {
    HttpResponse::Ok().body("Hello world!")
}

#[post("/echo")]
async fn echo(req_body: String) -> impl Responder {
    HttpResponse::Ok().body(req_body)
}

async fn manual_hello() -> impl Responder {
    HttpResponse::Ok().body("Hey there!")
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
            .service(hello)
            .service(echo)
            .service(process_request)
            .service(process_change_door)
            .service(process_get_door)
            .route("/hey", web::get().to(manual_hello))
    })
    .bind(("0.0.0.0", 80))?
    .run()
    .await
}