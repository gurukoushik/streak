mod db;
use clap::{Parser, Subcommand};
use colored::*;
use prettytable::{format, Cell, Row, Table};
use std::process::exit;
use std::{io, str::FromStr};

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
        /// Frequency of the streak (Alldays, Weekdays)
        frequency: Option<String>,
    },
    /// Log streak for the day
    Log {
        /// Name of the habit to log
        name: String,
    },
    /// List all the streaks
    List {},
    /// Remind about incomplete streaks for the day
    Remind {},
    /// Reset all data (WARNING: This is irreversible and will delete all data)
    Reset {},
}

fn main() {
    let args = App::parse();
    match args.command {
        Command::Create { name, frequency } => {
            let conn = db::get_db_connection(&db::get_db_path());
            db::init_streaks_db(&conn);

            let frequency = match frequency {
                Some(f) => match db::StreakFrequency::from_str(&f) {
                    Ok(f) => f,
                    Err(_) => {
                        println!("Invalid frequency provided! Valid values are: Alldays, Weekdays");
                        exit(1);
                    }
                },
                None => db::StreakFrequency::Alldays,
            };
            db::create_streak(&conn, &name, &frequency);

            let mut table = Table::new();
            table.set_format(*format::consts::FORMAT_DEFAULT);
            table.add_row(Row::new(vec![Cell::new(
                format!("Streak created for {} 🔥", name).as_str(),
            )]));
            table.printstd();
        }
        Command::Log { name } => {
            let conn = db::get_db_connection(&db::get_db_path());
            db::init_streaks_db(&conn);

            match db::log_streak(&conn, &name) {
                Ok(_) => {
                    let mut table = Table::new();
                    table.set_format(*format::consts::FORMAT_NO_LINESEP_WITH_TITLE);
                    table.add_row(Row::new(vec![
                        Cell::new(format!("Streak logged for {}!", name).as_str())
                            .style_spec("bFg"),
                        Cell::new(
                            format!("{} 🔥", db::get_streak_count(&conn, name.clone())).as_str(),
                        )
                        .style_spec("Fyc"),
                    ]));
                    table.printstd();
                }
                Err(_) => {
                    println!("{}", format!("No streak found with name: {}", name).red());
                    exit(1);
                }
            };
        }
        Command::List {} => {
            let conn = db::get_db_connection(&db::get_db_path());
            db::init_streaks_db(&conn);

            let streaks = db::list_streak(&conn);
            match streaks {
                Ok(s) => {
                    if s.len() == 0 {
                        println!("{}", "No streaks found!".red());
                        exit(0);
                    }
                    let mut table = Table::new();
                    table.set_format(*format::consts::FORMAT_NO_LINESEP_WITH_TITLE);
                    for streak in s {
                        table.add_row(Row::new(vec![
                            Cell::new(format!("{}", streak.name).as_str()).style_spec("bFg"),
                            Cell::new(
                                format!("{} 🔥", db::get_streak_count(&conn, streak.name.clone()))
                                    .as_str(),
                            )
                            .style_spec("Fyc"),
                        ]));
                    }
                    table.printstd();
                }
                Err(_) => {
                    println!("{}", "No streaks found!".red());
                    exit(1);
                }
            }
        }
        Command::Remind {} => {
            let conn = db::get_db_connection(&db::get_db_path());
            db::init_streaks_db(&conn);

            let remind_streaks = db::remind_streaks(&conn);
            if remind_streaks.len() == 0 {
                println!("{}", "Good job! All streaks completed for the day!".green());
                exit(0);
            }

            let mut table = Table::new();
            table.set_format(*format::consts::FORMAT_NO_LINESEP_WITH_TITLE);
            for streak in remind_streaks {
                table.add_row(Row::new(vec![
                    Cell::new(format!("{}", streak.name).as_str()).style_spec("bFg"),
                    Cell::new(
                        format!("{} 🔥", db::get_streak_count(&conn, streak.name.clone())).as_str(),
                    )
                    .style_spec("Fyc"),
                ]));
            }
            table.printstd();
        }
        Command::Reset {} => {
            println!(
                "Are you sure you want to reset all the data? ({}/{})",
                "y".green(),
                "n".red()
            );

            let mut input = String::new();
            io::stdin()
                .read_line(&mut input)
                .expect("Failed to read line");

            let confirmation = input.trim().to_lowercase();
            if confirmation == "y" {
                let db_path = db::get_db_path();
                match std::fs::remove_file(db_path) {
                    Ok(_) => {
                        println!("{}", "Streak data reset successfully.".green());
                    }
                    Err(_) => {
                        println!("{}", "Streak data reset successfully.".green());
                    }
                };
            } else {
                println!("{}", "Streak data reset canceled.".red());
            }
        }
    }
}
