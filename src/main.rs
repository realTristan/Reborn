use actix_web::{self, App, HttpRequest, HttpServer, Responder};
mod global;
mod auth;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // Establish a connection to http://127.0.0.1:8080/
    HttpServer::new(move || {
        App::new()
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

// The register account endpoint is used to register
// an user and add them to the sqlite database. A registration
// is required for the user to be able to provide a bearer token
// which will be used for verification.
#[actix_web::post("/register")]
async fn register_account(req: HttpRequest) -> impl Responder {
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