use actix_web::{ HttpRequest, web, Responder};
use crate::lib::{
    handlers::Database, global, auth
};

// The get_token() endpoint is used to get infrotmation about
// the provided token. This includes the token channel_id, the
// token creation_time and expiration_date, etc.
#[actix_web::get("/token/{token}")]
async fn get_token_endpoint(
    req: HttpRequest, db: web::Data<Database>
) -> impl Responder {

    // Get the provided authorization headers
    // Authorization: sha256("user_id")
    let auth: String = global::get_header(&req, "authorization");
    let access_token: String = global::get_header(&req, "access_token");

    // Verify the provided authorization headers
    if !auth::verify(&auth, &access_token) {
        return serde_json::json!({
            "status": 400,
            "response": "Invalid request"
        }).to_string()
    }

    // Get the token from the url parameters
    let token: &str = match req.match_info().get("token") {
        Some(t) => t,
        None => return serde_json::json!({
            "status": 400,
            "response": "Invalid token"
        }).to_string()
    };

    // Get the token information from the database
    return match db.get_token(token).await {
        Some(data) => serde_json::json!({
            "status": 200,
            "token": token,
            "channel": data.channel,
            "created_by": data.created_by,
            "created_at": data.created_at,
            "expires_in": data.expires_in.to_string()
        }).to_string(),
        None => serde_json::json!({
            "status": 400,
            "response": "Token invalid or expired"
        }).to_string()
    };
}


// The create_token endpoint is used to generate
// a new vac token and insert it into the database along
// with it's corresponding channel id and time of its creation.
#[actix_web::put("/token")]
async fn generate_token_endpoint(
    req: HttpRequest, db: web::Data<Database>, body: web::Bytes
) -> impl Responder {

    // Get the request body
    let body: serde_json::Value = match global::get_body(&body) {
        Ok(body) => body,
        Err(_) => return serde_json::json!({
            "status": 400,
            "response": "Invalid request body"
        }).to_string()
    };

    // Get the provided authorization headers
    // Authorization: sha256("user_id")
    let auth: String = global::get_header(&req, "authorization");
    let access_token: String = global::get_header(&req, "access_token");

    // Verify the provided authorization headers
    if !auth::verify(&auth, &access_token) {
        return serde_json::json!({
            "status": 400,
            "response": "Invalid request"
        }).to_string()
    }
    // Get the channel from the request body
    let channel: i64 = match body.get("channel") {
        Some(chan) =>  match chan.as_i64() {
            Some(chan) => chan,
            None => return serde_json::json!({
                "status": 400,
                "response": "Invalid channel"
            }).to_string()
        }
        None => return serde_json::json!({
            "status": 400,
            "response": "Invalid channel"
        }).to_string()
    };

    // Generate a new token
    return match db.generate_token(channel, &auth).await {
        Some(t) => serde_json::json!({
            "status": 400,
            "token": t
        }).to_string(),
        None => return serde_json::json!({
            "status": 400,
            "response": "Failed to generate token"
        }).to_string()
    };
}


// The delete_token endpoint is used to delete
// the provided token from the database.
#[actix_web::delete("/token/{token}")]
async fn delete_token_endpoint(req: HttpRequest, db: web::Data<Database>) -> impl Responder 
{
    // Get the provided authorization headers
    // Authorization: sha256("user_id")
    let auth: String = global::get_header(&req, "authorization");
    let access_token: String = global::get_header(&req, "access_token");

    // Verify the provided authorization headers
    if !auth::verify(&auth, &access_token) {
        return serde_json::json!({
            "status": 400,
            "response": "Invalid request"
        }).to_string()
    }

    // Get the token from the url parameters
    let token: &str = match req.match_info().get("token") {
        Some(t) => t,
        None => return serde_json::json!({
            "status": 400,
            "response": "Invalid token"
        }).to_string()
    };

    // Delete the token from the database
    return match db.delete_token(token, &auth).await {
        true => serde_json::json!({
            "status": 200,
            "response": "Token deleted"
        }).to_string(),
        false => serde_json::json!({
            "status": 400,
            "response": "Failed to delete token"
        }).to_string()
    }
}
