use actix_web::HttpRequest;

// The get_header() function is used to bypass
// any invalid header errors.
pub fn get_header(req: &HttpRequest, key: &str) -> String {
    return match req.headers().get(key) {
        Some(v) => v,
        None => return "".to_string(),
    }.to_str().unwrap().to_string();
}

// The get_time() function is used to quickly
// and cleanly get the time in seconds since
// the unix epoch.
pub fn get_time() -> std::time::Duration {
    let time: std::time::Duration = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .unwrap();
    return time;
}

// The generate_new_id() function is used to generate
// a unique hash using the provided identifier (class_id, bearer, etc.)
// and the current time in nanoseconds.
pub fn generate_new_id(identifier: &str) -> String {
    // Get the current time since epoch. This duration is later converted
    // into nanoseconds to ensure that the class hash is 100% unique.
    let time: std::time::Duration = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .unwrap();
    // Generate a new hash using the provided
    // class hash, and the current time as nanoseconds.
    return sha256::digest(format!("{}:{}", identifier, time.as_nanos()));
}
