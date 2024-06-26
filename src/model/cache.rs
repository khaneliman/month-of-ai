use super::movies::movie::TopRatedMovie;
use crate::model::movies::movie_embedding::MovieEmbedding;
use std::sync::Mutex;

pub struct Cache {
    pub movie_embeddings: Mutex<Vec<MovieEmbedding>>,
    pub top_movies: Mutex<Vec<TopRatedMovie>>,
}
