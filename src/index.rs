use crate::db::{open_db, upsert_package};
use serde::Deserialize;
use std::collections::HashMap;
use std::process::Command;

#[derive(Debug, Deserialize)]
struct PkgMeta {
    pname: String,
    version: Option<String>,
    description: Option<String>,
}

// JSON shape:
// attr -> { pname, version, description }
type Index = HashMap<String, PkgMeta>;

pub fn update_index() -> Result<(), Box<dyn std::error::Error>> {
    let conn = open_db()?;

    let output = Command::new("nix")
        .args(["search", "nixpkgs", ".", "--json"])
        .output()?;

    if !output.status.success() {
        eprintln!("{}", String::from_utf8_lossy(&output.stderr));
        return Err("nix search failed".into());
    }

    let packages: Index = serde_json::from_slice(&output.stdout)?;

    for (attr, meta) in packages {
        upsert_package(
            &conn,
            &attr,
            &meta.pname,
            meta.version.as_deref().unwrap_or(""),
            meta.description.as_deref().unwrap_or(""),
        )?;
    }

    Ok(())
}
