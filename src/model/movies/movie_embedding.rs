use openai_api_rs::v1::embedding::EmbeddingResponse;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct MovieEmbedding {
    pub movie_id: i32,
    pub embeddings: Option<EmbeddingResponse>,
}

impl MovieEmbedding {
    pub fn builder() -> MovieEmbeddingBuilder {
        MovieEmbeddingBuilder::new()
    }
}

pub struct MovieEmbeddingBuilder {
    movie_id: Option<i32>,
    embeddings: Option<EmbeddingResponse>,
}

impl MovieEmbeddingBuilder {
    pub fn new() -> Self {
        MovieEmbeddingBuilder {
            movie_id: None,
            embeddings: None,
        }
    }

    pub fn movie_id(mut self, movie_id: i32) -> Self {
        self.movie_id = Some(movie_id);
        self
    }

    pub fn embeddings(mut self, embeddings: EmbeddingResponse) -> Self {
        self.embeddings = Some(embeddings);
        self
    }

    pub fn build(self) -> MovieEmbedding {
        MovieEmbedding {
            movie_id: self.movie_id.unwrap(),
            embeddings: self.embeddings,
        }
    }
}
