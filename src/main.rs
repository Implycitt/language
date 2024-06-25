use std::{
    io,
    io::prelude::*,
    fs::File,
    path::Path,
};

fn input() -> String {
    let mut input = String::new();
    io::stdin().read_line(&mut input);
    return input
} 

fn write(inp: String) {
    let path = Path::new("data\\words.txt");
    let display = path.display();

    let mut file = match File::create(&path) {
        Err(why) => panic!("Couldn't create {}: {}", display, why),
        Ok(file) => file,
    };

    match file.write_all(inp.as_bytes()) {
        Err(why) => panic!("Couldn't write to {}: {}", display, why),
        Ok(_) => println!("Works"),
    }
}

fn main() {
    let text = input();
    write(text);
}

