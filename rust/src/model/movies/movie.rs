use serde::Deserialize;

#[derive(Deserialize, Debug)]
pub struct Movie {
    #[allow(dead_code)]
    backdrop_path: String,
    #[allow(dead_code)]
    id: String,
    #[allow(dead_code)]
    title: String,
    #[allow(dead_code)]
    poster_path: String,
    #[allow(dead_code)]
    release_date: String,
    #[allow(dead_code)]
    vote_average: f64,
    #[allow(dead_code)]
    vote_count: u32,
    #[allow(dead_code)]
    popularity: f64,
    #[allow(dead_code)]
    overview: String,
    #[allow(dead_code)]
    imdb_id: String,
    #[allow(dead_code)]
    budget: u32,
    #[allow(dead_code)]
    homepage: String,
    #[allow(dead_code)]
    revenue: u32,
    #[allow(dead_code)]
    runtime: u32,
    #[allow(dead_code)]
    tagline: String,
    #[allow(dead_code)]
    genres: Vec<String>,
    #[allow(dead_code)]
    cast: Vec<Cast>,
    #[allow(dead_code)]
    keywords: Vec<String>,
    #[allow(dead_code)]
    mpaa: String,
    #[allow(dead_code)]
    summaries: Vec<String>,
    #[allow(dead_code)]
    synopsis: String,
    #[allow(dead_code)]
    imdb_score: f64,
}

#[derive(Deserialize, Debug)]
struct Cast {
    #[allow(dead_code)]
    name: String,
    #[allow(dead_code)]
    character: String,
    #[allow(dead_code)]
    profile_path: String,
}
