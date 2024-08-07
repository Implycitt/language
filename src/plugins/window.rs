use iced::{
    executor,
    alignment::{Horizontal, Vertical},
    widget::{column, container, Container, horizontal_space, row, text, text_editor},
    {Length, Settings, Theme, Element, Command, Application},
};

use crate::plugins::card;

pub fn create_window() -> iced::Result {
    Window::run(Settings::default())
}

struct Window;

impl Application for Window {
    type Executor = executor::Default;
    type Flags = ();
    type Message = ();
    type Theme = Theme;

    fn new(_flags: ()) -> (Window, Command<Self::Message>) {
        (Window, Command::none())
    }

    fn title(&self) -> String {
        String::from("Language")
    }

    fn update(&mut self, _message: Self::Message)  -> Command<Self::Message> {
        Command::none()
    }

    fn view(&self) -> iced::Element<Self::Message> {

        let card: card::Card = card::get_data();
        let card = row![text(card.word), text(card.description)].spacing(20);
        
        container(card)
            .width(Length::Fill)
            .height(Length::Fill)
            .center_x()
            .center_y()
            .into()
    }

    fn theme(&self) -> Theme {
        Theme::Dark
    }
}
