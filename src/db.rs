use rusqlite::{params, Connection, Result};

#[derive(Debug)]
struct Streak {
    id: i32,
    name: String,
}

fn get_streaks_db() -> Connection {
    let db_path = String::from("/usr/local/streak/streaks.db");
    Connection::open(db_path).expect("Streaks DB failed to connect!")
}
