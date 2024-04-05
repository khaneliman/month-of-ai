use crate::model::cosine_similarity::CosineSimilarity;
use crate::model::{
    cache::Cache,
    movies::{
        movie::TopRatedMovie, movie_criteria::MovieCriteria, movie_embedding::MovieEmbedding,
    },
};
use crate::util::vector_math_helper::VectorMathHelper;
use chrono::prelude::*;
use log::debug;
use spinners::{Spinner, Spinners};
use std::{fs, path::Path, sync::Mutex};

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

pub fn find_similar_movies(
    movie_id: &str,
    movie_embeddings: &Vec<MovieEmbedding>,
) -> Vec<CosineSimilarity> {
    let movie_embedding_for_comparison = movie_embeddings
        .iter()
        .find(|x| x.movie_id.to_string() == *movie_id)
        .unwrap();

    let mut cosine_similarities = vec![];

    for movie_embedding in movie_embeddings.iter() {
        // Your existing code logic inside the loop remains the same
        if movie_embedding.movie_id != movie_embedding_for_comparison.movie_id {
            let result = VectorMathHelper::cosine_similarity(
                &movie_embedding_for_comparison
                    .embeddings
                    .as_ref()
                    .unwrap()
                    .data[0]
                    .embedding,
                &movie_embedding.embeddings.as_ref().unwrap().data[0].embedding,
            );
            cosine_similarities.push(CosineSimilarity {
                movie_id: movie_embedding.movie_id,
                similarity: result,
            });
        }
    }

    cosine_similarities
}

pub async fn filter_movies(
    criteria: MovieCriteria,
    top_movies: Vec<TopRatedMovie>,
) -> Vec<TopRatedMovie> {
    let mut filtered_movies = top_movies;

    if let Some(genre) = criteria.genre {
        let target_genres: Vec<&str> = genre.split(",").map(|g| g.trim()).collect();
        filtered_movies = filtered_movies
            .iter()
            .filter(|m| {
                target_genres
                    .iter()
                    .any(|g| m.genres.contains(&g.to_string()))
            })
            .cloned()
            .collect();
    }
    if let Some(mpaa) = criteria.mpaa {
        filtered_movies = filtered_movies
            .iter()
            .filter(|m| m.mpaa == mpaa)
            .cloned()
            .collect();
    }
    if let Some(release_date_min) = criteria.release_date_min {
        filtered_movies = filtered_movies
            .iter()
            .filter(|m| {
                Utc.datetime_from_str(&m.release_date, "%Y-%m-%d").unwrap()
                    >= Utc
                        .datetime_from_str(&release_date_min, "%Y-%m-%d")
                        .unwrap()
            })
            .cloned()
            .collect();
    }
    if let Some(release_date_max) = criteria.release_date_max {
        filtered_movies = filtered_movies
            .iter()
            .filter(|m| {
                Utc.datetime_from_str(&m.release_date, "%Y-%m-%d").unwrap()
                    <= Utc
                        .datetime_from_str(&release_date_max, "%Y-%m-%d")
                        .unwrap()
            })
            .cloned()
            .collect();
    }
    // if let Some(runtime_min) = criteria.runtime_min {
    //     filtered_movies = *filtered_movies
    //         .into_iter()
    //         .filter(|m| m.runtime >= runtime_min)
    //         .collect();
    // }
    // if let Some(runtime_max) = criteria.runtime_max {
    //     filtered_movies = *filtered_movies
    //         .into_iter()
    //         .filter(|m| m.runtime <= runtime_max)
    //         .collect();
    // }
    if let Some(score_min) = criteria.score_min {
        filtered_movies = filtered_movies
            .iter()
            .filter(|m| m.imdb_score >= score_min.into())
            .cloned()
            .collect();
    }

    if let Some(score_max) = criteria.score_max {
        filtered_movies = filtered_movies
            .iter()
            .filter(|m| m.imdb_score <= score_max.into())
            .cloned()
            .collect();
    }

    filtered_movies.to_vec()
}
