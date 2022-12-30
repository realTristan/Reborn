use iced::widget::{row, button, column, text};
use iced::{Alignment, Element};
use crate::{App, Page};

// Render the home page widget
pub fn render(page: &Page) -> Element<'static, App> {
    column![
        text("Reborn Anti-Cheat").size(50),

        // Text input
        row![iced::widget::TextInput::new(
                "Enter vac token here", &page.token, App::NameInputChanged
            ).padding(10).size(20),
        ],

        // Start and Stop buttons
        row![
            button(
                text("Start").horizontal_alignment(iced::alignment::Horizontal::Center)
            ).on_press(App::StopPressed).padding(10).width(iced::Length::FillPortion(15)),
            button(
                text("Stop").horizontal_alignment(iced::alignment::Horizontal::Center)
            ).on_press(App::StopPressed).padding(10).width(iced::Length::FillPortion(15))
        ].spacing(25)
    ]
    .padding(20)
    .spacing(20)
    .align_items(Alignment::Center)
    .into()
}