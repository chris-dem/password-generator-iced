use std::default;

use iced::widget::text;
use iced::widget::{self, button, row};
use iced::{theme, Element};
use serde::{Deserialize, Serialize};

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub enum PasswordState {
    #[default]
    Empty,
    PasswordGenerated(String),
}

#[derive(Clone, Debug)]
pub enum PasswordMessage {
    GenPassword,
    CopyToClip,
}

impl PasswordState {
    pub fn update_password(&mut self, message: String) {
        if !message.is_empty() {
            *self = Self::PasswordGenerated(message);
        }
    }

    pub fn view(&self) -> Element<'_, PasswordMessage> {
        let right_side: Element<'_, PasswordMessage> = match self {
            PasswordState::Empty => {
                text("Press the button in order to generate a new password").into()
            }
            PasswordState::PasswordGenerated(s) => row![
                text(s.clone()),
                button("Copy to clipboard")
                    .on_press(PasswordMessage::CopyToClip)
                    .padding(10)
                    .style(theme::Button::Primary)
            ]
            .spacing(10)
            .align_items(iced::Alignment::Center)
            .into(),
        };

        row![
            button("Generate password")
                .on_press(PasswordMessage::GenPassword)
                .padding(10)
                .style(theme::Button::Primary),
            right_side,
        ]
        .spacing(10)
        .align_items(iced::Alignment::Center)
        .into()
    }
}
