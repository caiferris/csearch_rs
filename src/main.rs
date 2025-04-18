use actix_web::{App, HttpResponse, HttpServer, Responder, get};

#[get("/")]
async fn home() -> impl Responder {
    HttpResponse::Ok().body("csearch_rs - OK")
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| App::new().service(home))
        .bind(("127.0.0.1", 8080))?
        .run()
        .await
}
