use actix_web::{App, HttpServer};
mod handlers;
mod models;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    println!("🚀 Cat-Agent Microservice running on http://localhost:8080 🐱");

    HttpServer::new(|| {
        App::new()
            .service(handlers::agent_decision)
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
