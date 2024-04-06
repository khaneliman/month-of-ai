use log::debug;

use crate::model::chat_completion_request::{RequestTool, ToolFunction};

pub fn return_filter_tool() -> RequestTool {
    let filter_function = ToolFunction::builder()
    .name("filter_movies".to_string())
    .description("Filters movies based on the movie criteria. Requires both movie_criteria and top_rated_movies to filter down.".to_string())
    .parameters(serde_json::json!({
        "type": "object",
        "properties": {
            "movie_criteria": {
                "type": "object",
                "properties": {
                    "search": { "type": "string" },
                    "genre": { "type": "string" },
                    "mpaa": { "type": "string" },
                    "release_date_min": { "type": "string" },
                    "release_date_max": { "type": "string" },
                    "score_min": { "type": "number" },
                    "score_max": { "type": "number" },
                    "natural_language": { "type": "string" }
                },
                "required": ["search"]
            },
            "top_rated_movies": {
                "type": "array",
                "items": {
                    "type": "object",
                    "properties": {
                        "backdrop_path": { "type": "string" },
                        "id": { "type": "integer" },
                        "title": { "type": "string" },
                        "poster_path": { "type": "string" },
                        "release_date": { "type": "string" },
                        "vote_average": { "type": "number" },
                        "vote_count": { "type": "integer" },
                        "popularity": { "type": "number" },
                        "overview": { "type": "string" },
                        "imdb_id": { "type": "string" },
                        "budget": { "type": "integer" },
                        "homepage": { "type": "string" },
                        "revenue": { "type": "integer" },
                        "runtime": { "type": "integer" },
                        "tagline": { "type": "string" },
                        "genres": { "type": "array", "items": { "type": "string" } },
                        "cast": { "type": "array", "items": { "type": "object" } },
                        "keywords": { "type": "array", "items": { "type": "string" } },
                        "mpaa": { "type": "string" },
                        "summaries": { "type": "array", "items": { "type": "string" } },
                        "synopsis": { "type": "string" },
                        "imdb_score": { "type": "number" }
                    },
                    "required": ["backdrop_path", "id", "title", "poster_path", "release_date", "vote_average", "vote_count", "popularity", "mpaa", "imdb_score"]
                }
            }
        },
        "required": ["movie_criteria", "top_rated_movies"]
    }))
    .build();

    let filter_tool = RequestTool::builder().function(filter_function).build();
    debug!("filter_tool: {:?}", filter_tool);

    filter_tool
}
