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

pub fn can_load_data(cache: &Mutex<Cache>) -> bool {
    let current_directory = std::env::current_dir().unwrap();

    let movie_embeddings_path = current_directory.join("src/data/embeddings.json");
    let top_movies_path = current_directory.join("src/data/topRatedMovies.json");

    if Path::new(&movie_embeddings_path).exists() && Path::new(&top_movies_path).exists() {
        debug!("Loading data from cache or disk...");
        let mut sp = Spinner::new(
            Spinners::Dots9,
            "\t\tLoading data from cache or disk...".into(),
        );

        let cache_lock = cache.lock().unwrap();
        let mut movie_embeddings_lock = cache_lock.movie_embeddings.lock().unwrap();
        if movie_embeddings_lock.is_empty() {
            let movie_embeddings_json_content = fs::read_to_string(&movie_embeddings_path).unwrap();
            let data: Vec<MovieEmbedding> =
                serde_json::from_str(&movie_embeddings_json_content).unwrap();
            *movie_embeddings_lock = data;
        }

        let mut top_movies_lock = cache_lock.top_movies.lock().unwrap();
        if top_movies_lock.is_empty() {
            let top_movies_json_content = fs::read_to_string(&top_movies_path).unwrap();
            let data: Vec<TopRatedMovie> = serde_json::from_str(&top_movies_json_content).unwrap();
            *top_movies_lock = data;
        }

        debug!("Loaded data from cache or disk");
        sp.stop();

        true
    } else {
        false
    }
}
