mod cmds;
mod config;
mod git;
mod keyring;
mod oauth;

use clap::{Parser, Subcommand};

#[derive(Parser, Debug)]
#[command(
    name = env!("CARGO_PKG_NAME"),
    version = env!("CARGO_PKG_VERSION"),
    author = env!("CARGO_PKG_AUTHORS"),
    about = env!("CARGO_PKG_DESCRIPTION"),
    arg_required_else_help = true,
)]
struct Cli {
    #[arg(
        short,
        long,
        global = true,
        help = "Enable verbose output for detailed execution information."
    )]
    verbose: bool,

    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand, Debug)]
enum Commands {
    #[command(about = "Show all available profiles.")]
    List {},
    #[command(about = "Show the current profile and login status.")]
    Current {},
    #[command(about = "Change the current profile to the selected one.")]
    Use {
        #[arg(
            help = "The slug (filename) of the profile you want to use. If left blank, it resets the current profile."
        )]
        slug: Option<String>,
        #[clap(long, short, action)]
        #[arg(help = "If set, this change will apply to the local repository.")]
        local: bool,
    },
    #[command(about = "Create profile.")]
    New {},
    #[command(about = "Delete profile.")]
    Delete {
        #[arg(help = "The slug (filename) of the profile you want to delete.")]
        slug: String,
    },
    #[command(about = "Manage credentials for profiles.")]
    Cred {
        #[command(subcommand)]
        command: cmds::cred::CredCommands,
    },
}

fn main() {
    // Initialize configuration and migrate old profiles on startup
    if let Err(e) = config::migrate_old_profiles() {
        eprintln!("Warning: Failed to migrate old profiles: {}", e);
    }

    // Load and initialize OAuth configuration
    match config::load_config() {
        Ok(_) => {
            // Configuration loaded or created successfully
        }
        Err(e) => {
            eprintln!("Warning: Failed to load OAuth configuration: {}", e);
        }
    }

    let cli = Cli::parse();

    match cli.command {
        Commands::List {} => cmds::list::list_profiles(),
        Commands::Current {} => cmds::list::show_current_profile(),
        Commands::Use { slug, local } => {
            cmds::use_profile::use_profile(slug, local);
        }
        Commands::New {} => {
            cmds::new::new_profile();
        }
        Commands::Delete { slug } => {
            cmds::delete::delete_profile(slug);
        }
        Commands::Cred { command } => {
            cmds::cred::handle_cred_command(command);
        }
    }
}
