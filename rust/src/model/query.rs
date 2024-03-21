use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct QuestionObject {
    pub question: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct InputObject {
    pub input: String,
}
