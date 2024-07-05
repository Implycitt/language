use std::fs;
use serde::{Deserialize, Serialize};
use crate::plugins::{utils};

#[derive(Debug, Deserialize, Serialize)]
pub struct Card {
    word: String,
    description: String,
}

pub fn get_data() {
    let path = "data\\words.ron";
    let data: String = fs::read_to_string(path).expect("Could not read");
    let out: Card = utils::deserialize(data.as_bytes());
    println!("{:?}", out);
}

pub fn get_word() -> String {
    todo!();
}

pub fn get_description() -> String {
    todo!();
}


