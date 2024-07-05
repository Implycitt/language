use std::{
    io::prelude::*,
    fs::OpenOptions,
    str,
};

use serde::{Deserialize, Serialize};

use ron::{self, ser, de};

pub fn serialize(object: &impl Serialize) -> String {
    let card = ser::to_string_pretty(&object, ser::PrettyConfig::default()).unwrap();
    return card;
}

pub fn deserialize<'a, T>(data: &'a [u8]) -> T where T: Deserialize<'a> {
    let msg = str::from_utf8(data).unwrap();
    de::from_str::<T>(msg).unwrap()
}

pub fn write(inp: String) {

    let mut file = OpenOptions::new()
        .append(true)
        .create(true)
        .open("data\\words.ron")
        .expect("Failed");

    match file.write_all(inp.as_bytes()) {
        Err(why) => panic!("Couldn't write: {}", why),
        Ok(_) => println!("Works"),
    }
}

// not sure if the input function will be any use

// pub fn input() -> String {
//     let mut input = String::new();
//     io::stdin().read_line(&mut input);
//     return input.replace(['\n', '\r'], "")
// } 
