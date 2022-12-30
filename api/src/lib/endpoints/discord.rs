use actix_web::{ HttpRequest, web, Responder};
use crate::lib::{
    handlers::Database, global, auth
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
    // Create empty form data array
    let mut form: Vec<(&str, String)> = Vec::new();

    // Get variables from the request body
    let hardware_info: String = body["hardware_info"].to_string();
    let image: String = body["image"].to_string();
    let embed: String = body["embed"].to_string();

    // Get the embed from the request body
    if embed.len() > 0 {
        form.push(("embeds", embed.clone()));
    }

    // Image valid image
    if image.len() > 0 {
        form.push(("file", image));
    };

    // If valid hardware info
    if hardware_info.len() > 0 {
        match base64::decode(hardware_info) {
            Ok(data) => match std::str::from_utf8(&data) {
                Ok(s) => form.push(("hardware_info", s.to_string())),
                Err(_) => return Err(())
            },
            Err(_) => return Err(())
        };
    };

    // Send the http request
    return match CLIENT.post(&format!("https://discord.com/api/v8/channels/{}/messages", channel))
        .header("user-agent", "Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/108.0.0.0 Safari/537.36")
        .header("authorization", format!("Bot {}", DISCORD_TOKEN.to_string()))
        .form(&form)
        .send().await 
    {
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
) -> impl Responder 
{

    // Get the request body
    let body: serde_json::Value = match global::get_body(&body) {
        Ok(body) => body,
        Err(_) => return serde_json::json!({
            "status": 400,
            "response": "Invalid request body"
        }).to_string()
    };

    // Get the provided authorization headers
    // Authorization: sha256("hwid")
    let auth: String = global::get_header(&req, "authorization");
    let access_token: String = global::get_header(&req, "access_token");

    // Verify the provided authorization headers
    if !auth::verify(&auth, &access_token) {
        return serde_json::json!({
            "status": 400,
            "response": "Invalid request"
        }).to_string()
    }
    
    // Get the token from the url parameters
    let token = match req.match_info().get("token") {
        Some(t) => match db.get_token(t).await {
            Some(t) => t,
            None => return serde_json::json!({
                "status": 400,
                "response": "Failed to fetch token data"
            }).to_string()
        },
        None => return serde_json::json!({
            "status": 400,
            "response": "Invalid token"
        }).to_string()
    };

    // Send the message to the discord channel via their api
    return match send_message(token.channel, body).await {
        Ok(r) => r,
        Err(_) => serde_json::json!({
            "status": 400,
            "response": "Failed to send message"
        }).to_string()
    };
}
