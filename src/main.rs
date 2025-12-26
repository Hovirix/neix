use clap::Parser;
use std::process;

mod db;
mod query;

#[derive(Parser, Debug)]
#[command(name = "neix")]
#[command(version)]
#[command(about = "Fast eix-like search for nixpkgs")]
struct Args {
    #[arg(long)]
    update: bool,
    query: Option<String>,
}

fn main() {
    let args = Args::parse();

    if args.update {
        if let Err(e) = db::update_db() {
            eprintln!("index update failed: {e}");
            process::exit(1);
        }
        println!("index updated");
    }

    if let Some(q) = args.query {
        match query::query(&q) {
            Ok(results) => {
                if results.is_empty() {
                    println!("No packages found");
                } else {
                    for pkg in results {
                        let version = pkg.version.unwrap_or_default();
                        let description = pkg.description.unwrap_or_default();
                        println!("{} [{}] - {}", pkg.attr, version, description);
                    }
                }
            }
            Err(e) => {
                eprintln!("query failed: {e}");
                process::exit(1);
            }
        }
    }
}
