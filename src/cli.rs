use clap::Parser;
use std::path::PathBuf;

#[derive(Parser, Debug)]
#[command(name = "drakoon")]
#[command(about = "MoonBit to LLVM compiler", long_about = None)]
pub struct Cli {
    /// Path to the .mbt source file
    #[arg()]
    pub file: PathBuf,
}
