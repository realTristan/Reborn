use crate::lib::{
    endpoints::AccountBody, 
    global
};

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
    // the provided email is already being used for an account. If it
    // is, then the user will not be able to register with the provided email.
    async fn account_already_exists(&self, email: &str) -> bool {
        let query = sqlx::query!(
            "SELECT email FROM users WHERE email = ?", email
        ).fetch_one(&self.conn).await;

        // Return whether the email already exists
        return match query {
            Ok(_) => true,
            Err(_) => false,
        };
    }

    // The register_account_to_database() function is used to
    // register an user to the sqlite database.
    pub async fn register_account_to_database(&self, body: actix_web::web::Json<AccountBody>) -> bool {
        // Check if the account already exists
        if self.account_already_exists(&body.email).await {
            return false;
        }

        // Get the current date
        let date: u128 = global::get_time().as_nanos();
        // Hash the password
        let password: String = sha256::digest(&*body.password);
        // Generate a new user id
        let user_id: String = global::generate_new_id(
            &format!("{}:{}:{}", body.email, body.password, date)
        );

        // Insert the user into the database
        let query = sqlx::query!(
            "INSERT INTO users (name, email, password, user_id, hwid, unique_hwid_count) VALUES (?, ?, ?, ?, ?, ?)",
            body.name, body.email, password, user_id, body.identifier, 0
        ).execute(&self.conn).await;

        // Return query result
        return match query {
            Ok(q) => q.rows_affected() > 0,
            Err(_) => false,
        };
    }


    // The valid_login_password() function is used to check whether the
    // provided password is the same password as the one in the
    // sqlite database for the provided email.
    pub async fn valid_login_password(&self, email: &str, password: &str) -> bool {
        // Hash the provided password
        //
        // The password will also be hashed in the incoming http request.
        //
        let password: String = sha256::digest(password);

        // Get the password for the provided email
        let query = sqlx::query!(
            "SELECT password FROM users WHERE email = ?", email
        ).fetch_one(&self.conn).await;

        // Check the query result. If the query resulted in an error
        // or the password doesn't match the one in the database, then
        // return an error message
        return match query {
            Ok(q) => q.password == password,
            Err(_) => false,
        }
    }

    pub async fn get_channel_id_from_token() {
        // TODO
    }

    pub async fn create_token() {
        // TODO
    }

    pub async fn delete_token() {
        // TODO
    }

}