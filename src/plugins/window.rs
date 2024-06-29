use iced::{ Element, Sandbox, Settings };

pub fn create_window() -> iced::Result {
    Window::run(Settings::default())
}

struct Window;

impl Sandbox for Window {
    type Message = ();

    fn new() -> Window {
        Window
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

