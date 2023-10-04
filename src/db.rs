use rusqlite::{params, Connection, Result};

pub static STREAKS_DB_PATH: &str = "streaks.db";
pub static STREAKS_TABLE_NAME: &str = "streaks";

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

// TODO: parametrize to accept any table and attrs
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

pub fn create_streak(conn: &Connection, name: &String) {
    conn.execute("INSERT INTO streaks (name) VALUES (?1)", &[&name])
        .expect("Failed to add streak!");
}

pub fn list_streak(conn: &Connection) {
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
    for entry in entries {
        println!("{:?}", entry);
    }
}
