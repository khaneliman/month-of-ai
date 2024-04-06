use serde::{Deserialize, Serialize};

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

impl MovieCriteria {
    pub fn builder() -> MovieCriteriaBuilder {
        MovieCriteriaBuilder::new()
    }
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

pub struct MovieCriteriaBuilder {
    search: Option<String>,
    genre: Option<String>,
    mpaa: Option<String>,
    release_date_min: Option<String>,
    release_date_max: Option<String>,
    score_min: Option<f32>,
    score_max: Option<f32>,
    natural_language: Option<String>,
}

impl MovieCriteriaBuilder {
    pub fn new() -> Self {
        MovieCriteriaBuilder {
            search: None,
            genre: None,
            mpaa: None,
            release_date_min: None,
            release_date_max: None,
            score_min: None,
            score_max: None,
            natural_language: None,
        }
    }

    pub fn search(mut self, search: String) -> Self {
        self.search = Some(search);
        self
    }

    pub fn genre(mut self, genre: String) -> Self {
        self.genre = Some(genre);
        self
    }

    pub fn mpaa(mut self, mpaa: String) -> Self {
        self.mpaa = Some(mpaa);
        self
    }

    pub fn release_date_min(mut self, release_date_min: String) -> Self {
        self.release_date_min = Some(release_date_min);
        self
    }

    pub fn release_date_max(mut self, release_date_max: String) -> Self {
        self.release_date_max = Some(release_date_max);
        self
    }

    pub fn score_min(mut self, score_min: f32) -> Self {
        self.score_min = Some(score_min);
        self
    }

    pub fn score_max(mut self, score_max: f32) -> Self {
        self.score_max = Some(score_max);
        self
    }

    pub fn natural_language(mut self, natural_language: String) -> Self {
        self.natural_language = Some(natural_language);
        self
    }

    pub fn build(self) -> MovieCriteria {
        MovieCriteria {
            search: self.search,
            genre: self.genre,
            mpaa: self.mpaa,
            release_date_min: self.release_date_min,
            release_date_max: self.release_date_max,
            score_min: self.score_min,
            score_max: self.score_max,
            natural_language: self.natural_language,
        }
    }
}
