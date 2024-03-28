use crate::model::config::Config;
use actix_web::http::header::ContentType;
use actix_web::{get, web, HttpResponse, Result};
use log::{debug, error, info, warn};
use openai_api_rs::v1::api::Client;
use openai_api_rs::v1::common::TEXT_EMBEDDING_3_LARGE;
use openai_api_rs::v1::embedding::EmbeddingRequest;
use spinners::{Spinner, Spinners};
use std::fs::File;
use std::fs::OpenOptions;
use std::io::Write;

#[get("/api/embed_movie_json")]
async fn embed_movie_json(
    config: web::Data<Config>,
) -> Result<HttpResponse, Box<dyn std::error::Error>> {
    let config_data = config.clone();

    let client = Client::new_with_endpoint(
        config_data.open_ai.url.to_string(),
        config_data.open_ai.key.to_string(),
    );
    debug!("api-key: {}", config_data.open_ai.key);
    debug!("api-url: {}", config_data.open_ai.url);

    // TODO: fetch movie json from data folder

    // TODO: replace dummy string with json data from files
    let mut req =
        EmbeddingRequest::new(TEXT_EMBEDDING_3_LARGE.to_string(), "story time".to_string());
    req.dimensions = Some(1024);

    let mut sp = Spinner::new(Spinners::Dots9, "\t\tOpenAI is thinking...".into());

    let result = client.embedding(req)?;

    // Serialize the EmbeddingResponse into a JSON string
    let json_str = serde_json::to_string(&result)?;
    debug!("json_str: {}", json_str);

    // Write the JSON data to a file named embeddings.json
    // Check if the file exists
    if std::path::Path::new("embeddings.json").exists() {
        // If the file exists, open it in append mode
        let mut file = OpenOptions::new().append(true).open("embeddings.json")?;
        writeln!(file, "{}", json_str)?;
    } else {
        // If the file doesn't exist, create a new file
        let mut file = File::create("embeddings.json")?;
        file.write_all(json_str.as_bytes())?;
    }

    sp.stop();
    let response = HttpResponse::Ok()
        .insert_header(ContentType(mime::TEXT_PLAIN))
        .body(json_str);

    return Ok(response);
}
