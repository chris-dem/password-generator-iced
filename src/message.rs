use iced::{window, Application};

use crate::{filter::FilterMessage, password::PasswordMessage};

#[derive(Debug, Clone)]
pub enum Message {
    SetFilter(FilterMessage),
    PasswordMessage(PasswordMessage),
    TabPressed { shift: bool },
    ToggleFullscreen(window::Mode),
}
