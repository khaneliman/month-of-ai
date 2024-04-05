use crate::model::cache::Cache;
use crate::model::chat_completion_request::{
    ChatCompletionRequest, Message, ResponseFormat, ResponseType::JsonObject,
};
use crate::model::chat_completion_response::ChatCompletionResponse;
use crate::model::config::Config;
use crate::model::movies::movie::TopRatedMovie;
use crate::model::movies::{movie::Movie, movie_criteria::MovieCriteria};
use crate::model::query::{InputObject, QuestionObject};
use crate::util::movie_helper::{can_load_data, find_similar_movies};
use actix_web::http::header::ContentType;
use actix_web::{get, post, web, HttpResponse, Result};
use log::{debug, error, info, warn};
use serde_json::{from_str, to_string};
use spinners::{Spinner, Spinners};
use std::sync::Mutex;

async fn fetch_movie_details(
    movie_id: &str,
    config: web::Data<Config>,
) -> Result<Movie, Box<dyn std::error::Error>> {
    let client = reqwest::Client::new();

    // Fetch movie details for movie_id
    let movie_details_response = client
        .get(format!(
            "{}indexes/idx-movies/docs/{}?api-version={}",
            config.azure_search.url, movie_id, config.azure_search.api_version
        ))
        .header("Content-Type", "application/json")
        .header("api-key", config.azure_search.key.clone())
        .send()
        .await?;

    let movie_details = movie_details_response.text().await?;
    let movie: Movie = from_str(&movie_details)?;

    Ok(movie)
}

#[get("/api/movies/{movie_id}/askQuestion")]
async fn ask_question(
    movie_id: web::Path<String>,              // Extract movieID from path
    query_object: web::Query<QuestionObject>, // Extract question from query string
    config: web::Data<Config>,
) -> Result<HttpResponse, Box<dyn std::error::Error>> {
    debug!("Movie ID: {}", movie_id);
    debug!("Question: {}", query_object.question);
    debug!("Parsed config: {:?}", config);

    let config_data = config.clone();

    let client = reqwest::Client::new();

    let mut sp = Spinner::new(Spinners::Dots9, "\t\tOpenAI is thinking...".into());

    let movie = fetch_movie_details(&movie_id, config_data.clone()).await?;
    debug!("{:?}", movie);

    let system_message = Message::builder()
        .role(String::from("system"))
        .content(format!(
            "Please answer the user's question using the provided movie context.
            context: {:?}",
            movie,
        ))
        .build();

    let user_message = Message::builder()
        .role(String::from("user"))
        .content(format!("{}", query_object.question))
        .build();

    let oai_request = ChatCompletionRequest::builder()
        .model(String::from(config_data.open_ai.model.clone()))
        .message(system_message)
        .message(user_message)
        .build();

    let body = to_string(&oai_request).unwrap();
    debug!("{}", body);

    // Call API with prompt and parse response
    let prompt_response = client
        .post(format!(
            "{}openai/deployments/{}/chat/completions?api-version={}",
            config_data.open_ai.url, config_data.open_ai.model, config_data.open_ai.api_version
        ))
        .header("Content-Type", "application/json")
        .header("api-key", config_data.open_ai.key.clone())
        .body(body)
        .send()
        .await?;

    sp.stop();

    let response_body = prompt_response.text().await?;
    debug!("{}", response_body);

    let json: ChatCompletionResponse = from_str(&response_body)?;

    let message = json.choices[0].message.content.to_string();
    debug!("{}", message);

    // Return the response as plain text
    let response = HttpResponse::Ok()
        .insert_header(ContentType(mime::TEXT_PLAIN))
        .body(message);

    return Ok(response);
}

#[get("/api/movieCriteria")]
async fn get_movie_criteria(
    input_object: web::Query<InputObject>, // Extract question from query string
    config: web::Data<Config>,
) -> Result<String, Box<dyn std::error::Error>> {
    debug!("Question: {}", input_object.input);

    let config_data = config.clone();

    let client = reqwest::Client::new();

    let mut sp = Spinner::new(Spinners::Dots9, "\t\tOpenAI is thinking...".into());

    let system_message = Message::builder()
        .role(String::from("system"))
        .content(format!(
            r#"Please take the user's question to generate a application/json response object with the following format that can be used in an api call:
            {{
              "search"?: string, // A keyword search query
              "genre"?: string, // A genre to filter on. Single value. The genre should be one of: Action, Adventure, Animation, Comedy, Crime, Documentary, Drama, Family, Fantasy, History, Horror, Music, Mystery, Romance, Science Fiction, Thriller, War, Western, TV Movie.
              "mpaa"?: string, // An MPAA rating to filter on (PG, PG-13, R, etc.)
              "releaseDateMin"?: string, // The minimum release date to filter on. Format: YYYY-MM-DD
              "releaseDateMax"?: string, // The maximum release date to filter on. Format: YYYY-MM-DD
              "scoreMin"?: number, // The minimum vote/score/rating to filter on. 0-10 scale.
              "scoreMax"?: number // The maximum vote/score/rating to filter on. 0-10 scale.
            }}
            "#,
        ))
        .build();

    let user_message = Message::builder()
        .role(String::from("user"))
        .content(format!("{}", input_object.input))
        .build();

    let oai_request = ChatCompletionRequest::builder()
        .model(String::from(config_data.open_ai.model.clone()))
        .message(system_message)
        .message(user_message)
        .response_format(ResponseFormat { type_: JsonObject })
        .build();

    let body = to_string(&oai_request).unwrap();
    debug!("Movie Criteria Request: {}", body);

    let prompt_response = client
        .post(format!(
            "{}openai/deployments/{}/chat/completions?api-version={}",
            config_data.open_ai.url, config_data.open_ai.model, config_data.open_ai.api_version
        ))
        .header("Content-Type", "application/json")
        .header("api-key", config_data.open_ai.key.clone())
        .body(body)
        .send()
        .await?;

    sp.stop();

    let response_body = prompt_response.text().await?;
    debug!("{}", response_body);

    let json: ChatCompletionResponse = from_str(&response_body)?;

    let movie_criteria_response: MovieCriteria =
        from_str(&json.choices[0].message.content.to_string())?;
    debug!("{:?}", movie_criteria_response);

    let message = json.choices[0].message.content.to_string();

    return Ok(message);
}

#[get("/api/movies/{movie_id}/similar")]
async fn similar_movies(
    movie_id: web::Path<String>, // Extract movieID from path
    cache: web::Data<Mutex<Cache>>,
) -> Result<String, Box<dyn std::error::Error>> {
    debug!("Movie ID: {}", movie_id);

    if can_load_data(&cache) {
        let mut cosine_similarities = find_similar_movies(
            &movie_id,
            &cache.lock().unwrap().movie_embeddings.lock().unwrap(),
        );

        if cosine_similarities.len() > 0 {
            cosine_similarities.sort_by(|a, b| b.similarity.partial_cmp(&a.similarity).unwrap());

            let cache_lock = cache.lock().unwrap();

            let top_movies_lock = cache_lock.top_movies.lock().unwrap();

            let similar_movies: Vec<TopRatedMovie> = cosine_similarities
                .iter()
                .take(10)
                .map(|similarity| {
                    let movie = top_movies_lock
                        .iter()
                        .find(|x| x.id == similarity.movie_id)
                        .unwrap();
                    movie.clone()
                })
                .collect();

            Ok(serde_json::to_string(&similar_movies).unwrap())
        } else {
            Err(Box::<dyn std::error::Error>::from(
                "No similar movies found.".to_string(),
            ))
        }
    } else {
        Err(Box::<dyn std::error::Error>::from(
            "JSON files not found.".to_string(),
        ))
    }
}

#[post("/api/movie-chat")]
async fn movie_chat(
    chat_messages: web::Json<ChatCompletionRequest>, // conversation from the app
    config: web::Data<Config>,
    cache: web::Data<Mutex<Cache>>,
) -> Result<HttpResponse, Box<dyn std::error::Error>> {
    debug!("Chat Messages: {:?}", chat_messages);

    let config_data = config.clone();

    let client = reqwest::Client::new();

    let mut sp = Spinner::new(Spinners::Dots9, "\t\tOpenAI is thinking...".into());

    // TODO: get movie criteria from the last question
    let system_message = Message::builder()
        .role(String::from("system"))
        .content(format!(
            r#"You are an expert movie critic. You will be tasked with providing movie recommendations to someone based on criteria they provide.
            You will need to phish for more information until you think you are ready to answer the question using the movie criteria.
            "#,
        ))
        .build();

    // Extract the chat_messages from web::Json<ChatCompletionRequest>
    let chat_messages_inner = chat_messages.into_inner();

    // Iterate over each message in the messages vector and add a .message(message) call for each one
    let mut oai_request_builder = ChatCompletionRequest::builder()
        .model(config_data.open_ai.model.clone())
        .message(system_message); // Start with the system message

    for message in chat_messages_inner.messages {
        oai_request_builder = oai_request_builder.message(message);
    }

    // Add any additional fields like response_format if needed
    let oai_request = oai_request_builder.build();

    let body = to_string(&oai_request).unwrap();
    debug!("{}", body);

    // Call API with prompt and parse response
    let prompt_response = client
        .post(format!(
            "{}openai/deployments/{}/chat/completions?api-version={}",
            config_data.open_ai.url, config_data.open_ai.model, config_data.open_ai.api_version
        ))
        .header("Content-Type", "application/json")
        .header("api-key", config_data.open_ai.key.clone())
        .body(body)
        .send()
        .await?;

    sp.stop();

    let response_body = prompt_response.text().await?;
    debug!("{}", response_body);

    let json: ChatCompletionResponse = from_str(&response_body)?;

    let message = json.choices[0].message.content.to_string();
    debug!("{}", message);

    // Return the response as plain text
    let response = HttpResponse::Ok()
        .insert_header(ContentType(mime::TEXT_PLAIN))
        .body(message);

    return Ok(response);
}
