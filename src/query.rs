use crate::db::open_db;
use rusqlite::Result;

pub fn search(q: &str) -> Result<Vec<(String, Option<String>)>> {
    let conn = open_db()?;

    let mut stmt = conn.prepare(
        "
        SELECT name, description
        FROM packages
        WHERE name LIKE '%' || ?1 || '%'
        ORDER BY name
        LIMIT 50
        ",
    )?;

    let rows = stmt.query_map([q], |row| {
        let name: String = row.get(0)?;
        let description: Option<String> = row.get(1)?;
        Ok((name, description))
    })?;

    let mut results = Vec::new();
    for row in rows {
        results.push(row?);
    }

    Ok(results)
}
