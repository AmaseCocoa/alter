use keyring::Entry;
use std::error::Error;

const KEYRING_SERVICE: &str = "alter";

/// Get a credential token from the system keyring
///
/// # Arguments
/// * `profile_id` - The profile UUID
/// * `host` - The Git host (e.g., "github.com", "gitlab.com")
///
/// # Returns
/// The OAuth token stored in the keyring
pub fn get_credential(profile_id: &str, host: &str) -> Result<String, Box<dyn Error>> {
    let username = format!("{}:{}", profile_id, host);
    let entry = Entry::new(KEYRING_SERVICE, &username)?;
    let password = entry.get_password()?;

    // Extract token from "oauth:<token>" format
    if let Some(token) = password.strip_prefix("oauth:") {
        Ok(token.to_string())
    } else {
        Ok(password)
    }
}

/// Store a credential token in the system keyring
///
/// # Arguments
/// * `profile_id` - The profile UUID
/// * `host` - The Git host (e.g., "github.com", "gitlab.com")
/// * `token` - The OAuth token to store
pub fn set_credential(profile_id: &str, host: &str, token: &str) -> Result<(), Box<dyn Error>> {
    let username = format!("{}:{}", profile_id, host);
    let password = format!("oauth:{}", token);
    let entry = Entry::new(KEYRING_SERVICE, &username)?;
    entry.set_password(&password)?;
    Ok(())
}

/// Delete a credential from the system keyring
///
/// # Arguments
/// * `profile_id` - The profile UUID
/// * `host` - The Git host (e.g., "github.com", "gitlab.com")
pub fn delete_credential(profile_id: &str, host: &str) -> Result<(), Box<dyn Error>> {
    let username = format!("{}:{}", profile_id, host);
    let entry = Entry::new(KEYRING_SERVICE, &username)?;
    entry.delete_password()?;
    Ok(())
}
