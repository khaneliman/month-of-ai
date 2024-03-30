use openai_api_rs::v1::embedding::EmbeddingResponse;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct MovieEmbedding {
    movie_id: i32,
    embeddings: EmbeddingResponse,
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
}
