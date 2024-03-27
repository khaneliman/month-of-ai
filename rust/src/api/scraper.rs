use crate::model::config::Config;
use actix_web::{get, web};
use openai_api_rs::v1::api::Client;
use openai_api_rs::v1::common::TEXT_EMBEDDING_3_LARGE;
use openai_api_rs::v1::embedding::{EmbeddingRequest, EmbeddingResponse};
use openai_api_rs::v1::thread::Message;
use spinners::{Spinner, Spinners};

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

    sp.stop();

    Ok(result)
}
