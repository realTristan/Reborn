// Define global variables
lazy_static::lazy_static! {
    static ref SUPER_SECRET_CODE: String = String::from("SUPER_SECRET_CODE");
}

// The get_unix_time() function is used to quickly
// and cleanly get the time in seconds since
// the unix epoch.
pub fn get_unix_time() -> std::time::Duration {
    return std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .unwrap();
}

// Get the current date and time as a formatted string
pub fn get_date_time() -> String {
    return chrono::offset::Utc::now()
        .format("%Y-%m-%d:%H:%M:%S")
        .to_string()
}

// The generate_access_token function is used to generate
// a new access token for interacting with our API that
// we privated. This is used to prevent attackers from abusing 
// our API.
pub fn generate_access_token(bearer: &str) -> String {
    // Get the current time in secoonds
    let time: u64 = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .unwrap().as_secs();

    // Return the access_token
    return sha256::digest(format!("{}:{}:{}", 
        bearer, time, SUPER_SECRET_CODE.to_string()
    ));
}

// The get_bearer function is used to get the users
// hardware id. This is used to identify the user
// for sending http requests to our privated api.
pub fn get_bearer() -> Result<String, String> {
    return match std::process::Command::new("cmd")
        .args(&["/C", "wmic csproduct get uuid"])
        .output() {
            Ok(o) => Ok(sha256::digest(
                String::from_utf8_lossy(&o.stdout).trim().to_string()
            )),
            Err(e) => Err(format!("Error: {}", e))
        };
}