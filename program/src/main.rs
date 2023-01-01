use iced::{Element, Sandbox, Settings};
mod pages;
mod lib;
use lib::{
    discord,
    user::User,
    thread::Thread
};

fn main() -> iced::Result {
    Page::run(Settings {
        window: iced::window::Settings {
            size: (500, 400),
            resizable: true,
            decorations: true,
            min_size: Some((500, 400)),
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
        let _user: User = User::new();
        Self {
            current_page: 1, // _user.login()
            user: _user,
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

    // Render the window
    fn view(&self) -> Element<App> {

        // If the current page is 1, render the home page
        if self.current_page == 1 {
            pages::home::render(self)
        } 

        // Else, if the current page is 0, render the register page
        else {
            pages::register::render(self)
        }
    }

    // Handle the user input updates
    fn update(&mut self, app: App) {
        // Initialize the thread
        let mut thread: Thread = Thread::new();

        // Match app for ui updates
        match app {
            App::NameInputChanged(name) => self.user.name = name,
            App::TokenInputChanged(token) => self.token = token,
            App::RegisterPressed => match self.user.is_valid_name() {
                Err(e) => self.error = e,
                Ok(_) => match self.user.register() {
                    Err(e) => self.error = e,
                    Ok(_) => self.current_page = 1
                }
            },
            App::StartPressed => {
                println!("thread started");

                // Clone the token for thread
                thread.start(&self.user.bearer, &self.token);

                // Update variables
                self.current_token = self.token.clone();
                self.logs = Vec::new();

                // Send a start notification
                if !discord::send_start_message(&self.user.bearer, &self.token) {
                    self.logs.push(String::from("failed to send start message"));
                    return
                }
            },
            App::StopPressed => {
                println!("thread stopped");

                // Stop the thread
                thread.stop();

                // Reset variables
                self.current_token = String::new();
                self.logs = Vec::new();

                // Send a start notification
                discord::send_stop_message(&self.user.bearer, &self.token);
            },
        }
    }
}