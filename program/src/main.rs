use std::collections::HashMap;
use iced::{Element, Sandbox, Settings};
mod widgets;

// Define the request client as a global variable
lazy_static::lazy_static! {
    static ref SUPER_SECRET_CODE: String = String::from("SUPER_SECRET_CODE");
    static ref CLIENT: reqwest::blocking::Client = reqwest::blocking::Client::new();
}

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

// The generate_access_token function is used to generate
// a new access token for interacting with our API that
// we privated. This is used to prevent attackers from abusing 
// our API.
fn generate_access_token(bearer: &str) -> String {
    // Get the current time in secoonds
    let time: u64 = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .unwrap().as_secs();

    // Return the access_token
    return sha256::digest(format!("{}:{}:{}", 
        bearer, time, SUPER_SECRET_CODE.to_string()
    ));
}

fn get_bearer() -> Result<String, String> {
    return match std::process::Command::new("cmd")
        .args(&["/C", "wmic csproduct get uuid"])
        .output() {
            Ok(o) => Ok(sha256::digest(
                String::from_utf8_lossy(&o.stdout).trim().to_string()
            )),
            Err(e) => Err(format!("Error: {}", e))
        };
}

// The login function is used to login to the API
// using the users hwid.
fn login(bearer: &str) -> u8 {
    // Generate a new access token
    let access_token: String = generate_access_token(bearer);

    // Build the http request
    let req = CLIENT
        .post("http://localhost:8080/account/login")
        .header("authorization", bearer)
        .header("access_token", access_token)
        .header("content-type", "application/json")
        .json(&serde_json::json!({
            "identifier": bearer
        }));
    
    // Send the http request
    match req.send() {
        Ok(resp) => println!("{}: {}", resp.status(), resp.text().unwrap()),
        Err(e) => panic!("Error: {}", e)
    };
    return 1;
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
            bearer: match get_bearer() {
                Ok(b) => b,
                Err(e) => panic!("Error: {}", e)
            },
            current_token: String::new(),
            logs: Vec::new(),
            current_page: match get_bearer() {
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
                    Ok(name) => {
                        self.error = String::from("");

                        // Generate a new access token
                        let access_token = generate_access_token(&self.bearer);

                        // Build the http request
                        let req = CLIENT
                            .put("http://localhost:8080/account/register/")
                            .header("authorization", &self.bearer)
                            .header("access_token", access_token)
                            .form(&[("username", name)]);
                        
                        // Send the http request
                        match req.send() {
                            Ok(resp) => match resp.json::<HashMap<String, String>>() {
                                Ok(json) => {
                                    return match json["status"] == "200" {
                                        true => self.current_page = 2,
                                        false => self.error = json["response"].clone()
                                    };
                                },
                                Err(e) => panic!("Error: {}", e)
                            },
                            Err(e) => panic!("Error: {}", e)
                        };
                            
                    },
                    Err(e) => self.error = e
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
            widgets::register::render(self)
        } else {
            widgets::home::render(self)
        }
    }
}