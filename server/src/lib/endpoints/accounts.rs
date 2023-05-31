use actix_web::{ HttpRequest, web, HttpResponse};
use crate::lib::{
    handlers::Database, auth, http
};

// The register account endpoint is used to register
// an user and add them to the sqlite database. A registration
// is required for the user to be able to provide a bearer token
// which will be used for verification.
#[actix_web::put("/account/register")]
async fn register_user_endpoint(
    req: HttpRequest, db: web::Data<Database>, body: web::Bytes
) -> HttpResponse {
    // Get the request body
    let body: serde_json::Value = match http::body(&body) {
        Ok(body) => body,
        Err(_) => return http::response(
            http::Status::BAD_REQUEST,
            serde_json::json!({
                "response": "Invalid request body"
            })
        )
    };
    // Get the identifier from the request body
    let identifier: String = match body.get("identifier") {
        Some(id) =>  id.to_string(),
        None => return http::response(
            http::Status::BAD_REQUEST,
            serde_json::json!({
                "response": "Invalid request body"
            })
        )
    };
    // Get the username from the request body
    let username: String = match body.get("username") {
        Some(u) => u.to_string(),
        None => return http::response(
            http::Status::BAD_REQUEST,
            serde_json::json!({
                "response": "Invalid request body"
            })
        )
    };

    /* Get the provided authorization headers
    // Authorization: sha256("hwid")
    let auth: String = http::header(&req, "authorization");
    let access_token: String = http::header(&req, "access_token");

    // Verify the provided authorization headers
    if !auth::verify(&auth, &access_token) {
        return http::response(
            http::Status::BAD_REQUEST,
            serde_json::json!({
                "response": "Invalid request"
            })
        )
    }*/

    // Check if the account already exists
    if db.account_already_exists(&username, &identifier).await {
        return http::response(
            http::Status::BAD_REQUEST,
            serde_json::json!({
                "response": "Username already exists"
            })
        )
    }

    // Register the account to the database
    return match db.register_user_to_database(&username, &identifier).await {
        true => http::response(
            http::Status::OK,
            serde_json::json!({
                "response": "Registered user"
            })
        ),
        false => http::response(
            http::Status::BAD_REQUEST,
            serde_json::json!({
                "response": "Failed to register user"
            })
        )
    }
}


// The login_user() endpoint is used to check whether
// an account already exists for the provided hwid. 
// If it does, then the user will automatically be logged 
// into that account. This is used to prevent users from
// making mulitple accounts when they don't need to.
#[actix_web::post("/account/login")]
async fn login_user_endpoint(
    req: HttpRequest, db: web::Data<Database>, body: web::Bytes
) -> HttpResponse {

    // Get the request body
    let body: serde_json::Value = match http::body(&body) {
        Ok(body) => body,
        Err(_) => return http::response(
            http::Status::BAD_REQUEST,
            serde_json::json!({
                "response": "Invalid request body"
            })
        )
    };
    // Get the identifier from the request body
    let identifier: String = match body.get("identifier") {
        Some(id) =>  id.to_string(),
        None => return http::response(
            http::Status::BAD_REQUEST,
            serde_json::json!({
                "response": "Invalid request body"
            })
        )
    };
    
    // Get the provided authorization headers
    // Authorization: sha256("hwid")
    let auth: String = http::header(&req, "authorization");
    let access_token: String = http::header(&req, "access_token");

    // Verify the provided authorization headers
    if !auth::verify(&auth, &access_token) {
        return http::response(
            http::Status::BAD_REQUEST,
            serde_json::json!({
                "response": "Invalid request"
            })
        )
    }

    // Check if the account already exists for the provided
    // body.identifier (hwid)
    return match db.account_hwid_exists(&identifier).await {
        Some(username) => http::response(
            http::Status::OK,
            serde_json::json!({
                "response": "User exists",
                "username": username
            })
        ),
        None => http::response(
            http::Status::BAD_REQUEST,
            serde_json::json!({
                "response": "Invalid user"
            })
        )
    };
}
