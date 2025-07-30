mod cli;
mod lexer;
mod token;

use clap::Parser;
use cli::Cli;
use lexer::{Lexer, LexerError};
use std::fs;

fn main() {
    let args = Cli::parse();

    if args.file.extension().and_then(|s| s.to_str()) != Some("mbt") {
        eprintln!("Error: only .mbt files are supported");
        std::process::exit(1);
    }

    let source = fs::read_to_string(&args.file)
        .unwrap_or_else(|_| panic!("Failed to read file {:?}", args.file));

    println!("Lexing file: {:?}", args.file);

    let lexer = Lexer::new(&source);

    for result in lexer {
        match result {
            Ok((start_pos, token, end_pos)) => {
                println!(
                    "{:>3}:{:<3} - {:>3}:{:<3} {:?}",
                    start_pos.line, start_pos.column, end_pos.line, end_pos.column, token
                );
            }
            Err(LexerError::InvalidToken {
                start,
                invalid_token,
                end,
            }) => {
                eprintln!(
                    "Error: Invalid token {} at line {}:{} to {}:{}",
                    invalid_token, start.line, start.column, end.line, end.column
                );
            }
        }
    }
}
