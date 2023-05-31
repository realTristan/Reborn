use crate::{lib::global, App, Page};
use iced::widget::{button, column, row, text};
use iced_native::widget::row::Row;

// Render the home page widget
pub fn render(page: &Page) -> iced::Element<App> {
    // Create the main column
    let mut col = iced::widget::column![
        header(page),
        // Token Input
        token_input(page),
        // Display the current token
        token_label(page),
        // Start and Stop buttons
        row![start_button(), stop_button()]
            .spacing(20)
            .padding([5, 30, 20, 30])
    ];

    // Push 8 test widgets
    // TODO: Replace this with the actual logs (page.logs)
    for log in &page.logs {
        col = col.push(row![text(format!("{}: {}", global::get_date_time(), log))].padding(5));
    }

    // Return the main column
    return col.align_items(iced::Alignment::Center).into();
}

// Render the current token label widget
pub fn token_label(page: &Page) -> row {
    return row![match page.current_token.len() > 0 {
        true => text(format!("Token: {}", &page.current_token)),
        false => text("").size(0),
    }]
    .padding([0, 30, 10, 30]);
}

// Render the token input widget
pub fn token_input(page: &Page) -> row {
    return row![iced::widget::TextInput::new(
        "Enter vac token here",
        &page.token,
        App::TokenInputChanged
    )
    .padding(10)
    .size(20),]
    .padding([10, 30, 10, 30]);
}

// Render the header widget
pub fn header(page: &Page) -> column {
    return column![
        text("Reborn Anti-Cheat").size(40),
        text(format!("Welcome, {0}", page.user.name)).size(20),
    ]
    .padding([30, 0, 10, 0])
    .spacing(10)
    .align_items(iced::Alignment::Center);
}

// Start the anti-cheat button
pub fn start_button() -> button {
    return button(text("Start").horizontal_alignment(iced::alignment::Horizontal::Center))
        .on_press(App::StartPressed)
        .padding(10)
        .width(iced::Length::FillPortion(15));
}

// Stop the anti-cheat button
pub fn stop_button() -> button {
    return button(text("Stop").horizontal_alignment(iced::alignment::Horizontal::Center))
        .on_press(App::StopPressed)
        .padding(10)
        .width(iced::Length::FillPortion(15));
}
