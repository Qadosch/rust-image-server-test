use actix_web::{get, App, HttpResponse, HttpServer, Responder};

#[get("/")]
async fn next_image() -> impl Responder {
    HttpResponse::Ok().body("Hello world!")
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .service(next_image)
    })
    .bind(("0.0.0.0", 8080))?
    .run()
    .await
}
