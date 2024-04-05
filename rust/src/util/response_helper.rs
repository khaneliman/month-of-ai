use crate::model::{
    chat_completion_response::{ChatCompletionChoice, ChatCompletionResponse},
    movies::movie_criteria::MovieCriteria,
};
use log::debug;
use serde::Deserialize;

pub fn extract_message(json: &ChatCompletionResponse) -> String {
    match &json.choices {
        choices if !choices.is_empty() => {
            if let Some(movie_criteria) = handle_tool_calls(choices) {
                debug!("Tool call: {:?}", movie_criteria);

                return serde_json::to_string(&movie_criteria)
                    .unwrap_or_else(|_| "Error serializing MovieCriteria".to_string());
            } else {
                debug!("Choices: {:?}", choices);

                return handle_choices(choices);
            }
        }
        _ => "No choices found in JSON".to_string(),
    }
}

pub fn handle_choices(choices: &Vec<ChatCompletionChoice>) -> String {
    for choice in choices {
        if let Some(content) = choice.message.content.as_ref() {
            return content.to_string();
        }
    }
    "Couldn't parse a message".to_string()
}

#[derive(Debug, Deserialize)]
struct RootObject {
    movie_criteria: MovieCriteria,
}

pub fn handle_tool_calls(choices: &Vec<ChatCompletionChoice>) -> Option<MovieCriteria> {
    for choice in choices {
        if let Some(tool_calls) = &choice.message.tool_calls {
            for call in tool_calls {
                if let Some(arguments) = &call.function.arguments {
                    debug!("Arguments: {:?}", arguments);
                    let arguments_str: &str = arguments;
                    debug!("Arguments str: {}", arguments_str);

                    match serde_json::from_str::<RootObject>(arguments_str) {
                        Ok(movie_criteria) => {
                            debug!("MovieCriteria: {:?}", movie_criteria.movie_criteria);

                            return Some(movie_criteria.movie_criteria);
                        }
                        Err(e) => {
                            debug!("Error deserializing MovieCriteria: {:?}", e);
                            return None;
                        }
                    }
                }
            }
        }
    }
    None
}
