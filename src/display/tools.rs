use iced::Theme;
use iced::widget::button::{self, Status};

pub fn button_style(_theme: &Theme, status: Status) -> button::Style {
    match status {
        Status::Active => {
            button::Style {
                background: None,
                text_color: iced::Color::WHITE,
                ..button::Style::default()
            }
        },
        Status::Hovered=> {
            button::Style {
                background: None,
                text_color: iced::Color::from_rgb(0.7, 0.7, 0.7),
                ..button::Style::default()
            }
        },
        Status::Pressed=> {
            button::Style {
                background: None,
                text_color: iced::Color::from_rgb(0.5, 0.5, 0.5),
                ..button::Style::default()
            }
        },
        Status::Disabled=> {
            button::Style {
                background: None,
                text_color: iced::Color::from_rgb(0.2, 0.2, 0.2),
                ..button::Style::default()
            }
        },
    }
}