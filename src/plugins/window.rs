use iced::{ Element, Sandbox, Settings };

pub fn create_window() -> iced::Result {
    Hello::run(Settings::default())
}

struct Hello;

impl Sandbox for Hello {
    type Message = ();

    fn new() -> Hello {
        Hello
    }

    fn title(&self) -> String {
        String::from("app")
    }

    fn update(&mut self, _message: Self::Message) {
        todo!();
    }

    fn view(&self) -> Element<Self::Message> {
        "Hello, World".into()
    }

}
