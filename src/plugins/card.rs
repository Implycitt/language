use std::fs;

use serde::{Deserialize, Serialize};

use crate::plugins::utils;

#[derive(Debug, Deserialize, Serialize)]
pub struct Card {
    pub word: String,
    pub description: String,
}

pub fn get_data() -> Card {
    let path = "data\\words.ron";
    let data: String = fs::read_to_string(path).expect("Could not read");
    utils::deserialize(data.as_bytes())
}
