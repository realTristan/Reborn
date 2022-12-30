mod lib;
use lib::{
    endpoints, handlers::Database
};
use actix_web::{
    self, web, App, HttpRequest, HttpServer, Responder, HttpResponse
};

// The default endpoint
#[actix_web::get("/")]
async fn main_endpoint(_req: HttpRequest) -> impl Responder {
    return serde_json::json!({
        "status": "200",
        "response": "Welcome to the Reborn Anti-Cheat API!"
    }).to_string();
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // Establish a connection to the database
    let db: Database = Database::init().await;

    // Establish a connection to http://127.0.0.1:8080/
    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(db.clone()))
            .wrap(actix_cors::Cors::permissive())
            
            // Send message endpoints
            .service(endpoints::discord::send_discord_message_endpoint)

            // Account endpoints
            .service(endpoints::accounts::register_user_endpoint)
            .service(endpoints::accounts::login_user_endpoint)

            // Token Endpoints
            .service(endpoints::tokens::get_token_endpoint)
            .service(endpoints::tokens::generate_token_endpoint)
            .service(endpoints::tokens::delete_token_endpoint)

    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}

// Bearer Token Verification
//
// The bearer token is just the users sha256(email:password)
// The bearer token is only used for keeping track of the access token cache
// Thus it doesn't need to be really complicated.


// HWID Spoofers
//
// When the user logs in, it will check whether they have the
// same hwid as when they first registered. If not, then it will
// send a message to the discord channel notifying the mods
// that the user has a different hwid.
//
// It will also keep track of how many unique hwids the user
// has used. This way, mods can track whether the user is using
// a hardware id spoofer. (what cheaters use to prevent being banned)


// Account Login
//
// To make sure the user doesn't just make a different account each time
// they use the anti-cheat, there will be no signout button.
//
// When the user first installs the program, it will check if an account is
// already registered with their hwid, if one is, it will automatically sign in
// using the account registered to that hwid.
//
//
// No password. Just an username, email, and hwid.
//
// When an user registers an account, if the api request is successfully, switch
// from the register page to the actual anti-cheat page. Else, display an error.
//
// When a user launches the app, it will check if an account is already registered with
// the users hwid, if so, it will send the user to the anti-cheat page.



// Discord Channel Tokens
//
// Have an algorithm for disguising the discord channel ids.
// This is so no can just use any discord channel id and make it
// send messages with the bot.
//
// Instead, an encrypted channel id will be sent to the 
// user when they need to get a vac token. Then have a way
// to decrypt the token and get access to the channel id

