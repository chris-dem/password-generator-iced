// use crate::message::Message;
// use iced::{
//     widget::{self, checkbox, row},
//     Element,
// };
// use im::{vector, Vector};
// use lazy_static::lazy_static;
// PNGVQSHU
// lazy_static! {
//     static ref FILTER_OPTIONS: Vector<&'static str> =
//         vector!["Uppercase", "Lowercase", "Numbers", "Symbols"];
// }

// fn view_controls() -> Element<'static, Message> {
//     let col1: Element<Message> =
//         widget::column![checkbox("Uppercase", false, |b| Message::SetFilter(
//             crate::filter::Filter::Uppercase(b)
//         ))]
//         .into();
//     row![col1].into()
// }
