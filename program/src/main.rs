use std::collections::HashMap;
use iced::{Element, Sandbox, Settings};
mod pages;
mod lib;
use lib::{
    global, http, files, thread, user::User
};

fn main() -> iced::Result {
    Page::run(Settings {
        window: iced::window::Settings {
            size: (500, 400),
            resizable: false,
            decorations: true,
            min_size: None,
            max_size: None,
            visible: true,
            transparent: false,
            always_on_top: false,
            icon: None,
            position: iced::window::Position::Centered,
        },
        ..Default::default()
    })
}

// Handle User Input Changes
#[derive(Debug, Clone)]
pub enum App {
    NameInputChanged(String),
    TokenInputChanged(String),
    RegisterPressed,
    StartPressed,
    StopPressed,
}

// Page struct
pub struct Page {
    current_token: String,
    logs: Vec<String>,
    current_page: u8,
    token: String,
    error: String,
    user: User,
}

// Implementation for the Page struct
impl Sandbox for Page {
    type Message = App;

    // Set the theme to dark
    fn theme(&self) -> iced::Theme {
        return iced::Theme::Dark
    }

    // Set the default values for the struct
    fn new() -> Self {
        let _user = User::new();
        Self {
            user: _user,
            current_page: _user.login(),
            current_token: String::new(),
            token: String::new(),
            error: String::new(),
            logs: Vec::new(),
        }
    }

    // Set the title of the window
    fn title(&self) -> String {
        return String::from("Reborn Anti-Cheat")
    }

    // Handle the user input updates
    fn update(&mut self, app: App) {
        match app {
            App::NameInputChanged(name) => self.user.name = name,
            App::TokenInputChanged(token) => self.token = token,
            App::RegisterPressed => self.register_button_pressed(),
            App::StartPressed => {
                self.current_token = self.token.clone();
                self.logs = Vec::new();
            },
            App::StopPressed => {
                self.current_token = String::new();
                self.logs = Vec::new();
            },
        }
    }

    // Render the window
    fn view(&self) -> Element<App> {

        // If the current page is 1, render the register page
        if self.current_page == 1 {
            pages::register::render(self)
        } 

        // Else, if the current page is 2, render the home page
        else if self.current_page == 2 {
            pages::home::render(self)
        }
    }

    // Register button callback function
    fn register_button_pressed(&mut self) {
        match user.is_valid_name() {
            Err(e) => self.error = e,
            Ok(name) => match user.register(&name, &self.bearer) {
                Ok(_) => {
                    self.current_page = 2;
                    thread::spawn(move || { thread::main_loop(); }).join();
                },
                Err(e) => self.error = e
            }
        }
    }
}