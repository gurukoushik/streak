use clap::{Parser, Subcommand};

/// streak (noun) [/strēk/]: a continuous period of specified success or luck.
/// Create, view and track streaks to develop lasting habits by creating 
/// positive reward signals. 
#[derive(Debug, Parser)]
#[clap(name = "streak", version)]
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
}

fn main () {
    let app = App::parse();
    println!("{:?}", app);
}
