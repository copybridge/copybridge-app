use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Debug)]
pub struct Error {
    pub code: i32,
    pub title: String,
    pub message: String,
}

impl Error {
    pub fn new(title: String, message: String) -> Self {
        Self {
            code: 1,
            title,
            message,
        }
    }
}