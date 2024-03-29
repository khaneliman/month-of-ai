use crate::model::config::Config;
use crate::model::embedding_request_body::EmbeddingRequestBody;
use actix_web::http::header::ContentType;
use actix_web::{get, web, HttpResponse, Result};
use log::{debug, error, info, warn};
use openai_api_rs::v1::embedding::EmbeddingData;
use spinners::{Spinner, Spinners};
use std::fs::File;
use std::fs::OpenOptions;
use std::io::Write;

#[get("/api/embed_movie_json")]
async fn embed_movie_json(
    config: web::Data<Config>,
) -> Result<HttpResponse, Box<dyn std::error::Error>> {
    let config_data = config.clone();

    // TODO: fetch movie json from data folder

    let client = reqwest::Client::new();

    // TODO: replace dummy string with json data from files
    let embedding_request = EmbeddingRequestBody::builder()
        .input(vec!["input1".to_string(), "input2".to_string()])
        .model(Some("text-embedding-3-large".to_string()))
        .dimensions(Some(1024))
        .build();
    debug!("embedding_request: {:?}", embedding_request);

    let body = serde_json::to_string(&embedding_request).unwrap();

    let mut sp = Spinner::new(Spinners::Dots9, "\t\tOpenAI is thinking...".into());

    let result = client
        .post(format!(
            "{}openai/deployments/text-embedding-3-large/embeddings?api-version={}",
            config_data.open_ai.url, config_data.open_ai.api_version
        ))
        .header("Content-Type", "application/json")
        .header("api-key", config_data.open_ai.key.clone())
        .body(body)
        .send()
        .await?;

    debug!("result: {:?}", result);
    // NOTE: can't use sdk for now
    // let result = client.embedding(req)?;
    // debug!("result: {:?}", result);

    sp.stop();

    let response_body = result.text().await?;
    debug!("{}", response_body);

    // Serialize the EmbeddingResponse into a JSON string
    let embedding_data: EmbeddingData = serde_json::from_str(&response_body)?;
    let json = serde_json::to_string(&embedding_data);

    let mut sp = Spinner::new(Spinners::Dots9, "\t\tSaving embedding to file...".into());
    // Write the JSON data to a file named embeddings.json
    // Check if the file exists
    if std::path::Path::new("embeddings.json").exists() {
        // If the file exists, open it in append mode
        let mut file = OpenOptions::new().append(true).open("embeddings.json")?;
        writeln!(file, "{:?}", json)?;
    } else {
        // If the file doesn't exist, create a new file
        let mut file = File::create("embeddings.json")?;
        file.write(json);
    }

    sp.stop();

    let response = HttpResponse::Ok()
        .insert_header(ContentType(mime::TEXT_PLAIN))
        .body("");

    return Ok(response);
}
