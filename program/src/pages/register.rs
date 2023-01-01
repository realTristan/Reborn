use iced::widget::{row, button, column, text};
use iced::{Alignment, Element};
use crate::{App, Page};

// Render the register page widget
pub fn render(page: &Page) -> Element<'static, App> {
    column![
        row![text("Reborn Anti-Cheat").size(40)].padding(10),

        // Username input
        row![iced::widget::TextInput::new(
                "Enter username here", &page.user.name, App::NameInputChanged
            ).padding(10).size(20),
        ].padding(10),
        
        // If there's an error in the provided username, display it here
        match page.error.len() > 0 {
            true => text(&page.error).size(15),
            false => text("").size(0)
        },

        // Display the username that is bound to the hardware
        row![
            text("The username you provide is bound to your hardware and cannot be changed.")
                .horizontal_alignment(iced::alignment::Horizontal::Center)
                .size(20)
        ].padding(10),

        // Register Button
        row![button(
                text("Register").horizontal_alignment(iced::alignment::Horizontal::Center)
            ).on_press(App::RegisterPressed).padding(15).width(iced::Length::FillPortion(15))
        ].padding(20)
    ]
    .padding(20)
    .spacing(10)
    .align_items(Alignment::Center)
    .into()
}