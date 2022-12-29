use actix_web::{ HttpRequest, web, Responder};
use crate::lib::{
    handlers::Database, global, auth, structs::AccountBody
};

// The register account endpoint is used to register
// an user and add them to the sqlite database. A registration
// is required for the user to be able to provide a bearer token
// which will be used for verification.
#[actix_web::put("/account/register/")]
async fn register_user_endpoint(
    req: HttpRequest, db: web::Data<Database>, body: web::Json<AccountBody>
) -> impl Responder {

    // Get the provided authorization headers
    // Authorization: sha256("hwid")
    let auth: String = global::get_header(&req, "authorization");
    let access_token: String = global::get_header(&req, "access_token");

    // Verify the provided authorization headers
    if !auth::verify(&auth, &access_token) {
        return serde_json::json!({
            "status": "400",
            "response": "Invalid request"
        }).to_string()
    }

    // Check if the account already exists
    if db.account_already_exists(&body.username, &body.identifier).await {
        return serde_json::json!({
            "status": "400",
            "response": "Username already exists"
        }).to_string()
    }

    // Register the account to the database
    if db.register_user_to_database(&body.username, &body.identifier).await {
        return serde_json::json!({
            "status": "200",
            "response": "Successfully registered user"
        }).to_string()
    }
    return serde_json::json!({
        "status": "400",
        "response": "Failed to register user"
    }).to_string()
}



// The login_user() endpoint is used to check whether
// an account already exists for the provided hwid. 
// If it does, then the user will automatically be logged 
// into that account. This is used to prevent users from
// making mulitple accounts when they don't need to.
#[actix_web::post("/account/")]
async fn login_user_endpoint(
    req: HttpRequest, db: web::Data<Database>, body: web::Json<AccountBody>
) -> impl Responder {

    // Get the provided authorization headers
    // Authorization: sha256("hwid")
    let auth: String = global::get_header(&req, "authorization");
    let access_token: String = global::get_header(&req, "access_token");

    // Verify the provided authorization headers
    if !auth::verify(&auth, &access_token) {
        return serde_json::json!({
            "status": "400",
            "response": "Invalid request"
        }).to_string()
    }

    // Check if the account already exists for the provided
    // body.identifier (hwid)
    return match db.account_hwid_exists(&body.identifier).await {
        Some(username) => serde_json::json!({
            "status": "200",
            "response": username
        }).to_string(),
        None => serde_json::json!({
            "status": "400",
            "response": "Invalid user"
        }).to_string()
    };
}

