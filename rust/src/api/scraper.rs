use crate::model::config::Config;
use actix_web::{get, web};
use openai_api_rs::v1::api::Client;
use openai_api_rs::v1::common::TEXT_EMBEDDING_3_LARGE;
use openai_api_rs::v1::embedding::{EmbeddingRequest, EmbeddingResponse};
use spinners::{Spinner, Spinners};
use std::fs::File;
use std::fs::OpenOptions;
use std::io::Write;

async fn embed_movie_json(
    config: web::Data<Config>,
) -> Result<EmbeddingResponse, Box<dyn std::error::Error>> {
    let config_data = config.clone();

    let client = Client::new(config_data.open_ai.key.to_string());

    // TODO: fetch movie json from data folder

    // TODO: replace dummy string with json data from files
    let mut req =
        EmbeddingRequest::new(TEXT_EMBEDDING_3_LARGE.to_string(), "story time".to_string());
    req.dimensions = Some(1056);

    let mut sp = Spinner::new(Spinners::Dots9, "\t\tOpenAI is thinking...".into());

    let result = client.embedding(req)?;

    // Serialize the EmbeddingResponse into a JSON string
    let json_str = serde_json::to_string(&result)?;

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

    Ok(result)
}
