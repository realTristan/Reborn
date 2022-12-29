use actix_web::{Responder, HttpRequest, web};
use crate::lib::{global, auth, database};

// Message Body Struct for the send message endpoint
#[derive(serde::Deserialize)]
pub struct MessageBody {
    pub username: String,
    pub identifier: String,     // sha256 hwid
    pub hardware_info: String   // Base64 encode a json of the users hardware info (open programs, etc.)
}

// Account Body Struct for the account endpoints
// and the account database functions
#[derive(serde::Deserialize)]
pub struct AccountBody {
    pub username: String,
    pub identifier: String
}

// Token Body Struct for the token endpoints
#[derive(serde::Deserialize)]
pub struct TokenBody {
    pub token: String,
    pub channel: i64
}


//
// The "authorization" header is a sha256("hwid") for the following functions:
//      send_discord_message_endpoint()
//      register_user_endpoint()
//      login_user_endpoint()
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


// The send message to a channel with the reborn
// discord bot endpoint. This is different from the
// original Versa Anti Cheat which used a discord webhook.
// Discord webhooks are not secure and can be easily tampered with
// thus having our own private api and using a discord bot is
// much more secure.
#[actix_web::post("/message/{channel}/")]
async fn send_discord_message_endpoint(
    req: HttpRequest, 
    body: web::Json<MessageBody>
) -> impl Responder 
{

    // Get the provided authorization headers
    // Authorization: sha256("hwid")
    let auth: String = global::get_header(&req, "authorization");
    let access_token: String = global::get_header(&req, "access_token");

    // Verify the provided authorization headers
    if !auth::verify(&auth, &access_token) {
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
#[actix_web::put("/account/register/")]
async fn register_user_endpoint(
    req: HttpRequest, 
    db: web::Data<database::Database>,
    body: web::Json<AccountBody>
) -> impl Responder 
{
    // Get the provided authorization headers
    // Authorization: sha256("hwid")
    let auth: String = global::get_header(&req, "authorization");
    let access_token: String = global::get_header(&req, "access_token");

    // Verify the provided authorization headers
    if !auth::verify(&auth, &access_token) {
        return "{\"error\": \"invalid request\"}".to_string();
    }

    // Check if the account already exists
    if db.account_already_exists(&body.username, &body.identifier).await {
        return "{\"error\": \"username already exists\"}".to_string();
    }

    // Register the account to the database
    if db.register_user_to_database(&body.username, &body.identifier).await {
        return "{\"success\": \"account registered\"}".to_string();
    }
    return "{\"error\": \"failed to register account\"}".to_string();
}



// The login_user() endpoint is used to check whether
// an account already exists for the provided hwid. 
// If it does, then the user will automatically be logged 
// into that account. This is used to prevent users from
// making mulitple accounts when they don't need to.
#[actix_web::post("/account/")]
async fn login_user_endpoint(
    req: HttpRequest, 
    db: web::Data<database::Database>,
    body: web::Json<AccountBody>
) -> impl Responder 
{
    // Get the provided authorization headers
    // Authorization: sha256("hwid")
    let auth: String = global::get_header(&req, "authorization");
    let access_token: String = global::get_header(&req, "access_token");

    // Verify the provided authorization headers
    if !auth::verify(&auth, &access_token) {
        return "{\"error\": \"invalid request\"}".to_string();
    }

    // Check if the account already exists for the provided
    // body.identifier (hwid)
    return match db.account_hwid_exists(&body.identifier).await {
        Some(username) => format!("{{\"username\": \"{}\"}}", username),
        None => "{\"error\": \"user does not exist\"}".to_string()
    };
}



// The get_token() endpoint is used to get infrotmation about
// the provided token. This includes the token channel_id, the
// token creation_time and expiration_date, etc.
#[actix_web::get("/token/{token}/")]
async fn get_token_endpoint(
    req: HttpRequest, 
    db: web::Data<database::Database>
) -> impl Responder 
{
    // Get the provided authorization headers
    // Authorization: sha256("user_id")
    let auth: String = global::get_header(&req, "authorization");
    let access_token: String = global::get_header(&req, "access_token");

    // Verify the provided authorization headers
    if !auth::verify(&auth, &access_token) {
        return "{\"error\": \"invalid request\"}".to_string();
    }

    // Get the channel id from the url parameters
    let token: &str = match req.match_info().get("token") {
        Some(t) => t,
        None => return "{\"error\": \"invalid token\"}".to_string()
    };

    // Get the token information from the database
    let data = match db.get_token(token).await {
        Some(t) => t,
        None => return "{\"error\": \"token is invalid or has expired\"}".to_string()
    };

    // Return the generated token
    return format!(
        "{{\"token\": \"{}\", \"channel\": \"{}\", \"created_by\": \"{}\", \"created_at\": \"{}\", \"expires_in\": \"{}\"}}",
        token, data.channel, data.created_at, data.created_by, data.expires_in.to_string()
    );
}



// The create_token endpoint is used to generate
// a new vac token and insert it into the database along
// with it's corresponding channel id and time of its creation.
#[actix_web::put("/token/")]
async fn generate_token_endpoint(
    req: HttpRequest, 
    db: web::Data<database::Database>,
    body: web::Json<TokenBody>
) -> impl Responder 
{
    // Get the provided authorization headers
    // Authorization: sha256("user_id")
    let auth: String = global::get_header(&req, "authorization");
    let access_token: String = global::get_header(&req, "access_token");

    // Verify the provided authorization headers
    if !auth::verify(&auth, &access_token) {
        return "{\"error\": \"invalid request\"}".to_string();
    }

    // Generate a new token
    let token = match db.generate_token(body.channel, &auth).await {
        Some(t) => t,
        None => return "{\"error\": \"failed to generate token\"}".to_string()
    };

    // Return the success json
    return format!("{{\"token\": \"{}\"}}", token);
}



// The delete_token endpoint is used to delete
// the provided token from the database.
#[actix_web::delete("/token/")]
async fn delete_token_endpoint(
    req: HttpRequest, 
    db: web::Data<database::Database>,
    body: web::Json<TokenBody>
) -> impl Responder 
{
    // Get the provided authorization headers
    // Authorization: sha256("user_id")
    let auth: String = global::get_header(&req, "authorization");
    let access_token: String = global::get_header(&req, "access_token");

    // Verify the provided authorization headers
    if !auth::verify(&auth, &access_token) {
        return "{\"error\": \"invalid request\"}".to_string();
    }

    // Delete the token from the database
    if db.delete_token(&body.token, &auth).await {
        return "{\"success\": \"token deleted\"}".to_string();
    }
    // Else, if the token could not be deleted, return the error json
    return "{\"error\": \"failed to delete token\"}".to_string();
}