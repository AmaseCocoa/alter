use std::process::Command;

pub fn update_git_config(
    account_id: &uuid::Uuid,
    username: &str,
    email: &str,
    with_cred: bool,
    signing_key: &Option<String>,
) -> std::io::Result<()> {
    if with_cred {
        Command::new("git")
            .arg("config")
            .args(["--global", "credential.namespace", &format!("{}", account_id)])
            .output()?;
    }

    if let Some(v) = signing_key {
        Command::new("git")
            .arg("config")
            .args(["--global", "user.signingkey", v])
            .output()?;
    } else {
        Command::new("git")
            .arg("config")
            .args(["--global", "--unset", "user.signingkey"])
            .output()?;
    }
    
    Command::new("git")
        .arg("config")
        .args(["--global", "user.name", username])
        .output()?;

    Command::new("git")
        .arg("config")
        .args(["--global", "user.email", email])
        .output()?;
    Ok(())
}