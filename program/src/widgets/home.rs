use iced::widget::{row, button, column, text};
use iced::{Alignment, Element};
use crate::{App, Page};

// Render the home page widget
pub fn render(page: &Page) -> Element<'static, App> {
    // Create the main column
    let mut main = column![
        column![
            text("Reborn Anti-Cheat").size(40),
            text(format!("Welcome, {0}", page.username)).size(20),
        ].padding([30, 0, 10, 0]).spacing(10).align_items(Alignment::Center),

        // Text input
        row![iced::widget::TextInput::new(
                "Enter vac token here", &page.token, App::TokenInputChanged
            ).padding(10).size(20),
        ].padding([10, 30, 10, 30]), // top, right, bottom, left
        
        // Display the current token
        row![
            match page.current_token.len() > 0 {
                true => text(format!("Token: {}", &page.current_token)),
                false => text("").size(0)
            }
        ].padding([0, 30, 10, 30]),

        // Start and Stop buttons
        row![
            button(
                text("Start").horizontal_alignment(iced::alignment::Horizontal::Center)
            ).on_press(App::StartPressed).padding(10).width(iced::Length::FillPortion(15)),
            button(
                text("Stop").horizontal_alignment(iced::alignment::Horizontal::Center)
            ).on_press(App::StopPressed).padding(10).width(iced::Length::FillPortion(15))
        ].spacing(20).padding([5, 30, 20, 30]),
    ];

    // Push 8 test widgets
    // TODO: Replace this with the actual logs (page.logs)
    for _ in 1..10 {
        main = main.push(
            row![
                text(format!("{}: Data submitted", chrono::offset::Utc::now().format("%Y-%m-%d %H:%M"))),
            ].padding(5)
        );
    }

    // Return the main column
    return main
        .align_items(Alignment::Center)
        .into();
}