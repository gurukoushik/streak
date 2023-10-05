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
            let conn = db::get_db_connection(db::STREAKS_DB_PATH);
            db::create_streaks_table_if_not_exists(&conn, db::STREAKS_TABLE_NAME);
            db::create_streak(&conn, &name);
            println!("{}", art::rhino());
            println!("Streak for {} created!", name);
        }
        Command::Log { name } => {
            let conn = db::get_db_connection(db::STREAKS_DB_PATH);
            db::create_streaks_log_table_if_not_exists(&conn, db::STREAKS_LOG_TABLE_NAME);
            db::log_streak(&conn, &name);
            println!("{}", art::jordan());
            println!("Streak logged for {}!", name)
        }
        Command::List {} => {
            let conn = db::get_db_connection(db::STREAKS_DB_PATH);
            db::create_streaks_table_if_not_exists(&conn, db::STREAKS_TABLE_NAME);
            let streaks = db::list_streak(&conn);
            match streaks {
                Ok(s) => {
                    for streak in s {
                        println!("{}", streak.name)
                    }
                }
                Err(_) => {
                    println!("No streaks found!");
                }
            }
        }
    }
}
