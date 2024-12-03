use actix_web::{web, App, HttpServer, Responder, HttpResponse};

// Example endpoint
async fn hello() -> impl Responder {
    HttpResponse::Ok().body("Hello, from the REST API!")
}

// Main function for the REST API
pub async fn run_server() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .route("/", web::get().to(hello))
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}
