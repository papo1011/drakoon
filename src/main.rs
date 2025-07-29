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
            Ok((start, token, end)) => {
                println!("{:?} [{}..{}]", token, start, end);
            }
            Err(LexerError::InvalidToken(start, end)) => {
                eprintln!("Errore: carattere non valido tra {} e {}", start, end);
            }
        }
    }
}
