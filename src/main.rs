mod cli;

use clap::Parser;
use cli::Cli;
use drakoon::{cfg::build_program_cfg, codegen::CodeGen, grammar::ScriptParser, lexer::Lexer};
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

    if args.cfg {
        let prog_cfg = build_program_cfg(&ast);

        if let Some(main_cfg) = prog_cfg.main.as_ref() {
            println!("{}", main_cfg.to_dot("main"));
        }

        for (name, fun_cfg) in &prog_cfg.functions {
            println!("{}", fun_cfg.to_dot(name));
        }

        return;
    }

    let mut codegen = CodeGen::new();
    for stmt in &ast {
        codegen.append_stmt(stmt);
    }

    if !codegen.functions.contains_key("main") {
        codegen.error("'main' function is not defined.");
    }

    if codegen.sem_errors > 0 {
        eprintln!("{}", codegen.errors);
        std::process::exit(1);
    }

    println!("{}", codegen.output);
}
