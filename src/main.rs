use std::{
    io,
    io::{BufWriter, Write},
    fs::File
};

fn input() -> String {
    let mut input = String::new();
    io::stdin().read_line(&mut input);
    return input
} 

fn write() -> std::io::Result<()> {
    let file = File::create("D:\\dev\\Projects\\language\\src\\words.txt")?;
    let mut writer = BufWriter::new(file);
    writer.write_all(b"man")?;
    writer.flush()?;
    Ok(())
}

fn main() {
    write();
}
