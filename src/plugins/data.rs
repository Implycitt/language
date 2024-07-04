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
        .expect("Failed");

    match file.write_all(inp.as_bytes()) {
        Err(why) => panic!("Couldn't write: {}", why),
        Ok(_) => println!("Works"),
    }
}

fn construct_data() {
    let word = input();
    let desc = input();
    let new_card: Card = Card { word: word, description: desc };  
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
    let path = Path::new("data\\words.ron");
    if let Ok(lines) = read_lines(path) {
        for line in lines.flatten() {
            println!("{}", line);
        }
    }
}

fn show_help() {
    println!("commands: new, list, help, exit");
}
