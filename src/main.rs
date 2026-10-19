mod checker;
mod generator;
mod parser;
mod scanner;

use clap::{Parser, Subcommand};
use colored::*;
use std::path::PathBuf;
use std::process;

#[derive(Parser)]
#[command(
    name = "envpatrol",
    version,
    about = "🛡️  A CLI tool to check .env sync, detect leaked secrets, and auto-generate .env.example",
    long_about = None
)]
struct Cli {
    #[command(subcommand)]
    command: Option<Commands>,
}

#[derive(Subcommand)]
enum Commands {
    /// Check whether .env and .env.example are in sync
    Check {
        /// Path to target .env file
        #[arg(short, long, default_value = ".env")]
        env: PathBuf,

        /// Path to template .env.example file
        #[arg(short = 'x', long = "example", default_value = ".env.example")]
        example: PathBuf,
    },

    /// Scan a file for exposed API keys or leaked secrets
    Scan {
        /// Path to file to scan (e.g. .env.example or source file)
        #[arg(short, long, default_value = ".env.example")]
        path: PathBuf,
    },

    /// Automatically generate a clean .env.example from an existing .env
    Generate {
        /// Source .env file
        #[arg(short, long, default_value = ".env")]
        source: PathBuf,

        /// Destination .env.example file
        #[arg(short, long, default_value = ".env.example")]
        output: PathBuf,

        /// Overwrite output file if it already exists
        #[arg(short, long)]
        force: bool,

        /// Keep harmless default values (ports, localhost, booleans, etc.)
        #[arg(short, long, default_value_t = true)]
        keep_defaults: bool,
    },
}

fn print_banner() {
    println!(
        "{} {}",
        "🛡️  EnvPatrol".bold().cyan(),
        format!("v{}", env!("CARGO_PKG_VERSION")).dimmed()
    );
    println!(
        "{}",
        "─────────────────────────────────────────────────────".dimmed()
    );
}

fn main() {
    let cli = Cli::parse();

    print_banner();

    match cli.command {
        Some(Commands::Check { env, example }) => {
            match checker::check_sync(&env, &example) {
                Ok(result) => {
                    checker::render_check_result(&env, &example, &result);
                    if !result.is_success() {
                        process::exit(1);
                    }
                }
                Err(err) => {
                    eprintln!("{} {}", "Error:".bold().red(), err);
                    process::exit(1);
                }
            }
        }

        Some(Commands::Scan { path }) => {
            match scanner::scan_env_file(&path) {
                Ok(findings) => {
                    scanner::render_scan_result(&path, &findings);
                    if !findings.is_empty() {
                        process::exit(1);
                    }
                }
                Err(err) => {
                    eprintln!("{} {}", "Error:".bold().red(), err);
                    process::exit(1);
                }
            }
        }

        Some(Commands::Generate {
            source,
            output,
            force,
            keep_defaults,
        }) => {
            println!("{}", "✨ Generating sanitized .env.example template...".bold().cyan());
            println!("  📄 Source: {}", source.display().to_string().yellow());
            println!("  📋 Output: {}", output.display().to_string().yellow());
            println!();

            let opts = generator::GenerateOptions {
                keep_defaults,
                force,
            };

            match generator::generate_example(&source, &output, &opts) {
                Ok(count) => {
                    println!("{}", "✅ SUCCESS!".bold().green());
                    println!(
                        "  Generated {} keys into {}",
                        count.to_string().cyan().bold(),
                        output.display().to_string().yellow()
                    );
                    println!("  All sensitive credentials have been safely masked.");
                }
                Err(err) => {
                    eprintln!("{} {}", "Error:".bold().red(), err);
                    process::exit(1);
                }
            }
        }

        None => {
            // Default behavior if run without arguments: check .env vs .env.example
            let default_env = PathBuf::from(".env");
            let default_example = PathBuf::from(".env.example");

            if default_env.exists() && default_example.exists() {
                println!("{}", "No subcommand specified. Running default `check`...".dimmed());
                println!();
                match checker::check_sync(&default_env, &default_example) {
                    Ok(result) => {
                        checker::render_check_result(&default_env, &default_example, &result);
                        if !result.is_success() {
                            process::exit(1);
                        }
                    }
                    Err(err) => {
                        eprintln!("{} {}", "Error:".bold().red(), err);
                        process::exit(1);
                    }
                }
            } else {
                println!("{}", "Run `envpatrol --help` to see all available commands:".yellow());
                println!("  • {} : Compare .env with .env.example", "envpatrol check".cyan());
                println!("  • {}  : Scan for leaked secrets & API keys", "envpatrol scan".cyan());
                println!("  • {}  : Auto-generate safe .env.example", "envpatrol generate".cyan());
            }
        }
    }
}
