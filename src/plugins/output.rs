use std::{
    io::prelude::*,
    io::{self, BufRead},
    fs::{File, OpenOptions, self},
    path::Path,
};

use serde::{ Deserialize, Serialize };

use ron::{ self, ser, de };

//use plugins::data;

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>> where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

pub fn get_data() {
    let path = "data\\words.ron";
    let data: String = fs::read_to_string(path).expect("Could not read");
    println!("{:?}", de::from_str(&data));
}

pub fn get_word() -> String {
    todo!();
}

pub fn get_description() -> String {
    todo!();
}


