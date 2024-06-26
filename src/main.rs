use std::{
    io::prelude::*,
    io::{self, BufRead},
    fs::File,
    path::Path,
};

fn input() -> String {
    let mut input = String::new();
    io::stdin().read_line(&mut input);
    return input.replace(['\n', '\r'], "")
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
            "new" => write(text),
            "exit" => break,
            _ => continue,
        }
    }
}

