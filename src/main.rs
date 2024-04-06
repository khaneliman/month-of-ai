mod api;
mod model;
mod util;

use actix_cors::Cors;
use actix_web::web::Data;
use actix_web::{http::header, middleware::Logger, web, App, HttpServer};
use api::movies::{ask_question, get_movie_criteria, movie_chat, similar_movies};
use api::scraper::embed_movie_json;
use log::debug;
use std::sync::Mutex;
use std::{fs::File, io::Read};

use crate::model::cache::Cache;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    std::env::set_var("RUST_LOG", "debug");
    std::env::set_var("RUST_BACKTRACE", "1");

    env_logger::init();

    let mut config_file = File::open("config.yaml")?;
    let mut config_str = String::new();
    config_file.read_to_string(&mut config_str)?;

    let config: crate::model::config::Config =
        serde_yaml::from_str(&config_str).expect("error getting config");

    // Initialize the cache
    let cache = Data::new(Mutex::new(Cache {
        movie_embeddings: Mutex::new(Vec::new()), // You can initialize this with actual data if available
        top_movies: Mutex::new(Vec::new()), // You can initialize this with actual data if available
    }));

    debug!("{:?}", config);

    HttpServer::new(move || {
        let cors = Cors::default()
            .allowed_origin(&config.front_end_url) // For development
            .allowed_methods(vec!["GET", "POST"])
            .allowed_headers(vec![
                header::AUTHORIZATION,
                header::ACCEPT,
                header::CONTENT_TYPE,
            ])
            .max_age(3600);

        let logger = Logger::default();
        App::new()
            .wrap(logger)
            .wrap(cors)
            .app_data(web::JsonConfig::default().limit(4096)) // <- limit size of the payload (global configuration)
            .app_data(Data::new(config.clone()))
            .app_data(Data::clone(&cache))
            .service(ask_question)
            .service(get_movie_criteria)
            .service(embed_movie_json)
            .service(similar_movies)
            .service(movie_chat)
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
