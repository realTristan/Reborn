use super::global;


// Database Struct for globalizing it's
// connection variable
#[derive(Clone)]
pub struct Database {
    pub conn: sqlx::SqlitePool,
}

pub struct Token {
    pub channel: i64,
    pub created_by: String,
    pub created_at: i64,
    pub expires_in: i64
}

// Database Implemenetation that contains all the
// functions for manipulating the sqlite db data
impl Database {
    // Initialize a new database connection
    pub async fn init() -> Self {
        // Initialize a connection to the sqlite database
        return Self {
            conn: sqlx::sqlite::SqlitePoolOptions::new()
                .connect_with(
                    sqlx::sqlite::SqliteConnectOptions::new()
                        .filename("database.sqlite")
                        .create_if_missing(true),
                )
                .await
                .expect("Couldn't connect to database"),
        };
    }


    // The account_hwid_exists() function is used to check whether
    // the provided hwid already has an account. If it does, then
    // it will return the username of the user with the provided hwid.
    pub async fn account_hwid_exists(&self, hwid: &str) -> Option<String> {
        let query = sqlx::query!(
            "SELECT username FROM users WHERE hwid = ?", hwid
        ).fetch_one(&self.conn).await;

        // Return whether the user already exists
        return match query {
            Ok(q) => Some(q.username),
            Err(_) => None
        };
    }


    // The account_already_exists() function is used to check whether
    // the provided username is already being used for an account. If it
    // is, then the user will not be able to register with the provided username.
    pub async fn account_already_exists(&self, username: &str, hwid: &str) -> bool {
        let query = sqlx::query!(
            "SELECT username, hwid FROM users WHERE username = ? AND hwid = ?", username, hwid
        ).fetch_one(&self.conn).await;

        // Return whether the user already exists
        return match query {
            Ok(_) => true,
            Err(_) => false,
        };
    }

    // The register_user_to_database() function is used to
    // register an user to the sqlite database.
    pub async fn register_user_to_database(&self, username: &str, hwid: &str) -> bool {
        // Insert the user into the database
        let query = sqlx::query!(
            "INSERT INTO users (username, hwid) VALUES (?, ?)",
            username, hwid
        ).execute(&self.conn).await;

        // Return query result
        return match query {
            Ok(_) => true,
            Err(_) => false
        };
    }


    // The get_token() function is used to get the channel id,
    // creator_id, and creation time of a token from the database 
    // using the provided token.
    pub async fn get_token(&self, token: &str) -> Option<Token> {
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
                let token_expires_in: i64 = token_expiration_date - global::get_time().as_millis() as i64;
                
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
        let query = sqlx::query!(
            "SELECT token FROM tokens WHERE token = ?",
            token
        ).fetch_one(&self.conn).await;
        return match query {
            Ok(_) => true,
            Err(_) => false
        }
    }


    // The generate_token() function is used to generate a new token
    // and insert it into the database. If the insertion fails, then the
    // function will return None instead of the newly generated token.
    pub async fn generate_token(&self, channel: i64, user: &str)-> Option<String> {
        // Generate a new token hash
        let mut token: String = global::generate_new_id(
            &format!("{}:{}", user, global::get_time().as_nanos())
        );

        // Check if the token already exists inm the database. If it does, 
        // generate a new one until it doesn't already exist in the database
        while self.token_exists(&token).await {
            token = global::generate_new_id(
                &format!("{}:{}", user, global::get_time().as_nanos())
            );
        }

        // Get the current time as milliseconds
        let time: i64 = global::get_time().as_millis() as i64;

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
        let query = sqlx::query!(
            "DELETE FROM tokens WHERE token = ? AND created_by = ?",
            token, user
        ).execute(&self.conn).await;
        return match query {
            Ok(_) => true,
            Err(_) => false
        };
    }
}