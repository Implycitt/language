mod plugins;

use plugins::{window, data};

fn main() {
    window::create_window();
    /*
    loop {
        let command = input();
        match command.as_str() {
            "new" => create_card(),
            "list" => display(),
            "help" => show_help(),
            "exit" => break,
            _ => continue,
        }
    }
    */
}
