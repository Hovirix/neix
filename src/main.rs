use clap::Parser;
use colored::*;
use std::process;

mod db;
mod query;

#[derive(Parser, Debug)]
#[command(name = "neix")]
#[command(version)]
#[command(about = "Blazing fast eix-like search for nixpkgs")]
struct Args {
    #[arg(long)]
    update: bool,

    #[arg(short, long, default_value = "10")]
    limit: isize,
    query: Option<String>,
}

fn main() {
    let args = Args::parse();

    if args.update {
        if let Err(e) = db::update_db() {
            eprintln!(
                "{} {}",
                "✗".red().bold(),
                format!("index update failed: {e}").red()
            );
            process::exit(1);
        }
        println!("{} {}", "✓".green().bold(), "index updated".green());
    }

    if let Some(q) = args.query {
        match query::query(&q, args.limit.try_into().unwrap()) {
            Ok(results) => {
                if results.is_empty() {
                    println!("{}", "No packages found".yellow());
                    println!("{}", "Run neix --update to create the database".yellow());
                } else {
                    println!(
                        "{} {} {}\n",
                        "Found".bright_blue().bold(),
                        results.len().to_string().bright_cyan().bold(),
                        "package(s):".bright_blue().bold()
                    );

                    for (i, pkg) in results.iter().enumerate() {
                        let version = pkg.version.as_deref().unwrap_or("no version");
                        let description = pkg.description.as_deref().unwrap_or("no description");

                        println!(
                            "{} {}",
                            format!("[{}]", i + 1).bright_black(),
                            pkg.attr.bright_green().bold()
                        );
                        println!("  {} {}", "Name:".bright_blue(), pkg.name.bright_white());
                        println!("  {} {}", "Version:".bright_blue(), version.cyan());
                        println!("  {} {}", "Description:".bright_blue(), description.white());

                        if i < results.len() - 1 {
                            println!();
                        }
                    }
                }
            }
            Err(e) => {
                eprintln!(
                    "{} {}",
                    "✗".red().bold(),
                    format!("query failed: {e}").red()
                );
                process::exit(1);
            }
        }
    }
}
