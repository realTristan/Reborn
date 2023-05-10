pub type Status = actix_web::http::StatusCode;

// The header() function is used to bypass
// any invalid header errors.
pub fn header(req: &actix_web::HttpRequest, key: &str) -> String {
    return match req.headers().get(key) {
        Some(v) => v.to_str().unwrap().to_string(),
        None => return "".to_string(),
    };
}

// The body() function is used to bypass any invalid
// body errors. This function is used to get the request
// body then return it as an accessible serde_json::Value.
pub fn body(body: &actix_web::web::Bytes) -> Result<serde_json::Value, ()> {
    return match serde_json::from_slice(&body.to_vec()) {
        Ok(v) => Ok(v),
        Err(_) => Err(()),
    };
}

// The response() function is used to quickly
// and cleanly return a response with the
// provided json data.
pub fn response(status: Status, json: serde_json::Value) -> actix_web::HttpResponse {
    return actix_web::HttpResponse::Ok()
        .status(status)
        .content_type("application/json")
        .json(json);
}