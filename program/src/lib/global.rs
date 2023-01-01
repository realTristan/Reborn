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

// Get the date and time as an iso formatted timestamp
pub fn get_iso_timestamp() -> String {
    let t: chrono::DateTime<chrono::Utc> = std::time::SystemTime::now().into();
    return t.to_rfc3339();
}
