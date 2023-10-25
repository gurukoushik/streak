mod art;
mod db;
use clap::{Parser, Subcommand};
use prettytable::{Cell, Row, Table, format};

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
    /// Remind about incomplete streaks for the day
    Remind {},
}

fn main() {
    let args = App::parse();
    match args.command {
        Command::Create { name } => {
            let conn = db::get_db_connection(db::STREAKS_DB_PATH);
            db::init_streaks_db(&conn);

            db::create_streak(&conn, &name);

            let mut table = Table::new();
            table.set_format(*format::consts::FORMAT_DEFAULT);
            table.add_row(Row::new(vec![
                Cell::new(
                    format!("Streak created for {} 🔥", name).as_str(),
                ),
            ]));
            table.printstd();
        }
        Command::Log { name } => {
            let conn = db::get_db_connection(db::STREAKS_DB_PATH);
            db::init_streaks_db(&conn);

            db::log_streak(&conn, &name);
            println!("Streak logged for {}!", name)
        }
        Command::List {} => {
            let conn = db::get_db_connection(db::STREAKS_DB_PATH);
            db::init_streaks_db(&conn);

            let streaks = db::list_streak(&conn);
            match streaks {
                Ok(s) => {
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
                    println!("No streaks found!");
                }
            }
        }
        Command::Remind {} => {
            let conn = db::get_db_connection(db::STREAKS_DB_PATH);
            db::init_streaks_db(&conn);

            let remind_streaks = db::remind_streaks(&conn);

            let mut table = Table::new();
            table.set_format(*format::consts::FORMAT_NO_LINESEP_WITH_TITLE);
            for streak in remind_streaks {
                table.add_row(Row::new(vec![
                    Cell::new(format!("{} 🔥", streak.name).as_str()).style_spec("bFg"),
                    Cell::new(
                        format!("{}", db::get_streak_count(&conn, streak.name.clone())).as_str(),
                    )
                    .style_spec("Fyc"),
                ]));
            }
            table.printstd();
        }
    }
}
