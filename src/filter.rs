use std::default;

use iced::{
    widget::{self, checkbox, container, text, text_input},
    Element, Length,
};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FilterState {
    pub has_upper: bool,
    pub has_lower: bool,
    pub has_number: bool,
    pub has_symbol: bool,
    pub pass_len: u8,
    pub include: String,
    pub exclude: String,
}

impl Default for FilterState {
    fn default() -> Self {
        Self {
            has_upper: true,
            has_lower: true,
            has_number: false,
            has_symbol: false,
            pass_len: 8,
            include: "".to_string(),
            exclude: "".to_string(),
        }
    }
}

#[derive(Clone, Debug)]
pub enum FilterMessage {
    SetUpper(bool),
    SetLower(bool),
    SetNumber(bool),
    SetSymbols(bool),
    SetInclude(String),
    SetExclude(String),
    SetLength(u8),
}

impl FilterState {
    pub fn update(&mut self, message: FilterMessage) {
        match message {
            FilterMessage::SetUpper(b) => self.has_upper = b,
            FilterMessage::SetLower(b) => self.has_lower = b,
            FilterMessage::SetNumber(b) => self.has_number = b,
            FilterMessage::SetSymbols(b) => self.has_symbol = b,
            FilterMessage::SetInclude(s) => self.include = s,
            FilterMessage::SetExclude(s) => self.exclude = s,
            FilterMessage::SetLength(n) => self.pass_len = n,
        }
    }

    pub fn view(&self) -> Element<'_, FilterMessage> {
        let upper =
            checkbox("Uppercase", self.has_upper, FilterMessage::SetUpper).width(Length::Fill);
        let lower =
            checkbox("Lowercase", self.has_lower, FilterMessage::SetLower).width(Length::Fill);
        let number =
            checkbox("Numbers", self.has_number, FilterMessage::SetNumber).width(Length::Fill);
        let symbols =
            checkbox("Symbols", self.has_symbol, FilterMessage::SetSymbols).width(Length::Fill);
        let col1 = widget::column![upper, lower]
            .spacing(20)
            .align_items(iced::Alignment::Center);
        let col2 = widget::column![number, symbols]
            .spacing(20)
            .align_items(iced::Alignment::Center);
        let col3 = widget::column![
            text("Include characters"),
            text_input(
                "Characters to include",
                &self.include,
                FilterMessage::SetInclude
            )
            .id(text_input::Id::new("text-include"))
        ]
        .spacing(20)
        .align_items(iced::Alignment::Center);
        let col4 = widget::column![
            text("Exclude characters"),
            text_input(
                "Characters to exclude",
                &self.exclude,
                FilterMessage::SetExclude
            )
            .id(text_input::Id::new("text-exclude"))
        ]
        .spacing(20)
        .align_items(iced::Alignment::Center);
        let col5 = widget::column![
            text(format!("String length : {}", self.pass_len as u8)),
            widget::Slider::new(6..=30, self.pass_len, FilterMessage::SetLength),
        ]
        .spacing(20)
        .align_items(iced::Alignment::Center);
        widget::row![
            col1.width(Length::FillPortion(5)),
            col2.width(Length::FillPortion(5)),
            col3.width(Length::FillPortion(5)),
            col4.width(Length::FillPortion(5)),
            col5.width(Length::FillPortion(5)),
        ]
        .spacing(20)
        .padding(10)
        .align_items(iced::Alignment::Center)
        .into()
    }
}
