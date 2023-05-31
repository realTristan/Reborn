use crate::{App, Page};
use iced::widget::{row, text, button, TextInput};

// Render the register page widget
pub fn render(page: &Page) -> iced::Element<App> {
    iced::widget::column![
        // Header
        header(),
        // Username input
        name_input(page),
        // If there's an error in the provided username, display it here
        error_label(page),
        // Display the username that is bound to the hardware
        warning_message(),
        // Register Button
        register_button()
    ]
    .padding(20)
    .spacing(10)
    .align_items(iced::Alignment::Center)
    .into()
}

// Header. This is the title of the page
pub fn header() -> row {
    return row![text("Reborn Anti-Cheat").size(40)].padding(10)
}

// Error label. This shows when the user provides an invalid username
// or when the server returns an error
pub fn error_label(page: &Page) -> text::Text {
    return match page.error.len() > 0 {
        true => text(&page.error).size(15),
        false => text("").size(0),
    };
}

// Warning message. This is shown to notify the user that the username
// they provide is bound to their hardware and cannot be changed.
pub fn warning_message() -> row {
    return row![
        text("The username you provide is bound to your hardware and cannot be changed.")
            .horizontal_alignment(iced::alignment::Horizontal::Center)
            .size(20)
    ]
    .padding(10);
}

// Register button. This is used to register the user with the name
// they provided.
pub fn register_button() -> row {
    return row![button(
        text("Register").horizontal_alignment(iced::alignment::Horizontal::Center)
    )
    .on_press(App::RegisterPressed)
    .padding(15)
    .width(iced::Length::FillPortion(15))]
    .padding(20);
}

// Username input. This is used to get the username from the user.
// Their name will be sent to the server.
pub fn name_input(page: &Page) -> row {
    return row![TextInput::new(
        "Enter username here",
        &page.user.name,
        App::NameInputChanged
    )
    .padding(10)
    .size(20),]
    .padding(10)
}