use crate::db::{open_db, upsert_packages};
use serde::Deserialize;
use std::collections::HashMap;
use std::process::Command;

#[derive(Debug, Deserialize)]
struct PkgMeta {
    version: Option<String>,
    description: Option<String>,
}

type Index = HashMap<String, Option<PkgMeta>>;

pub fn update_index() -> Result<(), Box<dyn std::error::Error>> {
    let conn = open_db()?;

    let output = Command::new("nix")
        .args(["eval", "--json", "--file", "src/query.nix"])
        .output()?;

    if !output.status.success() {
        eprintln!("nix eval failed");
        eprintln!("stdout:\n{}", String::from_utf8_lossy(&output.stdout));
        eprintln!("stderr:\n{}", String::from_utf8_lossy(&output.stderr));
        return Err("nix eval failed".into());
    }
    let packages: Index = serde_json::from_slice(&output.stdout)?;

    for (name, pkg_opt) in packages {
        let Some(pkg) = pkg_opt else { continue };

        upsert_packages(
            &conn,
            &name,
            pkg.version.as_deref().unwrap_or(""),
            pkg.description.as_deref().unwrap_or(""),
        )?;
    }

    Ok(())
}
