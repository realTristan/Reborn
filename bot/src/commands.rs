use serenity::framework::standard::macros::{command, group};
use serenity::framework::standard::CommandResult;
use serenity::model::channel::Message;
use serenity::prelude::*;

// Initialize the commands
// As a General group
#[group]
#[commands(ping)]
// This converts to commands::GENERAL_GROUP in main.rs
struct General; 

// Ping command for testing the discord bot
// This command will be removed once the code
// is production ready.
#[command]
pub async fn ping(ctx: &Context, msg: &Message) -> CommandResult {
    // Send a reply to the command author
    msg.reply(ctx, "Pong!").await?;
    // Return success value
    return Ok(())
}