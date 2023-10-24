use chrono::{self, DateTime, FixedOffset};
use rusqlite::{Connection, Result};

pub static STREAKS_DB_PATH: &str = "streaks.db";
pub static STREAKS_TABLE_NAME: &str = "streaks";
pub static STREAKS_LOG_TABLE_NAME: &str = "streakslog";

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

pub fn init_streaks_db(conn: &Connection) {
    create_streaks_table_if_not_exists(conn, STREAKS_TABLE_NAME);
    create_streaks_log_table_if_not_exists(conn, STREAKS_LOG_TABLE_NAME);
}

// TODO: parametrize to accept attrs object and construct query
pub fn create_streaks_table_if_not_exists(conn: &Connection, table_name: &str) {
    let query = format!(
        "CREATE TABLE IF NOT EXISTS {} (
        id INTEGER PRIMARY KEY,
        name TEXT UNIQUE
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
        streak_id INTEGER,
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
    let q = format!("SELECT id FROM streaks WHERE name = '{}'", name);
    let mut stmt = conn.prepare(q.as_str()).expect("Streak was not created!");
    let rows = stmt
        .query_map([], |row| Ok(row.get::<usize, i32>(0)))
        .expect("msg");
    let id_list: Vec<i32> = rows.filter_map(|result| result.ok()).flatten().collect();

    if !id_list.is_empty() {
        let current_timestamp = chrono::offset::Utc::now();
        conn.execute(
            "INSERT INTO streakslog (name, streak_id, timestamp_utc) VALUES (?1, ?2, ?3)",
            &[
                &name,
                &id_list[0].to_string(),
                &current_timestamp.to_rfc3339(),
            ],
        )
        .expect("Failed to log streak!");
    } else {
        println!("No streak with name {name}!");
    }
}

pub fn remind_streaks(conn: &Connection) -> Vec<Streak> {
    let current_timestamp = chrono::offset::Utc::now();
    let query = format!(
        "SELECT streak_id FROM streakslog WHERE substr(timestamp_utc, 1, 10) = substr('{}', 1, 10)",
        current_timestamp.to_string()
    );
    let mut stmt = conn.prepare(query.as_str()).expect("Failed to run query!");
    let logged_streaks = stmt
        .query_map([], |row| Ok(row.get::<_, i32>(0)?))
        .expect("Failed to extract entries!")
        .collect::<Result<Vec<_>>>()
        .expect("Failed to get logged streaks!");
    let all_streaks = list_streak(conn).expect("Failed to get all streaks!");

    let mut streaks_to_remind = Vec::new();
    for streak in all_streaks {
        if !logged_streaks.contains(&streak.id) {
            streaks_to_remind.push(streak);
        }
    }
    streaks_to_remind
}

// Get how many days in a row a streak has been logged
pub fn get_streak_count(conn: &Connection, streak_name: String) -> i32 {
    let query = format!(
        "SELECT timestamp_utc FROM streakslog WHERE name = '{}' ORDER BY timestamp_utc DESC",
        streak_name,
    );
    let mut stmt = conn.prepare(query.as_str()).expect("Failed to run query!");
    let streak_timestamps: Vec<chrono::DateTime<FixedOffset>> = stmt
        .query_map([], |row| {
            Ok(chrono::DateTime::parse_from_rfc3339(
                row.get::<_, String>(0)?.as_str(),
            ))
        })
        .expect("Failed to extract entries!")
        .collect::<Result<Vec<_>>>()
        .expect("Failed to get streak timestamps!")
        .into_iter()
        .filter_map(Result::ok)
        .collect();

    let current_timestamp = chrono::offset::Utc::now();
    calculate_streak_count(streak_timestamps, current_timestamp.into())
}

pub fn calculate_streak_count(
    timestamps: Vec<DateTime<FixedOffset>>,
    current_timestamp: DateTime<FixedOffset>,
) -> i32 {
    if timestamps.len() == 0 {
        return 0;
    }

    let mut timestamp = current_timestamp;
    let mut streak_count = 0;

    for ts in timestamps {
        let duration = timestamp.signed_duration_since(ts);
        if duration.num_days() == 0 {
            if streak_count == 0 {
                streak_count = 1;
            }
        } else if duration.num_days() == 1 {
            streak_count += 1;
        } else {
            break;
        }
        timestamp = ts.into();
    }

    streak_count
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_streak_count_1() {
        let current_timestamp =
            chrono::DateTime::parse_from_rfc3339("2021-01-13T00:00:00+00:00").unwrap();
        let timestamps = vec![
            chrono::DateTime::parse_from_rfc3339("2021-01-13T00:00:00+00:00").unwrap(),
            chrono::DateTime::parse_from_rfc3339("2021-01-12T00:00:00+00:00").unwrap(),
            chrono::DateTime::parse_from_rfc3339("2021-01-11T00:00:00+00:00").unwrap(),
            chrono::DateTime::parse_from_rfc3339("2021-01-01T00:00:00+00:00").unwrap(),
        ];
        assert_eq!(calculate_streak_count(timestamps, current_timestamp), 3);
    }

    #[test]
    fn test_streak_count_2() {
        let current_timestamp =
            chrono::DateTime::parse_from_rfc3339("2021-01-13T00:00:00+00:00").unwrap();
        let timestamps = vec![
            chrono::DateTime::parse_from_rfc3339("2021-01-12T00:00:00+00:00").unwrap(),
            chrono::DateTime::parse_from_rfc3339("2021-01-10T00:00:00+00:00").unwrap(),
            chrono::DateTime::parse_from_rfc3339("2021-01-09T00:00:00+00:00").unwrap(),
        ];
        assert_eq!(calculate_streak_count(timestamps, current_timestamp), 1);
    }

    #[test]
    fn test_streak_count_3() {
        let current_timestamp =
            chrono::DateTime::parse_from_rfc3339("2021-01-13T00:00:00+00:00").unwrap();
        let timestamps =
            vec![chrono::DateTime::parse_from_rfc3339("2021-01-10T00:00:00+00:00").unwrap()];
        assert_eq!(calculate_streak_count(timestamps, current_timestamp), 0);
    }
}
