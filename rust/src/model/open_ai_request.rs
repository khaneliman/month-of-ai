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
pub struct OAIRequest {
    pub messages: Vec<Message>,
}

impl OAIRequest {
    pub fn builder() -> OAIRequestBuilder {
        OAIRequestBuilder::new()
    }
}

pub struct OAIRequestBuilder {
    messages: Vec<Message>,
}

impl OAIRequestBuilder {
    pub fn new() -> Self {
        OAIRequestBuilder {
            messages: Vec::new(),
        }
    }

    pub fn message(mut self, message: Message) -> Self {
        self.messages.push(message);
        self
    }

    pub fn build(self) -> OAIRequest {
        OAIRequest {
            messages: self.messages,
        }
    }
}
