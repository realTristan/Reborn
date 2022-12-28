use actix_web::{self, web, App, HttpRequest, HttpServer, Responder};
mod lib;
use lib::{database, endpoints};

// The default endpoint
#[actix_web::get("/")]
async fn main_endpoint(_req: HttpRequest) -> impl Responder {
    return "Welcome to the Reborn API!"
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // Establish a connection to the database
    let db: database::Database = database::Database::init().await;

    // Establish a connection to http://127.0.0.1:8080/
    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(db.clone()))
            .wrap(actix_cors::Cors::permissive())
            .service(main_endpoint)
            
            // Send message endpoints
            .service(endpoints::send_message_endpoint)

            // Account endpoints
            .service(endpoints::register_account_endpoint)
            .service(endpoints::login_account_endpoint)


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


// Account Login
//
// When the user logs in, it will check whether they have the
// same hwid as when they first registered. If not, then it will
// send a message to the discord channel notifying the mods
// that the user has a different hwid.
//
// It will also keep track of how many unique hwids the user
// has used. This way, mods can track whether the user is using
// a hardware id spoofer. (what cheaters use to prevent being banned)
//
//
// To make sure the user doesn't just make a different account each time
// they use the anti-cheat, there will be no signout button, thus the user
// must reinstall the program all over again to signout.
//
//
// When the user first installs the program, it will check if an account is
// already registered with their hwid, if it is, it will automatically sign in
// using the account registered to that hwid.


// Discord Channel Tokens
//
// Have an algorithm for disguising the discord channel ids.
// This is so no can just use any discord channel id and make it
// send messages with the bot.
//
// Instead, an encrypted channel id will be sent to the 
// user when they need to get a vac token. Then have a way
// to decrypt the token and get access to the channel id

