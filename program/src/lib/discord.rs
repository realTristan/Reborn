use super::{global, http};

// Send an embed to the discord channel notifying
// the staff that this user has started the anti-cheat
pub fn send_start_message(bearer: &str, token: &str) -> bool {
    let access_token: String = global::generate_access_token(bearer);

    // Build the http request
    let resp = http::CLIENT
        .put(format!("http://localhost:8080/message/{token}"))
        .header("authorization", bearer)
        .header("access_token", access_token)
        .json(&serde_json::json!({
            "embeds": vec![
                serde_json::json!({
                    "title": "Reborn Anti-Cheat",
                    "description": "User has started the anti-cheat.",
                    "color": 0x00ff00,
                    "timestamp": global::get_iso_timestamp()
                })
            ]
        }))
        .send();

    // Return whether the request was a success
    match resp {
        Ok(r) => r.status().is_success(),
        Err(_) => false
    }
}

// Send an embed to the discord channel notifying
// the staff that this user has stopped the anti-cheat
pub fn send_stop_message(bearer: &str, token: &str) -> bool {
    let access_token: String = global::generate_access_token(bearer);

    // Build the http request
    let resp = http::CLIENT
        .put(format!("http://localhost:8080/message/{token}"))
        .header("authorization", bearer)
        .header("access_token", access_token)
        .json(&serde_json::json!({
            "embeds": vec![
                serde_json::json!({
                    "title": "Reborn Anti-Cheat",
                    "description": "User has stopped the anti-cheat!",
                    "color": 0x00ff00,
                    "timestamp": global::get_iso_timestamp()
                })
            ]
        }))
        .send();
    
    // Return whether the request was a success
    match resp {
        Ok(r) => r.status().is_success(),
        Err(_) => false
    }
}

// The send_files() function is used to send a request
// to our api which will send the provided files to
// the channel id provided in by token.
pub fn send_files(
    bearer: &str, token: &str, image_data: &str, sysinfo_data: &str
) -> bool {
    // Generate a new access token
    let access_token: String = global::generate_access_token(bearer);

    // Build the http request
    let resp = http::CLIENT
        .put(format!("http://localhost:8080/message/{token}"))
        .header("authorization", bearer)
        .header("access_token", access_token)
        .json(&serde_json::json!({
            // Attachments
            "attachments": vec![
                serde_json::json!({
                    "name": "screenshot.png",
                    "url": image_data
                }),
                serde_json::json!({
                    "name": "sysinfo.txt",
                    "url": sysinfo_data
                })
            ],

            // Embeds
            "embeds": vec![
                serde_json::json!({
                    "title": "Reborn Anti-Cheat",
                    "color": 0x00ff00,
                    "timestamp": global::get_iso_timestamp(),
                    "thumbnail": {
                        "url": "attachment://screenshot.png"
                    },
                })
            ]
        }))
        .send();

    // Return whether the request was a success
    match resp {
        Ok(r) => r.status().is_success(),
        Err(_) => false
    }
}