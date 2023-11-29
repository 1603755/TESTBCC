
use actix_web::{get, post, web, App, HttpResponse, HttpServer, Responder, http};
use actix_cors::Cors;
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
    println!("Hello, world!");
    HttpServer::new(|| {

        let cors = Cors::default()
            .allowed_origin("*")
            .allowed_methods(vec!["GET", "POST"]);

    
        App::new()
            .route("/hey", web::get().to(manual_hello))
    })
    .bind(("0.0.0.0", 80))?
    .run()
    .await
}