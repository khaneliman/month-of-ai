use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct QueryObject {
    question: String,
}

impl QueryObject {
    pub fn question(&self) -> &str {
        &self.question
    }

    pub fn set_question(&mut self, question: String) {
        self.question = question;
    }
}
