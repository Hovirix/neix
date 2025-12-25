use rusqlite::{Connection, Result, params};
use serde::Deserialize;
use std::{
    collections::HashMap,
    env,
    io::{self, Write},
    path::PathBuf,
    process::Command,
};

#[derive(Debug, Deserialize)]
struct PkgMeta {
    pname: String,
    version: Option<String>,
    description: Option<String>,
}

fn db_path() -> PathBuf {
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
    eprintln!("→ Opening database");
    let path = db_path();

    if let Some(parent) = path.parent() {
        std::fs::create_dir_all(parent).unwrap();
    }

    let conn = Connection::open(path)?;

    conn.execute_batch(
        r#"
        CREATE TABLE IF NOT EXISTS packages (
            attr TEXT PRIMARY KEY,
            name TEXT NOT NULL,
            version TEXT,
            description TEXT
        );

        CREATE INDEX IF NOT EXISTS idx_packages_name
        ON packages(name);
        "#,
    )?;

    eprintln!("✓ Database ready");
    Ok(conn)
}

pub fn upsert_package(
    conn: &Connection,
    attr: &str,
    name: &str,
    version: &str,
    description: &str,
) -> Result<()> {
    conn.execute(
        r#"
        INSERT INTO packages (attr, name, version, description)
        VALUES (?1, ?2, ?3, ?4)
        ON CONFLICT(attr) DO UPDATE SET
            name        = excluded.name,
            version     = excluded.version,
            description = excluded.description
        "#,
        params![attr, name, version, description],
    )?;
    Ok(())
}

type Index = HashMap<String, PkgMeta>;

pub fn update_db() -> Result<(), Box<dyn std::error::Error>> {
    eprintln!("→ Starting database update");
    io::stderr().flush().ok();

    let conn = open_db()?;

    eprintln!("→ Running `nix search` (this can take a while…)");
    io::stderr().flush().ok();

    let output = Command::new("nix")
        .args(["search", "nixpkgs", ".", "--json"])
        .output()?;

    eprintln!("✓ nix search finished");
    io::stderr().flush().ok();

    if !output.status.success() {
        eprintln!("{}", String::from_utf8_lossy(&output.stderr));
        return Err("nix search failed".into());
    }

    eprintln!("→ Parsing JSON output");
    io::stderr().flush().ok();

    let packages: Index = serde_json::from_slice(&output.stdout)?;
    let total = packages.len();

    eprintln!("→ Indexing {} packages", total);
    io::stderr().flush().ok();

    let mut count = 0usize;
    for (attr, meta) in packages {
        upsert_package(
            &conn,
            &attr,
            &meta.pname,
            meta.version.as_deref().unwrap_or(""),
            meta.description.as_deref().unwrap_or(""),
        )?;
        count += 1;

        if count.is_multiple_of(5000) {
            eprintln!("  indexed {}/{}", count, total);
            io::stderr().flush().ok();
        }
    }

    eprintln!("✓ Database update complete ({} packages)", count);
    io::stderr().flush().ok();

    Ok(())
}
