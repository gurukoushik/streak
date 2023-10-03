mod art;
mod db;
use clap::{Parser, Subcommand};

#[derive(Debug, Parser)]
#[clap(author, version, about, long_about = None)]
pub struct App {
    #[clap(subcommand)]
    command: Command,
}

#[derive(Debug, Subcommand)]
enum Command {
    /// Create new streak
    Create {
        /// Name of the habit to create a streak
        name: String,
    },
    /// Log streak for the day
    Log {
        /// Name of the habit to log
        name: String,
    },
    /// List all the streaks
    List {},
}

fn main() {
    let args = App::parse();
    match args.command {
        Command::Create { name } => {
            println!("{}", art::rhino());
            println!("Streak for {} created!", name)
        }
        Command::Log { name } => {
            println!("{}", art::jordan());
            println!("Streak logged for {}!", name)
        }
        Command::List {} => {
            println!("list")
        }
    }
}
