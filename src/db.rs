use anyhow::{Context, Result};
use rusqlite::{Connection, Transaction, params};
use serde::Deserialize;
use std::{collections::HashMap, env, fs, path::PathBuf, process::Command};

/// Returns the path to the database file, respecting XDG_DATA_HOME if set.
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

/// Opens a connection to the SQLite database.
/// Creates the database file and necessary tables if they don't exist.
pub fn open_db() -> Result<Connection> {
    eprintln!("→ Opening database");
    let path = db_path();

    if let Some(parent) = path.parent() {
        fs::create_dir_all(parent)
            .with_context(|| format!("Failed to create directory for database at {:?}", parent))?;
    }

    let conn = Connection::open(&path)
        .with_context(|| format!("Failed to open database at {:?}", path))?;

    conn.execute_batch(
        r#"
        PRAGMA journal_mode = WAL;
        PRAGMA synchronous = NORMAL;
        PRAGMA cache_size = -10000; -- 10MB cache
        "#,
    )
    .with_context(|| "Failed to set PRAGMA")?;

    conn.execute_batch(
        r#"
        CREATE TABLE IF NOT EXISTS packages (
            attr TEXT PRIMARY KEY,
            name TEXT NOT NULL,
            version TEXT,
            description TEXT
        );
        "#,
    )
    .with_context(|| "Failed to create table")?;

    eprintln!("✓ Database ready");
    Ok(conn)
}

/// Updates the database with the latest package information.
pub fn update_db() -> Result<()> {
    eprintln!("→ Starting database update");

    let mut conn = open_db().with_context(|| "Failed to open database")?;
    let tx = conn
        .transaction()
        .with_context(|| "Failed to start transaction")?;

    tx.execute("DROP INDEX IF EXISTS idx_packages_name", [])
        .with_context(|| "Failed to drop index")?;

    eprintln!("→ Fetching and parsing package data");
    let packages = parse_nix_search().with_context(|| "Failed to parse Nix search output")?;
    let total = packages.len();
    eprintln!("→ Indexing {} packages", total);

    insert_packages(&tx, &packages).with_context(|| "Failed to insert packages")?;
    tx.execute("CREATE INDEX idx_packages_name ON packages(name)", [])
        .with_context(|| "Failed to create index")?;
    tx.commit()
        .with_context(|| "Failed to commit transaction")?;

    eprintln!("✓ Database update complete ({} packages)", total);
    Ok(())
}

type Index = HashMap<String, PkgMeta>;

#[derive(Debug, Deserialize)]
struct PkgMeta {
    pname: String,
    version: Option<String>,
    description: Option<String>,
}

/// Runs `nix search` and returns the JSON output as bytes.
fn run_nix_search() -> Result<Vec<u8>> {
    eprintln!("→ Running `nix search` (this can take a while…)");

    let output = Command::new("nix")
        .args(["search", "nixpkgs", ".", "--json"])
        .output()
        .with_context(|| "Failed to run `nix search`")?;

    if !output.status.success() {
        eprintln!("{}", String::from_utf8_lossy(&output.stderr));
        anyhow::bail!("`nix search` failed");
    }

    eprintln!("✓ nix search finished");
    Ok(output.stdout)
}

/// Parses the output of `nix search` into a HashMap of package metadata.
fn parse_nix_search() -> Result<Index> {
    let output = run_nix_search()?;
    let packages: Index =
        serde_json::from_slice(&output).with_context(|| "Failed to parse JSON output")?;
    Ok(packages)
}

/// Inserts or updates packages in the database using a transaction.
fn insert_packages(tx: &Transaction, packages: &Index) -> Result<()> {
    let mut stmt = tx
        .prepare(
            r#"
        INSERT INTO packages (attr, name, version, description)
        VALUES (?1, ?2, ?3, ?4)
        ON CONFLICT(attr) DO UPDATE SET
            name = excluded.name,
            version = excluded.version,
            description = excluded.description
        "#,
        )
        .with_context(|| "Failed to prepare SQL statement")?;

    for (attr, meta) in packages {
        stmt.execute(params![
            attr,
            meta.pname,
            meta.version.as_deref().unwrap_or(""),
            meta.description.as_deref().unwrap_or(""),
        ])
        .with_context(|| format!("Failed to insert package: {}", attr))?;
    }

    Ok(())
}
