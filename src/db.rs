use rusqlite::{params, Connection, Result};

#[derive(Debug)]
struct Streak {
    id: i32,
    name: String,
}

// TODO: put db in a non local path
pub fn get_db_connection(db_path: &str) -> Connection {
    let db_path = String::from(db_path);
    Connection::open(db_path.clone()).expect(format!("DB at {db_path} failed to connect!").as_str())
}

pub fn create_table_if_not_exists(conn: &Connection) {
    conn.execute(
        "CREATE TABLE IF NOT EXISTS streaks (
            id INTEGER PRIMARY KEY,
            name TEXT
        )",
        [],
    )
    .expect("Failed to create table");
}
