use super::open_ai_request::Message;
use serde_derive::{Deserialize, Serialize};

#[derive(Deserialize, Debug)]
pub struct OAIChoices {
    pub text: String,
    pub index: u8,
    pub logprobs: Option<u8>,
    pub finish_reason: String,
}

#[derive(Deserialize, Debug)]
pub struct OAIResponse {
    pub id: Option<String>,
    pub object: Option<String>,
    pub created: Option<u64>,
    pub model: Option<String>,
    pub choices: Vec<OAIChoiceResponse>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct OAIChoiceResponse {
    pub message: Message,
}
