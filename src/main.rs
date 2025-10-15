mod cli;

use clap::Parser;
use cli::Cli;
use drakoon::{codegen::CodeGenContext, grammar::ScriptParser, lexer::Lexer};
use std::fs;

fn main() {
    let args = Cli::parse();

    if args.file.extension().and_then(|s| s.to_str()) != Some("mbt") {
        eprintln!("Error: only .mbt files are supported");
        std::process::exit(1);
    }

    let source = fs::read_to_string(&args.file)
        .unwrap_or_else(|_| panic!("Failed to read file {:?}", args.file));

    let lexer = Lexer::new(&source);
    let parser = ScriptParser::new();
    let ast = parser.parse(lexer).unwrap();

    let mut codegen = CodeGenContext::new();
    codegen.start_main();
    for stmt in &ast {
        codegen.append_stmt(stmt);
    }
    codegen.end_main();

    if codegen.sem_errors > 0 {
        eprintln!("{}", codegen.errors);
        std::process::exit(1);
    }

    println!("{}", codegen.output);
}
