use crate::lib::handlers::Database;

// Database Implemenetation that contains all the
// functions for manipulating the sqlite db data
impl Database {

    // The account_hwid_exists() function is used to check whether
    // the provided hwid already has an account. If it does, then
    // it will return the username of the user with the provided hwid.
    pub async fn account_hwid_exists(&self, hwid: &str) -> Option<String> 
    {
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
    pub async fn account_already_exists(&self, username: &str, hwid: &str) -> bool 
    {
        // Query the database for the user
        let query = sqlx::query!(
            "SELECT username, hwid FROM users WHERE username = ? AND hwid = ?", username, hwid
        ).fetch_one(&self.conn).await;

        // Return whether the user already exists
        return !query.is_err();
    }

    // The register_user_to_database() function is used to
    // register an user to the sqlite database.
    pub async fn register_user_to_database(&self, username: &str, hwid: &str) -> bool
    {
        // Insert the user into the database
        let query = sqlx::query!(
            "INSERT INTO users (username, hwid) VALUES (?, ?)",
            username, hwid
        ).execute(&self.conn).await;

        println!("{:?}", query.err());

        // Return query result
        return false;
    }
}
