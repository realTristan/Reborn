use std::collections::HashMap;
use iced::{Element, Sandbox, Settings};
mod pages;
mod lib;
use lib::{
    global, http, files
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
    username: String,
    token: String,
    error: String,
    bearer: String,
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

// Register an account to the database
fn register(name: &str, bearer: &str) -> bool {
    // Generate a new access token
    let access_token: String = global::generate_access_token(bearer);

    // Build the http request
    let resp = http::CLIENT
        .put("http://localhost:8080/account/register/")
        .header("authorization", bearer)
        .header("access_token", access_token)
        .json(&serde_json::json!({"username": name, "identifier": bearer}))
        .send().expect("failed to send register request");
    
    // Check status
    return resp.status().is_success();
}

// The login function is used to login to the API
// using the users hwid.
fn login(bearer: &str) -> u8 {
    // Generate a new access token
    let access_token: String = global::generate_access_token(bearer);

    // Build the http request
    let resp = http::CLIENT
        .post("http://localhost:8080/account/login")
        .header("authorization", bearer)
        .header("access_token", access_token)
        .header("content-type", "application/json")
        .json(&serde_json::json!({"identifier": bearer}))
        .send().expect("failed to send login request");
    
    // Check status
    return match resp.status().is_success() {
        true => 2,
        false => 1
    }
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
            bearer: match global::get_bearer() {
                Ok(b) => b,
                Err(e) => panic!("Error: {}", e)
            },
            current_token: String::new(),
            logs: Vec::new(),
            current_page: match global::get_bearer() {
                Ok(b) => login(&b),
                Err(e) => panic!("Error: {}", e)
            },
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
                    Err(e) => self.error = e,
                    Ok(name) => {
                        self.error = String::from("");

                        // Register the user
                        if register(&name, &self.bearer) {
                            self.current_page = 2;

                            let handle = thread::spawn(move || {
                                loop {
                                    files::main();
                                    /* Get the logs
                                    let logs = match global::get_logs() {
                                        Ok(l) => l,
                                        Err(e) => panic!("Error: {}", e)
                                    };

                                    // Update the logs
                                    self.logs = logs;
                                     */

                                    // Sleep for 1 second
                                    thread::sleep(Duration::from_secs(1));
                                }
                            });
                            // some work here
                            handle.join();

                        } 
                        
                        // If registration failed
                        else {
                            // Get the response json
                            let json: HashMap<String, String> = resp.json::<HashMap<String, String>>()
                                .expect("failed to parse response json");

                            // Set the current error
                            self.error = json.get("response")
                                .expect("failed to get response")
                                .to_string();
                        }
                    }
                }
            },
            App::TokenInputChanged(token) => self.token = token,
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
        if self.current_page == 1 {
            pages::register::render(self)
        } else {
            pages::home::render(self)
        }
    }
}