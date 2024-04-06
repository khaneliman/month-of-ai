use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct EmbeddingRequestBody {
    pub input: Vec<String>,
    pub model: String,
    pub encoding_format: Option<String>,
    pub dimensions: Option<i32>,
    pub user: Option<String>,
}

impl EmbeddingRequestBody {
    pub fn builder() -> EmbeddingRequestBodyBuilder {
        EmbeddingRequestBodyBuilder::new()
    }
}

pub struct EmbeddingRequestBodyBuilder {
    input: Vec<String>,
    model: Option<String>,
    encoding_format: Option<String>,
    dimensions: Option<i32>,
    user: Option<String>,
}

impl EmbeddingRequestBodyBuilder {
    pub fn new() -> Self {
        EmbeddingRequestBodyBuilder {
            input: Vec::new(),
            model: None,
            encoding_format: None,
            dimensions: None,
            user: None,
        }
    }

    pub fn input(mut self, input: Vec<String>) -> Self {
        self.input = input;
        self
    }

    pub fn model(mut self, model: Option<String>) -> Self {
        self.model = model;
        self
    }

    pub fn encoding_format(mut self, encoding_format: Option<String>) -> Self {
        self.encoding_format = encoding_format;
        self
    }

    pub fn dimensions(mut self, dimensions: Option<i32>) -> Self {
        self.dimensions = dimensions;
        self
    }

    pub fn user(mut self, user: Option<String>) -> Self {
        self.user = user;
        self
    }

    pub fn build(self) -> EmbeddingRequestBody {
        EmbeddingRequestBody {
            input: self.input,
            model: self.model.expect("model is required"),
            encoding_format: self.encoding_format,
            dimensions: self.dimensions,
            user: self.user,
        }
    }
}
