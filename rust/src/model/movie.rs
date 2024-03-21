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

#[derive(Debug, Serialize, Deserialize)]
pub struct MovieCriteria {
    pub search: Option<String>,
    pub genre: Option<String>,
    pub mpaa: Option<String>,
    pub release_date_min: Option<String>,
    pub release_date_max: Option<String>,
    pub score_min: Option<f32>,
    pub score_max: Option<f32>,
    pub natural_language: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SortCriteria {
    pub field: String,
    pub direction: Option<SortDirection>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "UPPERCASE")]
pub enum SortDirection {
    Asc,
    Desc,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Criteria {
    pub sort: Option<Vec<SortCriteria>>,
    pub page: Option<i32>,
    pub page_size: Option<i32>,
}
