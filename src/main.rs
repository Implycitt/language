use std::{
    io::prelude::*,
    io::{self, BufRead},
    fs::{File, OpenOptions},
    path::Path,
};


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

fn create_card() {
    let mut text = input();
    write(text);
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
