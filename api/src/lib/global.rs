// The get_time() function is used to quickly
// and cleanly get the time in seconds since
// the unix epoch.
pub fn get_time() -> std::time::Duration {
    return std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .unwrap();
}

// The generate_new_id() function is used to generate
// a unique hash using the provided identifier (class_id, bearer, etc.)
// and the current time in nanoseconds.
pub fn generate_new_id(identifier: &str) -> String {
    // Get the current time since epoch. This duration is later converted
    // into nanoseconds to ensure that the class hash is 100% unique.
    return sha256::digest(format!(
        "{}:{}", identifier, get_time().as_nanos()
    ));
}
