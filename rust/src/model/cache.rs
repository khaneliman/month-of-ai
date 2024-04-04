use super::movies::movie::TopRatedMovie;
use crate::model::movies::movie_embedding::MovieEmbedding;
use log::debug;
use spinners::{Spinner, Spinners};
use std::fs;
use std::path::Path;
use std::sync::Mutex;

// Wrap the cache data in a Mutex
pub struct Cache {
    pub movie_embeddings: Mutex<Vec<MovieEmbedding>>,
    pub top_movies: Mutex<Vec<TopRatedMovie>>,
}
