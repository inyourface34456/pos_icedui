use iced::widget::{self, button, column, row, text, Column};
use iced::{Element, Length, Theme};
use crate::button_style;
use crate::msg::Message;

pub struct Display {
    to_set: String,
    error: String,
    items: Vec<String>
}

impl Default for Display {
    fn default() -> Self {
        Self {
            to_set: String::from("0"),
            error: String::new(),
            items: Vec::new(),
        }
    }
}

impl Display {
    fn update(&mut self, message: Message) {
        match message {
            Message::ToSet(num) => self.to_set = num,
            Message::AddToList => self.items.push(self.to_set.clone()),
            Message::Remove(id) => {self.items.remove(id);},
        }
    }
    
    fn view(&self) -> Element<'_, Message> {
        row![
            column![
                self.view_list()
            ],
            column![
                row![
                    text("What do you want to add to the list?"),
                    widget::text_input("0", self.to_set.as_ref())
                        .on_input(|num| Message::ToSet(num))
                        .on_submit(Message::AddToList),
                ]
                 .spacing(5),
                 text(&self.error),
            ]
        ].into()
    }

    fn view_list(&self) -> Element<'_, Message> {
        let mut item_list = Column::new()
            .spacing(5)
            .width(Length::Fill);

        for (id, item) in self.items.iter().enumerate() {
            item_list = item_list.push(
                button(
                    text(item)
                )
                .on_press(Message::Remove(id))
                .width(300)
                .style(button_style)
            );
        }

        item_list.into()
    }

    pub fn start(&self) -> Result<(), iced::Error> {
        iced::application("POS software: ALPHA", Display::update, Display::view)
            .theme(|_| Theme::Dark)
            .centered()
            .antialiasing(false)
            .run()
    }
}