use std::sync::Mutex;

use iced::widget::container;
use iced::widget::pane_grid::StyleSheet;
use iced::{clipboard, theme, widget, window, Alignment, Length, Vector};
use iced::{Application, Command, Theme};
use im::{vector, OrdSet};
use lazy_static::lazy_static;
use once_cell::sync;
use rand::prelude::*;
use rand::rngs::SmallRng;
use rand::seq::IteratorRandom;
use rand::SeedableRng;
const ASCII_UPPER: &'static str = "ABCDEFGHIJKLMNOPQRSTUVWXYZ";
const ASCII_LOWER: &'static str = "abcdefghijklmnopqrstuvwxyz";
const ASCII_NUMBERS: &'static str = "0123456789";
const ASCII_SYMBOLS: &'static str = "!()-.?[]_`~;:!@#$%^&*+=";
use crate::{
    filter::FilterState,
    message::Message,
    password::{PasswordMessage, PasswordState},
};

lazy_static! {
    pub static ref RNG: sync::Lazy<Mutex<SmallRng>> =
        sync::Lazy::new(|| Mutex::new(SmallRng::from_entropy()));
}

#[derive(Debug)]
pub struct State {
    pass_state: PasswordState,
    filter_state: FilterState,
}

impl Application for State {
    type Message = Message;
    type Theme = Theme;
    type Executor = iced::executor::Default;
    type Flags = ();

    fn new(_flags: Self::Flags) -> (Self, iced::Command<Self::Message>) {
        (Self::default(), Command::none())
    }

    fn title(&self) -> String {
        "Password Generator".to_string()
    }

    fn update(&mut self, message: Self::Message) -> iced::Command<Self::Message> {
        match message {
            Message::SetFilter(f) => {
                self.filter_state.update(f);
                Command::none()
            }
            Message::PasswordMessage(PasswordMessage::GenPassword) => {
                self.pass_state.update_password(self.generate_password());
                Command::none()
            }
            Message::PasswordMessage(PasswordMessage::CopyToClip) => {
                if let PasswordState::PasswordGenerated(ref s) = self.pass_state {
                    clipboard::write(s.clone())
                } else {
                    Command::none()
                }
            }
            Message::TabPressed { shift } => {
                if shift {
                    widget::focus_previous()
                } else {
                    widget::focus_next()
                }
            }
            Message::ToggleFullscreen(mode) => window::change_mode(mode),
        }
    }

    fn view(&self) -> iced::Element<'_, Self::Message, iced::Renderer<Self::Theme>> {
        let cont = widget::column![
            self.pass_state
                .view()
                .map(move |message| { Message::PasswordMessage(message) }),
            self.filter_state
                .view()
                .map(move |message| Message::SetFilter(message))
        ]
        .align_items(Alignment::Center)
        .spacing(20)
        .padding(20);

        container(cont)
            .width(Length::Fill)
            .height(Length::Fill)
            .center_x()
            .center_y()
            .into()
    }

    fn theme(&self) -> Self::Theme {
        Theme::Dark
    }
}

impl Default for State {
    fn default() -> Self {
        Self {
            pass_state: PasswordState::default(),
            filter_state: FilterState::default(),
        }
    }
}

impl State {
    fn generate_password(&self) -> String {
        let mut strs: im::Vector<_> = im::vector![];
        if self.filter_state.has_upper {
            strs.push_back(ASCII_UPPER.to_string())
        }
        if self.filter_state.has_lower {
            strs.push_back(ASCII_LOWER.to_string())
        }
        if self.filter_state.has_number {
            strs.push_back(ASCII_NUMBERS.to_string())
        }

        if self.filter_state.has_symbol {
            strs.push_back(ASCII_SYMBOLS.to_string())
        }

        let strs: im::OrdSet<char> = strs
            .into_iter()
            .chain(vec![self.filter_state.include.clone()].into_iter())
            .collect::<Vec<_>>()
            .join("")
            .chars()
            .collect();
        let s2: OrdSet<char> = im::OrdSet::from_iter(self.filter_state.exclude.chars());
        let s = strs.symmetric_difference(s2);

        let mut ret = String::new();
        if !s.is_empty() {
            let mut rng = RNG.lock().unwrap();
            ret = (0..self.filter_state.pass_len as u8)
                .map(|_| *s.iter().choose(&mut *rng).unwrap() as char)
                .collect()
        }
        ret
    }
}
