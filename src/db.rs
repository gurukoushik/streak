use rusqlite::{params, Connection, Result};

#[derive(Debug)]
struct Streak {
    id: i32,
    name: String,
}

// TODO: parametrize db path
// TODO: put db in a non local path
pub fn get_db_connection() -> Connection {
    let db_path = String::from("streaks.db");
    Connection::open(db_path).expect("Streaks DB failed to connect!")
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
