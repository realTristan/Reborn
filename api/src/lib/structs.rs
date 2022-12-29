// Token Struct for the token database functions
pub struct Token {
    pub channel: i64,
    pub created_by: String,
    pub created_at: i64,
    pub expires_in: i64
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

// Message Body Struct for the send message endpoint
#[derive(serde::Deserialize)]
pub struct MessageBody {
    pub username: String,
    pub identifier: String,         // sha256 hwid
    pub embed: String,              // The embed to send in the discord channel
    pub image: String,              // Base64 encode a png of the users screen
    pub hardware_info: String       // Base64 encode a json of the users hardware info (open programs, etc.)
}