use actix_web::{self, web, App, HttpRequest, HttpServer, Responder};
mod handlers;
mod global;
mod auth;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // Establish a connection to the database
    let db: handlers::Database = handlers::Database::init().await;

    // Establish a connection to http://127.0.0.1:8080/
    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(db.clone()))
            .wrap(actix_cors::Cors::permissive())
            .service(main_endpoint)
            .service(send_message_endpoint)
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}

// The default endpoint
#[actix_web::get("/")]
async fn main_endpoint(_req: HttpRequest) -> impl Responder {
    return "Welcome to the Reborn API!"
}

// The send message to a channel with the reborn
// discord bot endpoint
#[actix_web::post("/send/message/{channel}/")]
async fn send_message_endpoint(req: HttpRequest) -> impl Responder {
    // Get the channel id from the url parameters
    let channel: &str = match req.match_info().get("channel") {
        Some(channel) => channel,
        None => return "{\"error\": \"Invalid channel id.\"}".to_string()
    };

    // Get the provided authorization headers
    let bearer: String = global::get_header(&req, "authorization");
    let access_token: String = global::get_header(&req, "access_token");

    // Verify the provided authorization headers
    if !auth::verify(&bearer, &access_token) {
        return "{\"error\": \"invalid request\"}".to_string();
    }
    
    // Response String
    return format!("{{\"success\": \"message sent to {}\"}}", channel);
}

// Account Body Struct for the account endpoints
// and the account database functions
#[derive(serde::Deserialize)]
pub struct AccountBody {
    pub name: String,
    pub email: String,
    pub password: String,
    identifier: String
}

// The register account endpoint is used to register
// an user and add them to the sqlite database. A registration
// is required for the user to be able to provide a bearer token
// which will be used for verification.
#[actix_web::post("/register")]
async fn register_account(
    req: HttpRequest, 
    db: web::Data<handlers::Database>,
    body: web::Json<AccountBody>
) -> impl Responder {
    // Get the provided authorization headers
    let bearer: String = global::get_header(&req, "authorization");
    let access_token: String = global::get_header(&req, "access_token");

    // Verify the provided authorization headers
    if !auth::verify(&bearer, &access_token) {
        return "{\"error\": \"invalid request\"}".to_string();
    }

    // Register the account to the database
    if !register_account_to_database(db, body).await {
        return "{\"error\": \"failed to register account\"}".to_string();
    }
    return format!("{{\"success\": \"successfully registered account\"}}");
}

// The register_account_to_database() function is used to
// register an user to the sqlite database.
async fn register_account_to_database(
    db: web::Data<handlers::Database>,
    body: web::Json<AccountBody>
) -> bool {
    // Get the current date
    let date: u128 = global::get_time().as_nanos();

    // Generate a new user id
    let user_id: String = global::generate_new_id(&format!("{}:{}:{}", body.email, body.password, date));

    // Insert the user into the database
    let query = sqlx::query!(
        "INSERT INTO users (name, email, password, user_id, hwid, unique_hwid_count) VALUES (?, ?, ?, ?, ?, ?)",
        body.name, body.email, body.password, user_id, body.identifier, 0
    ).execute(&db.conn).await;

    // Return query result
    return match query {
        Ok(q) => q.rows_affected() > 0,
        Err(_) => false,
    };
}

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

// Instead, an encrypted channel id will be sent to the 
// user when they need to get a vac token. Then have a way
// to decrypt the token and get access to the channel id

