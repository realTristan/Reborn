use serenity::{
    prelude::*,
    model::channel::Message,
    framework::standard::{
        macros::{command, group}, CommandResult, Args
    }
};

// Define the request client as a global variable
lazy_static::lazy_static! {
    static ref SUPER_SECRET_CODE: String = String::from("SUPER_SECRET_CODE");
    static ref CLIENT: reqwest::Client = reqwest::Client::new();
}

// Initialize the commands
// As a General group
#[group]
#[commands(ping)]
// This converts to commands::GENERAL_GROUP in main.rs
struct General; 

// Ping command for testing the bot
#[command]
pub async fn ping(ctx: &Context, msg: &Message) -> CommandResult {
    // Send a reply to the command author
    msg.reply(ctx, "Pong!").await?;
    // Return success value
    return Ok(())
}


// The generate_access_token function is used to generate
// a new access token for interacting with our API that
// we privated. This is used to prevent attackers from abusing 
// our API.
fn generate_access_token(bearer: &str) -> String {
    // Get the current time in secoonds
    let time: u64 = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .unwrap().as_secs();

    // Return the access_token
    return sha256::digest(format!("{}:{}:{}", 
        bearer, time, SUPER_SECRET_CODE.to_string()
    ));
}

// The option_create_token function is used to create a new
// token for the provided bearer. This token is used in the VAC program
// to send discord messages.
async fn option_create_token(bearer: &str, access_token: &str, msg: &Message, ctx: &Context) {
    let resp = reqwest::Client::new()
        .post("http://localhost:8080/api/v1/token")
        .header("Authorization", bearer)
        .header("access_token", access_token)
        .send().await;

    // Check if the response was successful
    match resp {
        Ok(resp) => {
            let _ = match resp.text().await {
                Ok(body) => msg.reply(ctx, format!("New token: {}", body)).await,
                Err(e) => msg.reply(ctx, format!("An error has occurred: {}", e)).await
            };
        },
        Err(e) => {
            let _ = msg.reply(ctx, format!("An error has occurred: {}", e)).await;
        }
    }
}

// The option_delete_token function is used to delete a token
// for the provided bearer.
async fn option_delete_token(bearer: &str, access_token: &str, token: &str, msg: &Message, ctx: &Context) {
    let resp = reqwest::Client::new()
        .delete(format!("http://localhost:8080/api/v1/token/{token}"))
        .header("Authorization", bearer)
        .header("access_token", access_token)
        .send().await;

    // Check if the response was successful
    match resp {
        Ok(resp) => {
            let _ = match resp.text().await {
                Ok(body) => msg.reply(ctx, format!("Deleted token: {}", body)).await,
                Err(e) => msg.reply(ctx, format!("An error has occurred: {}", e)).await
            };
        },
        Err(e) => {
            let _ = msg.reply(ctx, format!("An error has occurred: {}", e)).await;
        }
    }
}

// The option_show_token function is used to show the data
// for a token for the provided bearer.
async fn option_show_token(bearer: &str, access_token: &str, token: &str, msg: &Message, ctx: &Context) {
    let resp = reqwest::Client::new()
        .get(format!("http://localhost:8080/api/v1/token/{token}"))
        .header("Authorization", bearer)
        .header("access_token", access_token)
        .send().await;

    // Check if the response was successful
    match resp {
        Ok(resp) => {
            let _ = match resp.text().await {
                Ok(body) => msg.reply(ctx, format!("Token info: {}", body)).await,
                Err(e) => msg.reply(ctx, format!("An error has occurred: {}", e)).await
            };
        },
        Err(e) => {
            let _ = msg.reply(ctx, format!("An error has occurred: {}", e)).await;
        }
    }
}


#[command]
pub async fn token(ctx: &Context, msg: &Message, mut args: Args) -> CommandResult {
    // Get the option argument (the first argument)
    let option: String = args.single::<String>().unwrap();

    // Get the bearer token for the message author
    let bearer: String = sha256::digest(msg.author.id.0.to_string());

    // Get the access token for the message author
    let access_token: String = generate_access_token(&bearer);

    // Create a new token
    if option == "new" || option == "create" {
        option_create_token(&bearer, &access_token, msg, ctx).await;
    }
    
    // Delete the provided token
    else if option == "delete" || option == "remove" {
        let token: String = args.single::<String>().unwrap();
        option_delete_token(&bearer, &access_token, &token, msg, ctx).await;
    } 
    
    // Show the provided token's data
    else if option == "info" || option == "show" {
        let token: String = args.single::<String>().unwrap();
        option_show_token(&bearer, &access_token, &token, msg, ctx).await;
    } 

    // Else, respond with an error message
    else {
        msg.reply(ctx, "Invalid option. Please use `new`, `delete`, or `list`.").await?;
    }

    // Return success value
    return Ok(())
}