pub mod debug;
pub mod helper;
pub mod list;
pub mod revoke;
pub mod setup;

use clap::Subcommand;

#[derive(Subcommand, Debug)]
pub enum CredCommands {
    #[command(about = "Setup OAuth2 authentication for a host")]
    Setup {
        #[arg(help = "The slug of the profile")]
        profile: String,
        #[arg(long, help = "The Git host (e.g., github.com, gitlab.com)")]
        host: Option<String>,
    },
    #[command(about = "List credentials for a profile")]
    List {
        #[arg(help = "The slug of the profile")]
        profile: String,
    },
    #[command(about = "Revoke a credential")]
    Revoke {
        #[arg(help = "The slug of the profile")]
        profile: String,
        #[arg(long, help = "The Git host to revoke")]
        host: Option<String>,
    },
    #[command(about = "Debug credential storage and retrieval")]
    Debug {
        #[arg(help = "The profile ID (UUID) or slug to debug")]
        profile_id: String,
        #[arg(long, help = "The Git host to debug")]
        host: Option<String>,
    },
    #[command(about = "Git credential helper protocol handler")]
    Helper {
        #[command(subcommand)]
        command: HelperCommands,
    },
}

#[derive(Subcommand, Debug)]
pub enum HelperCommands {
    #[command(about = "Get credentials")]
    Get,
    #[command(about = "Store credentials")]
    Store,
    #[command(about = "Erase credentials")]
    Erase,
}

pub fn handle_cred_command(cmd: CredCommands) {
    match cmd {
        CredCommands::Setup { profile, host } => {
            setup::setup_credentials(profile, host);
        }
        CredCommands::List { profile } => {
            list::list_credentials(profile);
        }
        CredCommands::Revoke { profile, host } => {
            revoke::revoke_credential(profile, host);
        }
        CredCommands::Debug { profile_id, host } => {
            debug::debug_credential(profile_id, host);
        }
        CredCommands::Helper { command } => match command {
            HelperCommands::Get => helper::helper_get(),
            HelperCommands::Store => helper::helper_store(),
            HelperCommands::Erase => helper::helper_erase(),
        },
    }
}
