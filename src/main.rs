mod config;
mod cmds;
mod git;

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
    #[arg(short, long, global = true, help = "Enable verbose output for detailed execution information.")]
    verbose: bool,

    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand, Debug)]
enum Commands {
    #[command(
        about = "Show all avaliable profile and current profile.", 
    )]
    List {},
    #[command(
        about = "Change the current profile to the selected one.", 
    )]
    Use {
        #[arg(help = "The slug (filename) of the profile you want to use. If left blank, it resets the current profile.")]
        slug: Option<String>,
        #[clap(long, short, action)]
        #[arg(help = "If set, this change will apply globally.")]
        global: bool,
    },
    #[command(
        about = "Create profile.", 
    )]
    New {},
    #[command(
        about = "Delete profile.", 
    )]
    Delete {
        #[arg(help = "The slug (filename) of the profile you want to delete.")]
        slug: String,
    }
}

fn main() {
    let cli = Cli::parse();

    match cli.command {
        Commands::List {} => {
            cmds::list::list_profiles()
        }
        Commands::Use { slug, global } => {
            cmds::use_profile::use_profile(slug, global);
        },
        Commands::New {} => {
            cmds::new::new_profile();
        },
        Commands::Delete { slug } => {
            cmds::del::delete_profile(slug);
        }
    }
}