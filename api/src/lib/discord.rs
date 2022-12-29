use super::endpoints::MessageBody;
use actix_web::web;

// Define the request client as a global variable
lazy_static::lazy_static! {
    static ref CLIENT: reqwest::Client = reqwest::Client::new();
}

// Define the discord token as a global variable
static DISCORD_TOKEN: &str = "";

// Discord API Documentation
// https://discord.com/developers/docs/resources/channel#create-message

// The send_message() function is used to send the
// anti-cheat data to the provided discord channel.
// The data includes the users running programs, the
// a screenshot of the users screen, the users hwid, etc.
pub async fn send_message(channel: i64, body: web::Json<MessageBody>) -> Option<reqwest::Response> {
    return match CLIENT.post(&format!("https://discord.com/api/v8/channels/{}/messages", channel))
        .header("user-agent", "Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/108.0.0.0 Safari/537.36")
        .header("authorization", format!("Bot {}", DISCORD_TOKEN))
        .form(
            &vec![("content", "Reborn API Request")]
        )
        .send().await 
    {
        Ok(r) => Some(r),
        Err(_) => None
    };
}