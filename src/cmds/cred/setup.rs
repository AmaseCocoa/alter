use std::io::{self, Write};

use crate::config;
use crate::keyring;
use crate::oauth::{pkce::PKCESecret, session::OAuth2Session};

pub fn setup_credentials(profile: String, host: Option<String>) {
    let profile_info = match config::get_profile_from_slug(profile.clone()) {
        Ok(p) => p,
        Err(e) => {
            eprintln!("Failed to load profile '{}': {}", profile, e);
            return;
        }
    };

    let target_host = if let Some(h) = host {
        h
    } else {
        print!("Enter Git host (e.g., github.com): ");
        io::stdout().flush().ok();
        let mut input = String::new();
        match io::stdin().read_line(&mut input) {
            Ok(_) => input.trim().to_string(),
            Err(e) => {
                eprintln!("Failed to read host input: {}", e);
                return;
            }
        }
    };

    if target_host.is_empty() {
        eprintln!("Host cannot be empty");
        return;
    }

    let oauth_provider = match config::get_provider_for_host(&target_host) {
        Ok(Some(p)) => p,
        Ok(None) => {
            eprintln!("No OAuth provider available for host: {}", target_host);
            eprintln!("Please configure a provider in ~/.alter/config.toml");
            return;
        }
        Err(e) => {
            eprintln!("Failed to load provider configuration: {}", e);
            return;
        }
    };

    println!("Starting OAuth2 authentication for {}...", target_host);

    let (auth_endpoint, token_endpoint) = config::get_auth_token_endpoints(&oauth_provider);
    let scopes = config::get_scopes(&oauth_provider);
    let client_secret = oauth_provider.client_secret.clone().unwrap_or_default();

    let mut session = OAuth2Session::with_endpoints(
        oauth_provider.client_id.clone(),
        client_secret,
        format!("https://{}", oauth_provider.host),
        if auth_endpoint.is_empty() {
            None
        } else {
            Some(auth_endpoint)
        },
        if token_endpoint.is_empty() {
            None
        } else {
            Some(token_endpoint)
        },
    );

    let server = session.create_server();
    let pkce = PKCESecret::new();

    println!("Opening browser for authentication...");
    println!(
        "If browser doesn't open, visit: {}",
        session.auth_url(&server, &pkce, &scopes)
    );

    let _ = open::that(session.auth_url(&server, &pkce, &scopes));

    if let Some(code) = session.wait_code() {
        match session.get_token(code, &server, &pkce) {
            Ok(token_response) => {
                let profile_id = profile_info.id.to_string();
                match keyring::set_credential(
                    &profile_id,
                    &target_host,
                    &token_response.access_token,
                ) {
                    Ok(_) => {
                        println!("✓ Token stored in system keyring");
                        eprintln!(
                            "  Debug: service=alter, username={}:{}",
                            profile_id, target_host
                        );

                        match keyring::get_credential(&profile_id, &target_host) {
                            Ok(_) => {
                                eprintln!(
                                    "  Debug: Verification successful - token is retrievable"
                                );
                            }
                            Err(e) => {
                                eprintln!("  Warning: Token storage verification failed: {}", e);
                            }
                        }
                    }
                    Err(e) => {
                        eprintln!("Failed to store token in keyring: {}", e);
                        eprintln!(
                            "  Trying to store: service=alter, username={}:{}",
                            profile_id, target_host
                        );
                        return;
                    }
                }

                match config::add_host_to_credentials(profile, target_host.clone()) {
                    Ok(_) => {
                        println!("✓ Profile updated with host: {}", target_host);
                    }
                    Err(e) => {
                        eprintln!("Failed to update profile: {}", e);
                        return;
                    }
                }

                println!("✓ OAuth2 setup complete for {}", target_host);
            }
            Err(e) => {
                eprintln!("Failed to obtain token: {}", e);
            }
        }
    } else {
        eprintln!("Authentication failed or cancelled");
    }
}
