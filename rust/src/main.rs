mod api;
mod model;

use actix_cors::Cors;
use actix_web::{http::header, middleware::Logger, web, App, HttpServer};
use api::movies::ask_question;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    std::env::set_var("RUST_LOG", "debug");
    std::env::set_var("RUST_BACKTRACE", "1");
    env_logger::init();

    HttpServer::new(move || {
        let cors = Cors::default()
            .allowed_origin("http://localhost:4001") // For development
            .allowed_methods(vec!["GET", "POST"])
            .allowed_headers(vec![
                header::AUTHORIZATION,
                header::ACCEPT,
                header::CONTENT_TYPE,
            ])
            .max_age(3600); // Set preflight request cache duration (optional)

        let logger = Logger::default();
        App::new()
            .wrap(logger)
            .wrap(cors)
            .app_data(web::JsonConfig::default().limit(4096)) // <- limit size of the payload (global configuration)
            .service(ask_question)
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
