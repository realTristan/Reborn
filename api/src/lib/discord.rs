use actix_web::web;
use super::endpoints::MessageBody;


// Define the request client as a global variable
lazy_static::lazy_static! {
    static ref DISCORD_TOKEN: String = String::new();
    static ref CLIENT: reqwest::Client = reqwest::Client::new();
}

// Discord API Documentation
// https://discord.com/developers/docs/resources/channel#create-message

// The send_message() function is used to 
// send a message to a discord channel
//
// Make sure to get the channel_id from the database
//
pub async fn send_message(channel: i64, body: web::Json<MessageBody>) -> reqwest::Response {
    return http_post(
        &format!("https://discord.com/api/v8/channels/{}/messages", channel), 
        &vec![("content", "Hello, world!")]
    ).await;
}

// the http_post() function is used to send an http
// post request to the provided url
async fn http_post(url: &str, params: &Vec<(&str, &str)>) -> reqwest::Response {
    return match CLIENT.post(url)
        .header("user-agent", "Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/108.0.0.0 Safari/537.36")
        .form(params)
        .send().await 
    {
        Ok(r) => r,
        Err(e) => panic!("failed to request provided url. {:?}", e),
    };
}