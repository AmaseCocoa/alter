mod config;

use dialoguer::{Select, theme::ColorfulTheme};
use std::process::Command;

fn main() -> std::io::Result<()> {
    let mut cfg = match config::load_config() {
        Ok(c) => c,
        Err(e) => {
            panic!("Failed to load config!\n{}", e.to_string());
        }
    };

    let selections: Vec<String> = cfg
        .accounts
        .iter()
        .map(|account| match account {
            config::Account::AccountDetails {
                id,
                host,
                email,
                username,
                ..
            } => {
                if cfg.is_in_use_uuid_valid() {
                    let current = cfg.in_use.unwrap();
                    if &current == id {
                        format!("{} (current, {}, {})", username, host, email)
                    } else {
                        format!("{} ({}, {})", username, host, email)
                    }
                } else {
                    format!("{} ({}, {})", username, host, email)
                }
            }
            config::Account::LinkedAccount {
                id, host, link_to, ..
            } => {
                let linked_alias_email = cfg.get_alias_by_id(link_to).map_or_else(
                    || String::from("UNKNOWN ALIAS"),
                    |alias| alias.email.clone(),
                );
                let linked_alias_username = cfg.get_alias_by_id(link_to).map_or_else(
                    || String::from("UNKNOWN ALIAS"),
                    |alias| alias.username.clone(),
                );
                if cfg.is_in_use_uuid_valid() {
                    let current = cfg.in_use.unwrap();
                    if &current == id {
                        format!(
                            "{} (current, {}, {})",
                            linked_alias_username, host, linked_alias_email
                        )
                    } else {
                        format!(
                            "{} ({}, {})",
                            linked_alias_username, host, linked_alias_email
                        )
                    }
                } else {
                    format!(
                        "{} ({}, {})",
                        linked_alias_username, host, linked_alias_email
                    )
                }
            }
        })
        .collect();

    let selection = Select::with_theme(&ColorfulTheme::default())
        .with_prompt("Select Account to use")
        .items(&selections)
        .default(0)
        .interact_opt()?;

    match selection {
        Some(index) => {
            let selected_account = &cfg.accounts[index];
            let (id, _, username, email, with_cred, signing_key) = match selected_account {
                config::Account::AccountDetails {
                    id,
                    host,
                    email,
                    username,
                    with_cred,
                    signing_key: details_signing_key,
                    ..
                } => (
                    id,
                    host.clone(),
                    username.clone(),
                    email.clone(),
                    with_cred,
                    details_signing_key.clone(),
                ),
                config::Account::LinkedAccount {
                    id,
                    host,
                    link_to,
                    with_cred,
                    ..
                } => {
                    let alias_option = cfg.get_alias_by_id(link_to);
                    let (username, email) = alias_option.as_ref().map_or_else(
                        || (String::from("UNKNOWN ALIAS"), String::from("UNKNOWN ALIAS")),
                        |alias| (alias.username.clone(), alias.email.clone()),
                    );

                    let signing_key_val = alias_option
                        .map_or_else(|| String::from("UNKNOWN ALIAS"), |alias| alias.signing_key.clone().unwrap_or(String::from("UNKNOWN ALIAS")));

                    let signing_key_option: Option<String> = if signing_key_val == "UNKNOWN ALIAS" {
                        None
                    } else {
                        Some(signing_key_val)
                    };

                    (
                        id,
                        host.clone(),
                        username,
                        email,
                        with_cred,
                        signing_key_option,
                    )
                }
            };
            let id_deref = *id;

            cfg.in_use = Some(id_deref);
            let _ = cfg.save();
            if *with_cred {
                Command::new("git")
                    .arg("config")
                    .args(["--global", "credential.namespace", &format!("{}", id)])
                    .output()
                    .expect("failed to execute process");
            }
            if let Some(v) = signing_key {
                let _ = Command::new("git")
                    .arg("config")
                    .args(["--global", "user.signingkey", &format!("{}", v)])
                    .output()
                    .expect("failed to execute process");
            } else {
                let _ = Command::new("git")
                    .arg("config")
                    .args(["--global", "--unset", "user.signingkey"])
                    .output()
                    .expect("failed to execute process");
            }
            let _ = Command::new("git")
                .arg("config")
                .args(["--global", "user.name", &format!("{}", username)])
                .output()
                .expect("failed to execute process");
            let _ = Command::new("git")
                .arg("config")
                .args(["--global", "user.email", &format!("{}", email)])
                .output()
                .expect("failed to execute process");
            let _ = Command::new("git")
                .arg("config")
                .args(["--global", "user.email", &format!("{}", email)])
                .output()
                .expect("failed to execute process");
        }
        None => {}
    }

    Ok(())
}
