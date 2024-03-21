use crate::model::movie::{Movie, MovieCriteria};
use crate::model::open_ai_request::ResponseType::JsonObject;
use crate::model::open_ai_request::{Message, OAIRequest, ResponseFormat};
use crate::model::open_ai_response::OAIResponse;
use crate::model::query::{InputObject, QuestionObject};
use actix_web::http::header::ContentType;
use actix_web::{get, web, HttpResponse, Result};
use serde_json::{from_str, to_string};
use spinners::{Spinner, Spinners};

async fn fetch_movie_details(movie_id: &str) -> Result<Movie, Box<dyn std::error::Error>> {
    let client = reqwest::Client::new();

    let movie_details_response = client
        .get(format!(
            "https://srch-ai-demo.search.windows.net/indexes/idx-movies/docs/{}?api-version=2023-11-01",
            movie_id
        ))
        .header("Content-Type", "application/json")
        .header("api-key", "***REMOVED***")
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
) -> Result<HttpResponse, Box<dyn std::error::Error>> {
    println!("Movie ID: {}", movie_id);
    println!("Question: {}", query_object.question);

    let client = reqwest::Client::new();

    let mut sp = Spinner::new(Spinners::Dots9, "\t\tOpenAI is thinking...".into());

    let movie = fetch_movie_details(&movie_id).await?;
    // println!("{:?}", movie);

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

    let oai_request = OAIRequest::builder()
        .model(String::from("gpt-4-turbo-preview"))
        .message(system_message)
        .message(user_message)
        .build();

    let body = to_string(&oai_request).unwrap();
    // println!("{}", body);

    let prompt_response = client
        .post("https://oai-ai-demo-east.openai.azure.com/openai/deployments/gpt-4-turbo-preview/chat/completions?api-version=2024-02-15-preview")
        .header("Content-Type", "application/json")
        .header("api-key", "***REMOVED***")
        .body(body)
        .send()
        .await?;

    sp.stop();

    let response_body = prompt_response.text().await?;
    // println!("{}", response_body);

    let json: OAIResponse = from_str(&response_body)?;

    let message = json.choices[0].message.content.to_string();

    let response = HttpResponse::Ok()
        .insert_header(ContentType(mime::TEXT_PLAIN))
        .body(message);

    return Ok(response);
}

#[get("/api/movieCriteria")]
async fn get_movie_criteria(
    input_object: web::Query<InputObject>, // Extract question from query string
) -> Result<String, Box<dyn std::error::Error>> {
    println!("Question: {}", input_object.input);

    let client = reqwest::Client::new();

    let mut sp = Spinner::new(Spinners::Dots9, "\t\tOpenAI is thinking...".into());

    let system_message = Message::builder()
        .role(String::from("system"))
        .content(format!(
            "Please take the user's question to generate a application/json response object with the following format that can be used in an api call:
            {{
              \"search\"?: string, // A keyword search query
              \"genre\"?: string, // A genre to filter on. Single value. The genre should be one of: Action, Adventure, Animation, Comedy, Crime, Documentary, Drama, Family, Fantasy, History, Horror, Music, Mystery, Romance, Science Fiction, Thriller, War, Western, TV Movie.
              \"mpaa\"?: string, // An MPAA rating to filter on (PG, PG-13, R, etc.)
              \"releaseDateMin\"?: string, // The minimum release date to filter on. Format: YYYY-MM-DD
              \"releaseDateMax\"?: string, // The maximum release date to filter on. Format: YYYY-MM-DD
              \"scoreMin\"?: number, // The minimum vote/score/rating to filter on. 0-10 scale.
              \"scoreMax\"?: number // The maximum vote/score/rating to filter on. 0-10 scale.
            }}
            ",
        ))
        .build();

    let user_message = Message::builder()
        .role(String::from("user"))
        .content(format!("{}", input_object.input))
        .build();

    let oai_request = OAIRequest::builder()
        .model(String::from("gpt-4-turbo-preview"))
        .message(system_message)
        .message(user_message)
        .response_format(ResponseFormat { type_: JsonObject })
        .build();

    let body = to_string(&oai_request).unwrap();
    // println!("Movie Criteria Request: {}", body);

    let prompt_response = client
        .post("https://oai-ai-demo-east.openai.azure.com/openai/deployments/gpt-4-turbo-preview/chat/completions?api-version=2024-02-15-preview")
        .header("Content-Type", "application/json")
        .header("api-key", "***REMOVED***")
        .body(body)
        .send()
        .await?;

    sp.stop();

    let response_body = prompt_response.text().await?;
    println!("{}", response_body);

    let json: OAIResponse = from_str(&response_body)?;
    // let json: MovieCriteria = from_str(&response_body)?;
    // println!("{:?}", json);

    let message = json.choices[0].message.content.to_string();
    // println!("{}", message);

    // let response = HttpResponse::Ok()
    // .insert_header(ContentType(mime::APPLICATION_JSON))
    // .body(response_body);

    // println!("Response: {:?}", response);

    return Ok(message);
}
