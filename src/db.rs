use rusqlite::{Connection, Result};
use std::env;
use std::path::PathBuf;

fn set_db_path() -> PathBuf {
    if let Ok(xdg_data_home) = env::var("XDG_DATA_HOME") {
        PathBuf::from(xdg_data_home).join("neix").join("neix.db")
    } else {
        let home = env::var("HOME").expect("HOME not set");
        PathBuf::from(home)
            .join(".local")
            .join("share")
            .join("neix")
            .join("neix.db")
    }
}

pub fn open_db() -> Result<Connection> {
    let path = set_db_path();

    if let Some(parent) = path.parent() {
        std::fs::create_dir_all(parent).unwrap();
    }

    let conn = Connection::open(path)?;

    conn.execute(
        r#"
        CREATE TABLE IF NOT EXISTS packages (
            name TEXT NOT NULL,
            version TEXT NOT NULL,
            description TEXT,
            PRIMARY KEY (name, version)
        )
        "#,
        [],
    )?;

    Ok(conn)
}

pub fn upsert_packages(
    conn: &Connection,
    name: &str,
    version: &str,
    description: &str,
) -> Result<()> {
    conn.execute(
        r#"
    INSERT INTO packages (name, version, description)
    VALUES (?1, ?2, ?3)
    ON CONFLICT(name, version) DO UPDATE SET
        description = excluded.description
    WHERE description IS NOT excluded.description
    "#,
        rusqlite::params![name, version, description],
    )?;

    Ok(())
}
