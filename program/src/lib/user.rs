use super::{global, http};

#[derive(Debug, Clone)]
pub struct User {
    pub name: String,
    bearer: String,
}

impl User {
    pub fn new() -> Self {
        Self {
            name: String::new(),
            bearer: "test".to_string(),
            /*
            match global::get_bearer() {
                Ok(b) => b,
                Err(e) => panic!("Error: {}", e)
            }
            
            */
        }
    }

    // Verify whether the provided username is valid
    pub fn is_valid_name(&self) -> Result<(), String> {
        for c in self.name.chars() {
            if !c.is_alphanumeric() {
                return Err(String::from("Username can only contain alphanumeric characters"))
            }
        }
        if self.name.len() < 3 {
            return Err(String::from("Username must be at least 3 characters long"))
        }
        if self.name.len() > 16 {
            return Err(String::from("Username cannot be longer than 16 characters"))
        }
        Ok(())
    }

    // The login function is used to login to the API
    // using the users hwid.
    pub fn login(&self) -> u8 {
        // Generate a new access token
        let access_token: String = global::generate_access_token(&self.bearer);

        // Build the http request
        let resp = http::CLIENT
            .post("http://localhost:8080/account/login")
            .header("authorization", self.bearer.clone())
            .header("access_token", access_token)
            .header("content-type", "application/json")
            .json(&serde_json::json!({"identifier": self.bearer}))
            .send().expect("failed to send login request");
        
        // Return the page number depending on resp success
        return resp.status().is_success() as u8;
    }

    // Register an account to the database
    pub fn register(&self) -> Result<(), String> {
        // Generate a new access token
        let access_token: String = global::generate_access_token(&self.bearer);

        // Build the http request
        let resp = http::CLIENT
            .put("http://localhost:8080/account/register/")
            .header("authorization", self.bearer.clone())
            .header("access_token", access_token)
            .json(&serde_json::json!({"username": self.name, "identifier": self.bearer}))
            .send().expect("failed to send register request");
        
        
        // If the response was a success, return Ok
        if resp.status().is_success() {
            return Ok(())
        }

        // Else, if the response was an error, return an Err
        return match resp.json::<std::collections::HashMap<String, String>>() {
            Ok(j) => match j.get("response") {
                Some(e) => Err(e.to_string()),
                None => Err(String::from("Failed to parse error response"))
            },
            Err(e) => Err(e.to_string())
        };
    }
}