use actix_web::{ HttpRequest, web, Responder};
use crate::lib::{
    handlers::Database, global, auth, structs::MessageBody
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
async fn send_message(channel: i64, body: web::Json<MessageBody>) -> Result<String, ()> {
    // Create the form data
    let mut form: Vec<(&str, String)> = Vec::new();
    if body.image.len() > 0 {
        form.push(("file", body.image.clone()));
    };
    if body.hardware_info.len() > 0 {
        match base64::decode(&body.hardware_info) {
            Ok(data) => match std::str::from_utf8(&data) {
                Ok(s) => form.push(("hardware_info", s.to_string())),
                Err(_) => return Err(())
            },
            Err(_) => return Err(())
        };
    };
    if body.embed.len() > 0 {
        form.push(("embeds", body.embed.clone()));
    }

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
#[actix_web::post("/message/{token}/")]
async fn send_discord_message_endpoint(
    req: HttpRequest, db: web::Data<Database>, body: web::Json<MessageBody>
) -> impl Responder 
{

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
