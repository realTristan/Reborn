use iced::{Element, Sandbox, Settings};
mod widgets;

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
    current_page: u8,
    username: String,
    token: String,
    error: String
}

// Verify the provided username
fn verify_username(name: &str) -> Result<String, String> {
    for c in name.chars() {
        if !c.is_alphanumeric() {
            return Err(String::from("Username can only contain alphanumeric characters"))
        }
    }
    if name.len() < 3 {
        return Err(String::from("Username must be at least 3 characters long"))
    }
    if name.len() > 16 {
        return Err(String::from("Username cannot be longer than 16 characters"))
    }
    Ok(name.to_string())
}

// Implementation for the Page struct
impl Sandbox for Page {
    type Message = App;

    // Set the theme to dark
    fn theme(&self) -> iced::Theme {
        iced::Theme::Dark
    }

    // Set the default values for the struct
    fn new() -> Self {
        Self {
            current_page: 1,
            token: String::new(),
            username: String::new(),
            error: String::new()
        }
    }

    // Set the title of the window
    fn title(&self) -> String {
        String::from("Reborn Anti-Cheat")
    }

    // Handle the user input updates
    fn update(&mut self, app: App) {
        match app {
            App::NameInputChanged(name) => self.username = name,
            App::RegisterPressed => {
                match verify_username(&self.username) {
                    Ok(name) => {
                        self.error = String::from("");
                        self.current_page = 2;
                    },
                    Err(e) => self.error = e
                }
            },
            App::TokenInputChanged(_) => todo!(),
            App::StartPressed => todo!(),
            App::StopPressed => todo!(),
        }
    }

    // Render the window
    fn view(&self) -> Element<App> {
        if self.current_page == 1 {
            widgets::register::render(self)
        } else {
            widgets::home::render(self)
        }
    }
}