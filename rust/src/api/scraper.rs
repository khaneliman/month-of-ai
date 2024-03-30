use crate::model::config::Config;
use crate::model::embedding_request_body::EmbeddingRequestBody;
use crate::model::movies::movie::TopRatedMovie;
use crate::model::movies::movie_embedding::MovieEmbedding;
use actix_web::http::header::ContentType;
use actix_web::{get, web, HttpResponse, Result};
use log::{debug, error, info, warn};
use openai_api_rs::v1::embedding::EmbeddingResponse;
use spinners::{Spinner, Spinners};
use std::fs::File;
use std::fs::OpenOptions;
use std::io::{BufReader, Write};

#[get("/api/embed_movie_json")]
async fn embed_movie_json(
    config: web::Data<Config>,
) -> Result<HttpResponse, Box<dyn std::error::Error>> {
    let config_data = config.clone();

    let top_rated_movies = read_top_rated_movies("src/data/topRatedMovies.json")?;

    for movie in top_rated_movies {
        let movie_id = movie.id;
        let movie_json_path = format!("src/data/movies/{}.json", movie_id);
        let movie = read_movie_json(&movie_json_path)?;

        let embedding_request = EmbeddingRequestBody::builder()
            .input(vec![
                movie.overview.unwrap_or_default(),
                movie.tagline.unwrap_or_default(),
                movie.genres.join(","),
                movie.keywords.unwrap_or_default().join(","),
                movie.summaries.unwrap_or_default().join(","),
                movie.synopsis.unwrap_or_default(),
            ])
            .model(Some("text-embedding-3-large".to_string()))
            .dimensions(Some(1024))
            .user(Some("ah-scraper".to_string()))
            .build();
        debug!("embedding_request: {:?}", embedding_request);

        let body = serde_json::to_string(&embedding_request).unwrap();

        let mut sp = Spinner::new(Spinners::Dots9, "\t\tOpenAI is thinking...".into());

        let client = reqwest::Client::new();
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

        sp.stop();

        let response_body = result.text().await?;
        debug!("Response Body: {}", response_body);

        // Serialize the EmbeddingResponse into a JSON string
        let embedding_data: EmbeddingResponse = serde_json::from_str(&response_body)?;

        let movie_embedding = MovieEmbedding::builder()
            .movie_id(movie_id)
            .embeddings(embedding_data)
            .build();

        let json = serde_json::to_string(&movie_embedding);

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
            file.write(json.unwrap().to_string().as_bytes())?;
        }

        sp.stop();
    }

    let response = HttpResponse::Ok()
        .insert_header(ContentType(mime::TEXT_PLAIN))
        .body("");

    return Ok(response);
}

fn read_top_rated_movies(
    file_path: &str,
) -> Result<Vec<TopRatedMovie>, Box<dyn std::error::Error>> {
    debug!("Reading file: {}", file_path);
    let file = File::open(file_path)?;
    let reader = BufReader::new(file);
    let movies: Vec<TopRatedMovie> = serde_json::from_reader(reader)?;
    Ok(movies)
}

fn read_movie_json(file_path: &str) -> Result<TopRatedMovie, Box<dyn std::error::Error>> {
    debug!("Reading file: {}", file_path);
    let file = File::open(file_path)?;
    let reader = BufReader::new(file);
    let movie: TopRatedMovie = serde_json::from_reader(reader)?;
    Ok(movie)
}
