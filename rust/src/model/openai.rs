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

// #[derive(Serialize, Debug)]
// pub struct OAIRequest {
//     pub prompt: String,
//     pub max_tokens: u32,
// }

#[derive(Serialize, Deserialize, Debug)]
pub struct UserMessage {
    pub role: String,
    pub content: String,
}

impl UserMessage {
    pub fn builder() -> UserMessageBuilder {
        UserMessageBuilder::new()
    }
}

pub struct UserMessageBuilder {
    role: Option<String>,
    content: Option<String>,
}

impl UserMessageBuilder {
    pub fn new() -> Self {
        UserMessageBuilder {
            role: None,
            content: None,
        }
    }

    pub fn role(mut self, role: String) -> Self {
        self.role = Some(role);
        self
    }

    pub fn content(mut self, content: String) -> Self {
        self.content = Some(content);
        self
    }

    pub fn build(self) -> UserMessage {
        UserMessage {
            role: self.role.expect("Role is required for UserMessage"),
            content: self.content.expect("Content is required for UserMessage"),
        }
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct OAIRequest {
    pub messages: Vec<UserMessage>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct OAIChoiceResponse {
    pub message: UserMessage,
}
