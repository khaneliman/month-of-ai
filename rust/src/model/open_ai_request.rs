use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct Message {
    pub role: String,
    pub content: String,
}

impl Message {
    pub fn builder() -> MessageBuilder {
        MessageBuilder::new()
    }
}

pub struct MessageBuilder {
    role: Option<String>,
    content: Option<String>,
}

impl MessageBuilder {
    pub fn new() -> Self {
        MessageBuilder {
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

    pub fn build(self) -> Message {
        Message {
            role: self.role.expect("Role is required for UserMessage"),
            content: self.content.expect("Content is required for UserMessage"),
        }
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ResponseFormat {
    #[serde(rename = "type")]
    pub type_: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct OAIRequest {
    pub model: Option<String>,
    pub messages: Vec<Message>,
    pub response_format: Option<ResponseFormat>,
}

impl OAIRequest {
    pub fn builder() -> OAIRequestBuilder {
        OAIRequestBuilder::new()
    }
}

pub struct OAIRequestBuilder {
    model: Option<String>,
    messages: Vec<Message>,
    response_format: Option<ResponseFormat>,
}

impl OAIRequestBuilder {
    pub fn new() -> Self {
        OAIRequestBuilder {
            model: None,
            messages: Vec::new(),
            response_format: None,
        }
    }
    pub fn model(mut self, model: String) -> Self {
        self.model = Some(model);
        self
    }

    pub fn message(mut self, message: Message) -> Self {
        self.messages.push(message);
        self
    }

    pub fn response_format(mut self, response_format: ResponseFormat) -> Self {
        self.response_format = Some(response_format);
        self
    }

    pub fn build(self) -> OAIRequest {
        OAIRequest {
            model: self.model,
            messages: self.messages,
            response_format: self.response_format,
        }
    }
}
