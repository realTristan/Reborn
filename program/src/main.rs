use iced::{Element, Sandbox, Settings};
mod pages;
mod lib;

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
    user: lib::user::User,
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
        let _user = lib::user::User::new();
        Self {
            current_page: _user.login(),
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
                self.current_token = self.token.clone();
                self.logs = Vec::new();
                
                // Clone the token for thread
                let token: String = self.token.clone();

                // Start the main thread
                match std::thread::spawn(move || {main_loop(&token);}).join() {
                    Ok(_) => (),
                    Err(_) => self.error = String::from("failed to start main thread")
                }
            },
            App::StopPressed => {
                self.current_token = String::new();
                self.logs = Vec::new();
            },
        }
    }
}

// Main thread loop
fn main_loop(token: &str) {
    let mut sys = lib::system::System::new();
    let mut zip = lib::zip::Zip::new();
    loop {
        // Start after 10 seconds
        std::thread::sleep(std::time::Duration::from_secs(10));

        // Capture the image then add it to the zip file
        let img_buf: Vec<u8> = match lib::files::capture_image() {
            Ok(f) => f,
            Err(e) => panic!("Error: {}", e)
        };
        match zip.add_file("screenshot.png", &img_buf) {
            Ok(_) => (),
            Err(e) => panic!("Error: {}", e)
        }
        
        // Get the system info then add it to a file
        let sys_info = match serde_json::to_vec(&sys.info()){
            Ok(buf) => buf,
            Err(e) => panic!("Error: {}", e)
        };
        match zip.add_file("sysinfo.json", &sys_info) {
            Ok(_) => (),
            Err(e) => panic!("Error: {}", e)
        }

        // Encode the image data
        let image_data: String = lib::files::encode_png(img_buf);
        let sysinfo_data: String = lib::files::encode_json(sys_info);

        // Send the files to discord
        lib::discord::send_files(token, &image_data, &sysinfo_data);

        // Repeat after 20 to 50 seconds
        std::thread::sleep(std::time::Duration::from_secs(50));
    }
}