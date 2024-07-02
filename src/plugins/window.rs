use iced::{
    alignment::{Horizontal, Vertical},
    widget::{column, container, Container},
    Length, Sandbox, Settings, Theme,
};

pub fn create_window() -> iced::Result {
    Window::run(Settings::default())
}

struct Window;

impl Sandbox for Window {
    type Message = ();

    fn new() -> Self {
        Self
    }

    fn title(&self) -> String {
        String::from("Language")
    }

    fn update(&mut self, _message: Self::Message) {
        todo!();
    }

    fn view(&self) -> iced::Element<Self::Message> {
        column![
            Container::new("Here we go")
                .width(Length::Fill)
                .align_x(Horizontal::Center),
        ].into()
    }

    fn theme(&self) -> Theme {
        Theme::Dark
    }
}
