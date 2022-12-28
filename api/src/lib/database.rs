
// Database Struct for globalizing it's
// connection variable
#[derive(Clone)]
pub struct Database {
    pub conn: sqlx::SqlitePool,
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

    // The account_already_exists() function is used to check whether
    // the provided username is already being used for an account. If it
    // is, then the user will not be able to register with the provided username.
    async fn account_already_exists(&self, username: &str, hwid: &str) -> bool {
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
    pub async fn register_user_to_database(&self, username: &str, hwid: &str) -> String {
        // Check if the account already exists
        if self.account_already_exists(username, hwid).await {
            return "username already exists".to_string();
        }

        // Insert the user into the database
        let query = sqlx::query!(
            "INSERT INTO users (username, hwid) VALUES (?, ?)",
            username, hwid
        ).execute(&self.conn).await;

        // Return query result
        return match query {
            Ok(_) => "successfully registered user".to_string(),
            Err(_) => "failed to register user".to_string()
        };
    }

    pub async fn get_channel_id_from_token() {
        // TODO
    }

    pub async fn user_with_hwid_already_exists() {
        // TODO
    }

    pub async fn get_token() {
        // TODO
        // get the token creator, expiration date, channel id, etc.
    }

    pub async fn create_token() {
        // TODO
        // generate a new token hash
        // insert the token into the database
    }

    pub async fn delete_token() {
        // TODO
        // delete the token from the database
    }

}