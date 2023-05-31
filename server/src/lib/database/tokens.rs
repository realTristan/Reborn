use crate::lib::{
    handlers::Database, utils, structs::Token
};

impl Database {
    // The get_token_info() function is used to get the channel id,
    // creator_id, and creation time of a token from the database 
    // using the provided token.
    pub async fn get_token_info(&self, token: &str) -> Option<Token> {
        // Query the database for the token
        let query = sqlx::query!(
            "SELECT channel, created_by, created_at FROM tokens WHERE token = ?",
            token
        ).fetch_one(&self.conn).await;

        // Return the query result
        return match query {
            Ok(query) => {
                // Calculate the expiration date of the token
                let token_expiration_date: i64 = query.created_at + 86400000;
                let token_expires_in: i64 = token_expiration_date - utils::get_time().as_millis() as i64;
                
                // Check if the token has expired
                if token_expires_in <= 0 {
                    // Delete the token from the database
                    let _ = sqlx::query!("DELETE FROM tokens WHERE token = ?", token)
                        .execute(&self.conn)
                        .await;

                    // Return None
                    return None
                }

                // Return the token
                return Some(Token {
                    channel: query.channel,
                    created_by: query.created_by.to_string(),
                    created_at: query.created_at,
                    expires_in: token_expires_in
                })
            },
            Err(_) => None
        };
    }

    // The token_exists() function is used to check whether
    // the provided token exists in the database.
    async fn token_exists(&self, token: &str) -> bool {
        // Query the database for the token
        let query = sqlx::query!(
            "SELECT token FROM tokens WHERE token = ?",
            token
        ).fetch_one(&self.conn).await;

        // Return query result
        return !query.is_err();
    }

    // The generate_token() function is used to generate a new token
    // and insert it into the database. If the insertion fails, then the
    // function will return None instead of the newly generated token.
    pub async fn generate_token(&self, channel: i64, user: &str)-> Option<String> {
        // Generate a new token hash
        let mut token: String = utils::generate_new_id(user);

        // Check if the token already exists inm the database. If it does, 
        // generate a new one until it doesn't already exist in the database
        while self.token_exists(&token).await {
            token = utils::generate_new_id(user);
        }

        // Get the current time as milliseconds
        let time: i64 = utils::get_time().as_millis() as i64;

        // Query the database to insert the token
        let query = sqlx::query!(
            "INSERT INTO tokens (token, channel, created_by, created_at) VALUES (?, ?, ?, ?)",
            token, channel, user, time
        ).execute(&self.conn).await;

        // Return the token if the insertion was a success
        return match query {
            Ok(_) => Some(token),
            Err(_) => None
        };
    }

    
    // The delete_token() function is used to delete the
    // provided token from the database.
    //
    // This function is only called by the delete_token_endpoint()
    pub async fn delete_token(&self, token: &str, user: &str) -> bool {
        // Query the database to delete the token
        let query = sqlx::query!(
            "DELETE FROM tokens WHERE token = ? AND created_by = ?",
            token, user
        ).execute(&self.conn).await;

        // Return the query result
        return !query.is_err();
    }
}