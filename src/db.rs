use rusqlite::{Connection, Result};

pub static STREAKS_DB_PATH: &str = "streaks.db";
pub static STREAKS_TABLE_NAME: &str = "streaks";

#[derive(Debug)]
pub struct Streak {
    id: i32,
    pub name: String,
}

// TODO: put db in a non local path
pub fn get_db_connection(db_path: &str) -> Connection {
    let db_path = String::from(db_path);
    Connection::open(db_path.clone()).expect(format!("DB at {db_path} failed to connect!").as_str())
}

// TODO: parametrize to accept attrs object and construct query
pub fn create_table_if_not_exists(conn: &Connection, table_name: &str) {
    conn.execute(
        "CREATE TABLE IF NOT EXISTS {table_name} (
            id INTEGER PRIMARY KEY,
            name TEXT
        )",
        [&table_name],
    )
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
