use std::collections::HashMap;
use std::io::{self, BufRead};

use crate::config;
use crate::git;
use crate::keyring;
use crate::oauth::{pkce::PKCESecret, session::OAuth2Session};

fn read_credential_input() -> Result<HashMap<String, String>, Box<dyn std::error::Error>> {
    let stdin = io::stdin();
    let mut input = HashMap::new();

    for line in stdin.lock().lines() {
        let line = line?;
        if line.is_empty() {
            break;
        }

        if let Some((key, value)) = line.split_once('=') {
            input.insert(key.to_string(), value.to_string());
        }
    }

    Ok(input)
}

fn write_credential_output(output: &HashMap<String, String>) {
    for (key, value) in output {
        println!("{}={}", key, value);
    }
}

fn get_current_profile_id() -> Result<Option<String>, Box<dyn std::error::Error>> {
    // Prefer repository-local config if available and contains a non-empty namespace.
    if let Ok(local_cfg) = git::GitConfig::load(true) {
        if let Ok(section) = local_cfg.file.section("credential", None) {
            if let Some(namespace) = section.value("namespace") {
                let id_str = namespace.to_string();
                if !id_str.is_empty() {
                    return Ok(Some(id_str));
                }
            }
        }
    }

    // Fallback to the global/user gitconfig if local did not yield a namespace.
    if let Ok(global_cfg) = git::GitConfig::load(false) {
        if let Ok(section) = global_cfg.file.section("credential", None) {
            if let Some(namespace) = section.value("namespace") {
                let id_str = namespace.to_string();
                if !id_str.is_empty() {
                    return Ok(Some(id_str));
                }
            }
        }
    }

    Ok(None)
}

fn setup_credential_via_oauth(
    profile_id: &str,
    host: &str,
) -> Result<String, Box<dyn std::error::Error>> {
    // Get OAuth provider for this host from config
    let oauth_provider = match config::get_provider_for_host(host) {
        Ok(Some(p)) => p,
        Ok(None) => {
            return Err(format!("No OAuth provider configured for host: {}", host).into());
        }
        Err(e) => {
            return Err(format!("Failed to load provider configuration: {}", e).into());
        }
    };

    let client_secret = oauth_provider.client_secret.clone().unwrap_or_default();
    let scopes = config::get_scopes(&oauth_provider);

    eprintln!(
        "alter cred helper: Opening browser for {} authentication...",
        host
    );

    let mut session = if let (Some(auth_ep), Some(token_ep)) = (
        &oauth_provider.auth_endpoint,
        &oauth_provider.token_endpoint,
    ) {
        // Custom OAuth endpoints
        OAuth2Session::with_endpoints(
            oauth_provider.client_id.clone(),
            client_secret,
            format!("https://{}", host),
            Some(auth_ep.clone()),
            Some(token_ep.clone()),
        )
    } else {
        // Default GitHub-style endpoints
        OAuth2Session::new(
            oauth_provider.client_id.clone(),
            client_secret,
            format!("https://{}", host),
        )
    };

    let server = session.create_server();
    let pkce = PKCESecret::new();
    let auth_url = session.auth_url(&server, &pkce, &scopes);
    eprintln!("  URL: {}", auth_url);

    // Try to open browser
    let _ = open::that(&auth_url);

    // Wait for OAuth2 callback
    eprintln!("  Waiting for authentication...");
    if let Some(code) = session.wait_code() {
        let token_response = session.get_token(code, &server, &pkce)?;

        // Store in keyring
        keyring::set_credential(profile_id, host, &token_response.access_token)?;
        eprintln!("  ✓ Credential stored");

        Ok(token_response.access_token)
    } else {
        Err("Authentication cancelled or timeout".into())
    }
}

pub fn helper_get() {
    // Migrate old profiles if needed
    if let Err(e) = config::migrate_old_profiles() {
        eprintln!("Warning: Failed to migrate old profiles: {}", e);
    }

    // Read input from stdin (protocol, host, etc.)
    let input = match read_credential_input() {
        Ok(i) => i,
        Err(e) => {
            eprintln!("alter cred helper: Failed to read credential input: {}", e);
            return;
        }
    };

    let host = match input.get("host") {
        Some(h) => h.clone(),
        None => {
            eprintln!("alter cred helper: No host specified in credential request");
            return;
        }
    };

    let protocol = input.get("protocol").map(|p| p.as_str()).unwrap_or("https");

    // Get current profile ID from git config
    let profile_id = match get_current_profile_id() {
        Ok(Some(id)) => id,
        Ok(None) => {
            eprintln!("alter cred helper: No active profile found");
            eprintln!("  Run: alter use <profile>");
            return;
        }
        Err(e) => {
            eprintln!("alter cred helper: Failed to get current profile: {}", e);
            return;
        }
    };

    // Try to get token from keyring
    let token = match keyring::get_credential(&profile_id, &host) {
        Ok(t) => t,
        Err(_e) => {
            // Token not found, try OAuth2 setup
            eprintln!("alter cred helper: No credential found for {}", host);
            match setup_credential_via_oauth(&profile_id, &host) {
                Ok(t) => {
                    // Also update profile metadata if possible
                    if let Ok(profiles) = config::list_profiles() {
                        if let Some(profile) = profiles
                            .into_iter()
                            .find(|p| p.id.to_string() == profile_id)
                        {
                            let _ = config::add_host_to_credentials(profile.slug, host.clone());
                        }
                    }
                    t
                }
                Err(e) => {
                    eprintln!("alter cred helper: Failed to setup credential: {}", e);
                    eprintln!(
                        "  Alternatively, run: alter cred setup <profile> --host {}",
                        host
                    );
                    return;
                }
            }
        }
    };

    // Output credential in Git helper protocol format
    let mut output = HashMap::new();
    output.insert("protocol".to_string(), protocol.to_string());
    output.insert("host".to_string(), host);
    output.insert("username".to_string(), "oauth".to_string());
    output.insert("password".to_string(), token);

    write_credential_output(&output);
}

pub fn helper_store() {
    // Store credential - handled by 'alter cred setup', so this is a no-op
}

pub fn helper_erase() {
    // Erase credential - handled by 'alter cred revoke', so this is a no-op
}
