use actix_web::{ HttpRequest, web, HttpResponse};
use crate::lib::{
    handlers::Database, http, auth
};

// Define the request client as a global variable
lazy_static::lazy_static! {
    static ref DISCORD_TOKEN: String = std::env::var("DISCORD_TOKEN").unwrap();
    static ref CLIENT: reqwest::Client = reqwest::Client::new();
}

// Discord API Documentation
// https://discord.com/developers/docs/resources/channel#create-message

// The send_message() function is used to send the
// anti-cheat data to the provided discord channel.
// The data includes the users running programs, the
// a screenshot of the users screen, the users hwid, etc.
async fn send_message(channel: i64, body: serde_json::Value) -> Result<String, ()> {
    let timestamp: chrono::DateTime<chrono::Utc> = std::time::SystemTime::now().into();

    let req = CLIENT
        .post(&format!("https://discord.com/api/v8/channels/{}/messages", channel))
        .header("user-agent", "Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/108.0.0.0 Safari/537.36")
        .header("authorization", format!("Bot {}", DISCORD_TOKEN.to_string()))
        .header("content-type", "application/json")
        .form(&vec![
            ("attachments", serde_json::json!({
                "filename": "screenshot.png",
                "url": body.get("image").unwrap().to_string()
            })),

            // use body.get(embeds) this is just for testing right now.
            ("embeds", serde_json::json!({
                "title": "Reborn Anti-Cheat",
                "color": 0x00ff00,
                "timestamp": timestamp.to_rfc3339(),
                "thumbnail": {
                    "url": "attachment://screenshot.png"
                },
            }))
        ]);


    /*
    
    // Get the embed from the request body
    match body.get("embed") {
        Some(embed) => req.body(json!({ "embeds": embed })),
        None => ()
    };

    */


    // Get the zip file from the request body
    // and decode it from base64 then set it as the
    // request body for the outgoing request.
    let req = match body.get("zip_file") {
        Some(zip) => match base64::decode(zip.to_string()) {
            Ok(zip) => req.body(zip),
            Err(_) => req
        },
        None => req
    };

    // Send the http request
    return match req.send().await {
        Ok(r) => match r.text().await {
            Ok(t) => Ok(t),
            Err(_) => Err(())
        },
        Err(_) => Err(())
    };
}

// The send message to a channel with the reborn
// discord bot endpoint. This is different from the
// original Versa Anti Cheat which used a discord webhook.
// Discord webhooks are not secure and can be easily tampered with
// thus having our own private api and using a discord bot is
// much more secure.
#[actix_web::post("/message/{token}")]
async fn send_discord_message_endpoint(
    req: HttpRequest, db: web::Data<Database>, body: web::Bytes
) -> HttpResponse 
{

    // Get the request body
    let body: serde_json::Value = match http::body(&body) {
        Ok(body) => body,
        Err(_) => return http::response(
            http::Status::BAD_REQUEST,
            serde_json::json!({
                "response": "Invalid request body"
            })
        )
    };

    // Get the provided authorization headers
    // Authorization: sha256("hwid")
    let auth: String = http::header(&req, "authorization");
    let access_token: String = http::header(&req, "access_token");

    // Verify the provided authorization headers
    if !auth::verify(&auth, &access_token) {
        return http::response(
            http::Status::BAD_REQUEST,
            serde_json::json!({
                "response": "Invalid request"
            })
        )
    }
    
    // Get the token from the url parameters
    let token = match req.match_info().get("token") {
        Some(t) => match db.get_token(t).await {
            Some(t) => t,
            None => return http::response(
                http::Status::BAD_REQUEST,
                serde_json::json!({
                    "response": "Failed to fetch token"
                })
            )
        },
        None => return http::response(
            http::Status::BAD_REQUEST,
            serde_json::json!({
                "response": "Invalid token"
            })
        )
    };

    // Send the message to the discord channel via their api
    return match send_message(token.channel, body).await {
        Ok(r) => http::response(
            http::Status::OK,
            serde_json::Value::String(r)
        ),
        Err(_) => http::response(
            http::Status::BAD_REQUEST,
            serde_json::json!({
                "response": "Failed to send message"
            })
        )
    };
}
