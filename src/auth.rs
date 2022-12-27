// Library Usages
use std::collections::HashMap;
use std::sync::Mutex;

use super::global;

// The SUPER_SECRET_CODE is what's used to prevent
// users trying to abuse the api from being able
// to generate their own auth tokens
static SUPER_SECRET_CODE: &str = "super_secret_code";

// The TOKEN_STORAGE is used to store previously used
// tokens so that abusers can't access the api using
// a previous token.
lazy_static::lazy_static! {
    static ref TOKEN_STORAGE: Mutex<HashMap<String, Vec<String>>> = {
        Mutex::new(HashMap::new())
    };
}

// The verify() function is used to check whether the
// provided auth token is valid. It does this by
// checking whether the token has been created within
// the past 8 seconds. If so, return true, else, return false.
pub fn verify(bearer: &str, access_token: &str) -> bool {
    // Lock the TOKEN STORAGE so we can access it's data
    let _token_storage = TOKEN_STORAGE.lock();
    // If an error has occurred, return false
    if _token_storage.is_err() {
        return false;
    }

    // Unwrap the token storage data
    let mut token_storage = _token_storage.unwrap();
    // Get the system time since epoch. This value
    // is used to check how long ago the auth token was
    // generated. Doing this prevents users from consecutively
    // using a single auth token if trying to abuse the api
    let time: u64 = global::get_time();
    // Convert the token storage into a mutable variable.
    // This is required so that we can append the access_token
    // to the users token storage, or so that we can clear
    // the token storage if full.
    let mut_storage: Option<&mut Vec<String>> = token_storage.get_mut(bearer);

    // If the user doesn't already exist within the
    // token storage return true
    if mut_storage.is_none() {
        // Insert the user into the token storage
        // along with the current time and auth token
        token_storage.insert(
            bearer.to_string(),
            [time.to_string(), access_token.to_string()].to_vec(),
        );
        // Return true as the token did not
        // previously exist in the token storage
        return true;
    }
    // Unwrap the mutable token storage
    let mut_storage: &mut Vec<String> = mut_storage.unwrap();

    // Execute the storage handler
    // If the function returns false, then the provided
    // auth token has already been used within the past 8 seconds.
    if !storage_handler(mut_storage, access_token, &time) {
        return false;
    };

    // Check whether the auth token was generated
    // within the past 8 seconds
    for i in 0..8 {
        let gen: String = format!("{}:{}:{}", bearer, time - i, SUPER_SECRET_CODE);
        // If the provided auth token is equal to the
        // generated auth token, return true
        if access_token == sha256::digest(gen) {
            // Append the access token to the TOKEN_STORAGE
            mut_storage.push(access_token.to_string());
            return true;
        }
    }
    return false;
}

// The storage_handler() function is used to check whether
// the provided auth token has already been used
// within the past 8 seconds. This is function is
// necessary to prevent abusers from using the same
// token more than once.
fn storage_handler(mut_storage: &mut Vec<String>, access_token: &str, time: &u64) -> bool {
    // Get the last storage wipe time
    let last_wipe_time: u64 = mut_storage[0].parse().unwrap();
    // If the last wipe happened over 8 seconds ago,
    // wipe the users token storage to prevent an
    // overflow. If the user has too many tokens and
    // the cache isn't eventually cleared.. you already
    // know what'll happen lmao.
    if time > &(last_wipe_time + 8) || mut_storage.len() > 10 {
        // Clear the users token storage and set
        // the first value of the array to the
        // current time as a string
        mut_storage.clear();
        mut_storage[0] = time.to_string();

        // Return true to ignore using the below
        // return satatement
        return true;
    }
    // After the users current token storage has or hasn't been
    // cleared, check whether the access_token is already existant
    // in the token storage. If it is, return false, thus the
    // user is using an unauthorized token. Else, return true.
    return !mut_storage.contains(&access_token.to_string());
}
