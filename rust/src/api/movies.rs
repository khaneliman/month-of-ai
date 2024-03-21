use crate::model::movie::Movie;
use crate::model::open_ai_request::{Message, OAIRequest};
use crate::model::open_ai_response::OAIResponse;
use crate::model::query::QueryObject;
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
    movie_id: web::Path<String>,           // Extract movieID from path
    query_object: web::Query<QueryObject>, // Extract question from query string
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
