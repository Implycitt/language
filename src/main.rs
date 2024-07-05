mod plugins;

use plugins::{window, card};

fn main() {
    //window::create_window();
    card::get_data();
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
