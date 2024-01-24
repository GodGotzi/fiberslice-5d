use iced::executor;
use iced::font::{self, Font};
use iced::widget::{checkbox, column, container, text};
use iced::{Application, Command, Element, Length, Settings, Theme};

mod application;

#[derive(Default)]
struct Instance {
    default_checkbox: bool,
    default_checkbox2: bool,
}

#[derive(Debug, Clone, Copy)]
enum Message {
    DefaultChecked(bool),
    DefaultChecked2(bool),
}

impl Application for Instance {
    type Message = Message;
    type Flags = ();
    type Executor = executor::Default;
    type Theme = Theme;

    fn new(_flags: Self::Flags) -> (Self, Command<Message>) {
        (Self::default(), Command::none())
    }

    fn title(&self) -> String {
        String::from("Checkbox - Iced")
    }

    fn update(&mut self, message: Message) -> Command<Message> {
        match message {
            Message::DefaultChecked(value) => self.default_checkbox = value,
            Message::DefaultChecked2(value) => self.default_checkbox2 = value,
        }

        Command::none()
    }

    fn view(&self) -> Element<Message> {
        let default_checkbox = checkbox("Default", self.default_checkbox, Message::DefaultChecked);
        let default_checkbox2 =
            checkbox("Default2", self.default_checkbox2, Message::DefaultChecked2);
        let content = column![default_checkbox, default_checkbox2].spacing(22);

        container(content)
            .width(Length::Fill)
            .height(Length::Fill)
            .center_x()
            .center_y()
            .into()
    }
}
