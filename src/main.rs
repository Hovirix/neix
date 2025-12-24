use clap::Parser;
use std::process;

mod db;
mod index;
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
        if let Err(e) = index::update_index() {
            eprintln!("index update failed: {e}");
            process::exit(1);
        }
        println!("index updated");
        return;
    }

    let query = match args.query {
        Some(q) => q,
        None => {
            eprintln!("usage: neix <query> | --update");
            process::exit(1);
        }
    };

    let results = match query::search(&query) {
        Ok(r) => r,
        Err(e) => {
            eprintln!("search failed: {e}");
            process::exit(1);
        }
    };

    for (name, description) in results {
        println!("{:<30} {}", name, description.unwrap_or_default());
    }
}
