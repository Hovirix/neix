use crate::db::open_db;
use anyhow::{Context, Result};
use rusqlite::params;

#[derive(Debug)]
pub struct PackageInfo {
    pub attr: String,
    pub name: String,
    pub version: Option<String>,
    pub description: Option<String>,
}

pub fn query(q: &str, limit: usize) -> Result<Vec<PackageInfo>> {
    let conn = open_db().with_context(|| "Failed to open database")?;

    let exact = q;
    let prefix = format!("{}%", q);
    let substring = format!("%{}%", q);

    let mut stmt = conn.prepare(
        r#"
        SELECT p.attr, p.name, p.version, p.description
        FROM packages p
        JOIN (
            SELECT name, MAX(version) AS version
            FROM packages
            GROUP BY name
        ) latest
        ON p.name = latest.name AND p.version = latest.version
        WHERE p.name LIKE ?3
        ORDER BY
            CASE
                WHEN p.name = ?1 THEN 0
                WHEN p.name LIKE ?2 THEN 1
                ELSE 2
            END,
            p.attr
        LIMIT ?4
        "#,
    )?;

    let rows = stmt.query_map(params![exact, prefix, substring, limit as i64], |row| {
        Ok(PackageInfo {
            attr: row.get(0)?,
            name: row.get(1)?,
            version: row.get(2)?,
            description: row.get(3)?,
        })
    })?;

    let mut results = Vec::new();
    for row in rows {
        results.push(row?);
    }

    Ok(results)
}
