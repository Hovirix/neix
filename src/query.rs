use crate::db::open_db;
use rusqlite::Result;

pub fn search(q: &str) -> Result<Vec<(String, Option<String>, Option<String>)>> {
    let conn = open_db()?;

    let mut stmt = conn.prepare(
        r#"
        SELECT name, version, description
        FROM packages
        WHERE name LIKE '%' || ?1 || '%'
           OR description LIKE '%' || ?1 || '%'
        ORDER BY name
        LIMIT 50
        "#,
    )?;

    let rows = stmt.query_map([q], |row| {
        Ok((
            row.get::<_, String>(0)?,         // name
            row.get::<_, Option<String>>(1)?, // version
            row.get::<_, Option<String>>(2)?, // description
        ))
    })?;

    let mut results = Vec::new();
    for row in rows {
        results.push(row?);
    }

    Ok(results)
}
