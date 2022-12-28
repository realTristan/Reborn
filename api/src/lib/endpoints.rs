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
    pub identifier: String
}

// The request_guard() function is used to check whether
// the request is from a valid source. This is done by grabbing
// the authorization and the access_token headers then using the
// auth::verify() function to check whether the provided tokens
// are valid.
//
//
// The "authorization" header is a sha256("hwid") for the following functions:
//      send_discord_message_endpoint()
//      register_account_endpoint()
//      login_account_endpoint()
// 
//
// The "authorization" header is a sha256("user_id") for the following functions:
//      get_token_endpoint()
//      create_token_endpoint()
//      delete_token_endpoint()
//
//
// The "access_token" header is a sha256("{authorization_header}:{time_in_seconds}:{SUPER_SECRET_CODE}")
//
// The request_guard() function returns a bool for whether the request is valid.
fn request_guard(req: &HttpRequest) -> bool {
    // Get the provided authorization headers
    let auth: String = global::get_header(req, "authorization");
    let access_token: String = global::get_header(req, "access_token");

    // Verify the provided authorization headers
    return auth::verify(&auth, &access_token)
}



// The send message to a channel with the reborn
// discord bot endpoint
#[actix_web::post("/message/{channel}/")]
async fn send_discord_message_endpoint(
    req: HttpRequest,
    body: web::Json<MessageBody>
) -> impl Responder {
    // Check if the reuqest is from a valid source
    if !request_guard(&req) {
        return "{\"error\": \"invalid request\"}".to_string();
    }
    
    // Get the channel id from the url parameters
    let channel: &str = match req.match_info().get("channel") {
        Some(c) => c,
        None => return "{\"error\": \"invalid channel id\"}".to_string()
    };
    
    // Response String
    return format!("{{\"success\": \"message sent to {}\"}}", channel);
}



// The register account endpoint is used to register
// an user and add them to the sqlite database. A registration
// is required for the user to be able to provide a bearer token
// which will be used for verification.
#[actix_web::put("/account/register")]
async fn register_account_endpoint(
    req: HttpRequest, 
    db: web::Data<database::Database>,
    body: web::Json<AccountBody>
) -> impl Responder {
    // Check if the reuqest is from a valid source
    if !request_guard(&req) {
        return "{\"error\": \"invalid request\"}".to_string();
    }

    // Register the account to the database
    if !db.register_account_to_database(body).await {
        return "{\"error\": \"failed to register account\"}".to_string();
    }
    return "{\"success\": \"successfully registered account\"}".to_string();
}



// The login_account() endpoint is used to check whether
// an account already exists for the provided hwid. 
// If it does, then the user will automatically be logged 
// into that account. This is used to prevent users from
// making mulitple accounts when they don't need to.
#[actix_web::post("/account/")]
async fn login_account_endpoint(
    req: HttpRequest, 
    db: web::Data<database::Database>,
    body: web::Json<AccountBody>
) -> impl Responder {
    // Check if the reuqest is from a valid source
    if !request_guard(&req) {
        return "{\"error\": \"invalid request\"}".to_string();
    }

    // Check if the account already exists for the provided
    // body.identifier (hwid)
    let account_exists = false;
    if account_exists {
        return "{\"error\": \"an account with the provided hwid already exists\"}".to_string();
    }
    return format!("{{\"user\": \"{}\"}}", "the name of the user with the hwid");
}



// The get_token() endpoint is used to get infrotmation about
// the provided token. This includes the token channel_id, the
// token creation_time and expiration_date, etc.
#[actix_web::get("/token/{token}")]
async fn get_token_endpoint(req: HttpRequest, db: web::Data<database::Database>) -> impl Responder {
    // Check if the reuqest is from a valid source
    if !request_guard(&req) {
        return "{\"error\": \"invalid request\"}".to_string();
    }

    // Get the channel id from the url parameters
    let token: &str = match req.match_info().get("channel") {
        Some(t) => t,
        None => return "{\"error\": \"invalid token\"}".to_string()
    };

    // Return the generated token
    return format!("{{\"token\": \"{}\", \"expires_in\": \"{}\"}}", "the generated token", "expiration time");
}



// The create_token endpoint is used to generate
// a new vac token and insert it into the database along
// with it's corresponding channel id and time of its creation.
#[actix_web::put("/token")]
async fn create_token_endpoint(
    req: HttpRequest, 
    db: web::Data<database::Database>,
    body: web::Json<AccountBody>
) -> impl Responder {
    // Check if the reuqest is from a valid source
    if !request_guard(&req) {
        return "{\"error\": \"invalid request\"}".to_string();
    }

    // Return the generated token
    return format!("{{\"token\": \"{}\"}}", "the generated token");
}



// The delete_token endpoint is used to delete
// the provided token from the database.
#[actix_web::delete("/token")]
async fn delete_token_endpoint(
    req: HttpRequest, 
    db: web::Data<database::Database>,
    body: web::Json<AccountBody>
) -> impl Responder {
    // Check if the reuqest is from a valid source
    if !request_guard(&req) {
        return "{\"error\": \"invalid request\"}".to_string();
    }

    // Return the generated token
    return "{\"success\": \"token deleted\"}".to_string();
}
