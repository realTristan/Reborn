use actix_web::{Responder, HttpRequest, web};
use crate::lib::{global, auth, database};

// Message Body Struct for the send message endpoint
#[derive(serde::Deserialize)]
pub struct MessageBody {
    pub name: String,
    pub email: String,
    pub identifier: String,
    pub hardware_info: String // Base64 encode a json of the users hardware info (open programs, etc.)
}

// Account Body Struct for the account endpoints
// and the account database functions
#[derive(serde::Deserialize)]
pub struct AccountBody {
    pub name: String,
    pub email: String,
    pub password: String,
    pub identifier: String
}


// The send message to a channel with the reborn
// discord bot endpoint
#[actix_web::post("/message/{channel}/")]
async fn send_message_endpoint(
    req: HttpRequest,
    body: web::Json<MessageBody>
) -> impl Responder {
    
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


// The register account endpoint is used to register
// an user and add them to the sqlite database. A registration
// is required for the user to be able to provide a bearer token
// which will be used for verification.
#[actix_web::put("/register")]
async fn register_account_endpoint(
    req: HttpRequest, 
    db: web::Data<database::Database>,
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
    if !db.register_account_to_database(body).await {
        return "{\"error\": \"failed to register account\"}".to_string();
    }
    return "{\"success\": \"successfully registered account\"}".to_string();
}


// The login account endpoint is used to login an user.
#[actix_web::post("/login")]
async fn login_account_endpoint(
    req: HttpRequest, 
    db: web::Data<database::Database>,
    body: web::Json<AccountBody>
) -> impl Responder {

    // Get the provided authorization headers
    let bearer: String = global::get_header(&req, "authorization");
    let access_token: String = global::get_header(&req, "access_token");

    // Verify the provided authorization headers
    if !auth::verify(&bearer, &access_token) {
        return "{\"error\": \"invalid request\"}".to_string();
    }

    // Return whether the password was valid or not
    if db.valid_login_password(&body.email, &body.password).await {
        return "{\"success\": \"valid credentials\"}".to_string();
    }
    return "{\"error\": \"invalid email or password\"}".to_string();
}

#[actix_web::get("/token/{token}")]
async fn get_token(
    req: HttpRequest, 
    db: web::Data<database::Database>,
    body: web::Json<AccountBody>
) -> impl Responder {

    // Get the provided authorization headers
    let user_id: String = global::get_header(&req, "user_id");
    let access_token: String = global::get_header(&req, "access_token");

    // Verify the provided authorization headers
    if !auth::verify(&user_id, &access_token) {
        return "{\"error\": \"invalid request\"}".to_string();
    }

    // Return the generated token
    return format!("{{\"token\": \"{}\", \"expires_in\": \"{}\"}}", "the generated token", "expiration time");
}

// The create_token endpoint is used to generate
// a new vac token and insert it into the database along
// with it's corresponding channel id and time of its creation.
#[actix_web::put("/token")]
async fn create_token(
    req: HttpRequest, 
    db: web::Data<database::Database>,
    body: web::Json<AccountBody>
) -> impl Responder {

    // Get the provided authorization headers
    let user_id: String = global::get_header(&req, "user_id");
    let access_token: String = global::get_header(&req, "access_token");

    // Verify the provided authorization headers
    if !auth::verify(&user_id, &access_token) {
        return "{\"error\": \"invalid request\"}".to_string();
    }

    // Return the generated token
    return format!("{{\"token\": \"{}\"}}", "the generated token");
}

// The delete_token endpoint is used to delete
// the provided token from the database.
#[actix_web::delete("/token")]
async fn delete_token(
    req: HttpRequest, 
    db: web::Data<database::Database>,
    body: web::Json<AccountBody>
) -> impl Responder {

    // Get the provided authorization headers
    let user_id: String = global::get_header(&req, "user_id");
    let access_token: String = global::get_header(&req, "access_token");

    // Verify the provided authorization headers
    if !auth::verify(&user_id, &access_token) {
        return "{\"error\": \"invalid request\"}".to_string();
    }

    // Return the generated token
    return "{\"success\": \"token deleted\"}".to_string();
}
