mod config;
mod git_ops;
mod ui;

use config::Config;

fn main() -> std::io::Result<()> {
    let mut cfg: Config = match config::load_config() {
        Ok(c) => c,
        Err(e) => {
            panic!("Failed to load config!\n{}", e.to_string());
        }
    };

    let selection = ui::get_account_selection(&cfg)?;

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

                    let signing_key_val = alias_option.map_or_else(
                        || String::from("UNKNOWN ALIAS"),
                        |alias| {
                            alias
                                .signing_key
                                .clone()
                                .unwrap_or(String::from("UNKNOWN ALIAS"))
                        },
                    );

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

            git_ops::update_git_config(&id_deref, &username, &email, *with_cred, &signing_key)?;
        }
        None => {}
    }

    Ok(())
}
