use crate::config;

use dialoguer::{Error as DialoguerError, Select, theme::ColorfulTheme};
use std::io::{self, ErrorKind};

pub fn get_account_selection(cfg: &config::Config) -> std::io::Result<Option<usize>> {
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

    let selection_result = Select::with_theme(&ColorfulTheme::default())
        .with_prompt("Select Account to use")
        .items(&selections)
        .default(0)
        .interact_opt()
        .map_err(|e: DialoguerError| {
            io::Error::new(ErrorKind::Other, format!("Dialoguer error: {}", e))
        })?;
    
    Ok(selection_result)
}
