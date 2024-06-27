use std::{
    io::prelude::*,
    io::{self, BufRead},
    fs::{File, OpenOptions},
    path::Path,
};

use serde::{ Deserialize, Serialize };

use ron::{ self, ser };

#[derive(Debug, Deserialize, Serialize)]
struct Card {
    word: String,
    description: String,
}

fn input() -> String {
    let mut input = String::new();
    io::stdin().read_line(&mut input);
    return input.replace(['\n', '\r'], "")
} 

fn write(inp: String) {

    let mut file = OpenOptions::new()
        .append(true)
        .create(true)
        .open("data\\words.ron")
        .expect("create failed");

    match file.write_all(inp.as_bytes()) {
        Err(why) => panic!("Couldn't write: {}", why),
        Ok(_) => println!("Works"),
    }
}

fn construct_data() {
    let mut word = input();
    let mut desc = input();
    let mut new_card: Card = Card { word: word, description: desc };  
    let output = ser::to_string_pretty(&new_card, ser::PrettyConfig::default()).unwrap();
    write(output);
}

fn create_card() {
    construct_data();
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>> where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

fn display() {
    let path = Path::new("data\\words.txt");
    if let Ok(lines) = read_lines(path) {
        for line in lines.flatten() {
            println!("{}", line);
        }
    }
}

fn main() {
    loop {
        let command = input();
        match command.as_str() {
            "new" => create_card(),
            "exit" => break,
            _ => continue,
        }
    }
}
