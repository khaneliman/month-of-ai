use crate::model::config::Config;
use crate::model::embedding_request_body::EmbeddingRequestBody;
use crate::model::movies::movie::TopRatedMovie;
use crate::model::movies::movie_embedding::MovieEmbedding;
use actix_web::http::header::ContentType;
use actix_web::{get, web, HttpResponse, Result};
use log::{debug, error, info, warn};
use openai_api_rs::v1::embedding::EmbeddingResponse;
use spinners::{Spinner, Spinners};
use std::collections::HashSet;
use std::fs::File;
use std::io::{BufReader, ErrorKind, Write};
use std::io::{Read, Seek};

#[get("/api/embed_movie_json")]
async fn embed_movie_json(
    config: web::Data<Config>,
) -> Result<HttpResponse, Box<dyn std::error::Error>> {
    let config_data = config.clone();

    let movies_embedded = read_embedded_movies("src/data/embeddings.json")?;
    let mut existing_movie_ids = populate_existing_movie_ids(movies_embedded);
    let top_rated_movies = read_top_rated_movies("src/data/topRatedMovies.json")?;

    for movie in top_rated_movies {
        let movie_id = movie.id;
        let movie_json_path = format!("src/data/movies/{}.json", movie_id);

        // Check if movie ID already exists in embeddings.json
        if existing_movie_ids.contains(&movie_id) {
            // Skip processing the movie if it already exists
            continue;
        }

        let movie = read_movie_json(&movie_json_path)?;

        let embedding_request = EmbeddingRequestBody::builder()
            .input(vec![
                movie.overview.unwrap_or("".to_string()),
                movie.tagline.unwrap_or("".to_string()),
                movie.genres.join(","),
                movie.keywords.unwrap_or_default().join(","),
                movie.summaries.unwrap_or_default().join(","),
                movie.synopsis.unwrap_or("".to_string()),
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

        let json = serde_json::to_value(&movie_embedding).unwrap();

        let mut sp = Spinner::new(Spinners::Dots9, "\t\tSaving embedding to file...".into());

        let _ = write_json_to_file(json);

        sp.stop();

        // Add the movie ID to the HashSet after processing
        existing_movie_ids.insert(movie_id);
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

fn read_embedded_movies(
    file_path: &str,
) -> Result<Vec<MovieEmbedding>, Box<dyn std::error::Error>> {
    debug!("Reading file: {}", file_path);
    let file = match File::open(file_path) {
        Ok(file) => file,
        Err(err) if err.kind() == ErrorKind::NotFound => {
            // If the file does not exist, create an empty vector and return it
            return Ok(Vec::new());
        }
        Err(err) => return Err(err.into()),
    };
    let reader = BufReader::new(file);
    debug!("reader: {:?}", reader);
    let movies: Vec<MovieEmbedding> = serde_json::from_reader(reader)?;
    debug!("movies read: {:?}", movies);
    Ok(movies)
}

fn populate_existing_movie_ids(movies_embedded: Vec<MovieEmbedding>) -> HashSet<i32> {
    let mut existing_movie_ids = HashSet::new();

    for movie_embedding in movies_embedded {
        existing_movie_ids.insert(movie_embedding.movie_id);
    }

    existing_movie_ids
}

fn write_json_to_file(json: serde_json::Value) -> Result<(), Box<dyn std::error::Error>> {
    let json_string = json.to_string();

    let mut file = if std::path::Path::new("src/data/embeddings.json").exists() {
        // Open in read-write mode to modify content
        std::fs::OpenOptions::new()
            .read(true)
            .write(true)
            .open("src/data/embeddings.json")?
    } else {
        // Create a new file with the initial `[`
        File::create("src/data/embeddings.json")?.write_all(b"[")?;
        File::open("src/data/embeddings.json")? // Reopen for efficient append
    };

    let mut contents = String::new();
    file.read_to_string(&mut contents)?;

    // Seek to the last ']' position, not including any trailing whitespace
    let last_bracket_index = contents
        .trim_end()
        .rfind(']')
        .unwrap_or_else(|| contents.len());

    file.seek(std::io::SeekFrom::Start(last_bracket_index as u64))?; // Use u64 for seek offset

    // Write a comma before the existing bracket if the file isn't empty
    if last_bracket_index > 1 {
        write!(file, ",")?;
    }

    // Write the new JSON object and close the bracket
    writeln!(file, "{}", json_string)?;
    writeln!(file, "]")?; // Ensure a closing bracket at the end

    Ok(())
}
