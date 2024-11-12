use clap::{Parser, Subcommand};
use std::fs;
use std::path::PathBuf;

use sql_select_parser::{parse_query, ParseError};

#[derive(Parser)]
#[command(name = "sql_select_parser")]
#[command(author = "Bohdan Prokhorov <howchromium@gmail.com>")]
#[command(version = "1.0")]
#[command(about = "Parses SQL SELECT queries", long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    Parse {
        #[arg(short, long, value_name = "FILE")]
        file: PathBuf,
    },
    Credits,
}

fn main() {
    let cli = Cli::parse();

    match &cli.command {
        Commands::Parse { file } => {
            if let Err(e) = handle_parse_command(file) {
                eprintln!("Error: {}", e);
                std::process::exit(1);
            }
        }
        Commands::Credits => {
            handle_credits_command();
        }
    }
}

fn handle_parse_command(file: &PathBuf) -> Result<(), ParseError> {
    let content = fs::read_to_string(file).map_err(|e| {
        ParseError::ParsingError(format!(
            "Failed to read file {}: {}",
            file.display(),
            e
        ))
    })?;

    match parse_query(&content) {
        Ok(query) => {
            println!("Parsed Query:\n{:#?}", query);
            Ok(())
        }
        Err(e) => Err(e),
    }
}

fn handle_credits_command() {
    println!("SQL Parser CLI");
    println!("Version 1.0");
    println!("Developed by Bohdan Prokhorov");
    println!("© 2023 bohdamnnnnn");
    println!("\nThis tool parses SIMPLE SQL SELECT queries and outputs their abstract syntax tree (AST).");
}