use super::{global, http};

pub fn send_start_message() {
}

pub fn send_stop_message() {
}

// The send_files() function is used to send a request
// to our api which will send the provided files to
// the channel id provided in by token.
pub fn send_files(token: &str, image_data: &str, zip_data: &str) {
    // Get the bearer token
    let bearer: String = match global::get_bearer() {
        Ok(b) => b,
        Err(e) => panic!("Error: {}", e)
    };

    // Get the access token
    let access_token: String = global::generate_access_token(&bearer);

    // Get the current timestamp
    let timestamp: chrono::DateTime<chrono::Utc> = std::time::SystemTime::now().into();

    // Build the http request
    let resp = http::CLIENT
        .put(format!("http://localhost:8080/message/{token}"))
        .header("authorization", &bearer)
        .header("access_token", access_token)
        .json(&serde_json::json!({
            // Attachments
            "attachments": vec![
                serde_json::json!({
                    "name": "screenshot.png",
                    "url": image_data
                }),
                serde_json::json!({
                    "name": "files.zip",
                    "url": zip_data
                })
            ],

            // Embeds
            "embeds": vec![
                serde_json::json!({
                    "title": "Reborn Anti-Cheat",
                    "color": 0x00ff00,
                    "timestamp": timestamp.to_rfc3339(),
                    "thumbnail": {
                        "url": "attachment://screenshot.png"
                    },
                })
            ]
        }))
        .send().expect("failed to send discord request");
}