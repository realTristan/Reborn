

// Create a new directory at the provided path
pub fn mkdir(path: &str) -> Result<(), std::io::Error> {
    return match std::fs::create_dir_all(path) {
        Ok(_) => Ok(()),
        Err(e) => Err(e)
    };
}

// Read the provided file via the provided path then
// return the buffer as a vector of bytes
pub fn read_file(path: &str) -> Result<Vec<u8>, std::io::Error> {
    return match std::fs::read(path) {
        Ok(f) => Ok(f),
        Err(e) => Err(e)
    }
}

// Convert the png image buffer to a base64 string
// which can be passed to the api as a readable image
pub fn encode_png(buf: Vec<u8>) -> String {
    return format!(
        "data:image/png;base64,{}", base64::encode(buf)
    )
}

// Convert the json buffer to a base64 string which
// can be passed to the api as a readable file
pub fn encode_json(buf: Vec<u8>) -> String {
    return format!(
        "data:application/json;base64,{}", base64::encode(buf)
    )
}

// Capture a screenshot and save it to the current folder
pub fn capture_image() -> Result<Vec<u8>, String> {
    let screens = match screenshots::Screen::all() {
        Some(s) => s,
        None => return Err(String::from("failed to get screens"))
    };

    // Iterate through all the screens
    for screen in screens {
        return match screen.capture() {
            Some(f) => Ok(f.buffer().to_vec()),
            None => return Err(String::from("failed to capture screen"))
        }
    }
    Ok(Vec::new())
}

