use iced::widget::{self, button, column, row, text, Column};
use iced::Theme;
use crate::msg::Message;

pub struct State {
    value: i64,
    to_set: String,
    error: String,
}

impl Default for State {
    fn default() -> Self {
        Self {
            value: 0,
            to_set: String::from("0"),
            error: String::new(),
        }
    }
}

impl State {
    fn update(&mut self, message: Message) {
        match message {
            Message::Increment => self.value += 1,
            Message::Decrment => self.value -= 1,
            Message::ToSet(num) => self.to_set = num,
            Message::SetSig => {
                match self.to_set.parse::<i64>() {
                    Ok(num) => {
                        self.value = num;
                        self.error = String::new();
                    },
                    Err(e) => self.error = e.to_string(),
                }
            }
        }
    }
    
    fn view(&self) -> Column<Message> {
        column![
            text(self.value),
            row![
                button("+").on_press(Message::Increment),
                button("-").on_press(Message::Decrment),
            ]
                .spacing(10),

            row![
                text("What do you want to change it to?"),
                widget::text_input("0", self.to_set.as_ref())
                    .on_input(|num| Message::ToSet(num))
                    .on_submit(Message::SetSig),
            ]
             .spacing(5),
             text(&self.error),

        ]
    }

    pub fn start(&self) -> Result<(), iced::Error> {
        iced::application("A counter", State::update, State::view)
            .theme(|_| Theme::Dark)
            .centered()
            .antialiasing(false)
            .run()
    }
}