use chrono;
use rusqlite::{Connection, Result};

pub static STREAKS_DB_PATH: &str = "streaks.db";
pub static STREAKS_TABLE_NAME: &str = "streaks";
pub static STREAKS_LOG_TABLE_NAME: &str = "streakslog";

#[derive(Debug)]
pub struct Streak {
    id: i32,
    pub name: String,
}

#[derive(Debug)]
pub struct LogStreak {
    id: i32,
    streakId: i32,
    timestamp_utc: chrono::DateTime<chrono::Utc>,
}

// TODO: put db in a non local path
pub fn get_db_connection(db_path: &str) -> Connection {
    let db_path = String::from(db_path);
    Connection::open(db_path.clone()).expect(format!("DB at {db_path} failed to connect!").as_str())
}

// TODO: parametrize to accept attrs object and construct query
pub fn create_streaks_table_if_not_exists(conn: &Connection, table_name: &str) {
    let query = format!(
        "CREATE TABLE IF NOT EXISTS {} (
        id INTEGER PRIMARY KEY,
        name TEXT
    )",
        table_name
    );
    conn.execute(query.as_str(), [])
        .expect("Failed to create table");
}

pub fn create_streaks_log_table_if_not_exists(conn: &Connection, table_name: &str) {
    let query = format!(
        "CREATE TABLE IF NOT EXISTS {} (
        id INTEGER PRIMARY KEY,
        name TEXT,
        timestamp_utc DATETIME
    )",
        table_name
    );
    conn.execute(query.as_str(), [])
        .expect("Failed to create table");
}

pub fn create_streak(conn: &Connection, name: &String) {
    conn.execute("INSERT INTO streaks (name) VALUES (?1)", &[&name])
        .expect("Failed to add streak!");
}

pub fn list_streak(conn: &Connection) -> Result<Vec<Streak>> {
    let query = "SELECT id, name FROM streaks";
    let mut stmt = conn.prepare(query).expect("Failed to run query!");
    let entries = stmt
        .query_map([], |row| {
            Ok(Streak {
                id: row.get(0)?,
                name: row.get(1)?,
            })
        })
        .expect("Failed to extract entries!")
        .collect::<Result<Vec<_>>>();
    entries
}

pub fn log_streak(conn: &Connection, name: &String) {
    let current_timestamp = chrono::offset::Utc::now();
    println!("{}", current_timestamp);
    conn.execute("INSERT INTO streakslog (name, timestamp_utc) VALUES (?1, ?2)", &[&name, &current_timestamp.to_string()]).expect("Failed to log streak!");
}
