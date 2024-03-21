use serde::{Deserialize, Serialize};

#[derive(Deserialize, Debug)]
pub struct Movie {
    backdrop_path: String,
    id: String,
    title: String,
    poster_path: String,
    release_date: String,
    vote_average: f64,
    vote_count: u32,
    popularity: f64,
    overview: String,
    imdb_id: String,
    budget: u32,
    homepage: String,
    revenue: u32,
    runtime: u32,
    tagline: String,
    genres: Vec<String>,
    cast: Vec<Cast>,
    keywords: Vec<String>,
    mpaa: String,
    summaries: Vec<String>,
    synopsis: String,
    imdb_score: f64,
}

#[derive(Deserialize, Debug)]
struct Cast {
    name: String,
    character: String,
    profile_path: String,
}
