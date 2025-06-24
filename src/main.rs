use std::fs;
use std::process::exit;
use clap::{Parser, Subcommand};
use crate::modules::assembler::assembler::assembler;
use crate::modules::disassembler::disassembler::disassembler;

mod modules;

#[derive(Parser)]
#[command(
    version,
    about = "A command-line interface for assembling and disassembling CHIP-8 ROMs",
    author = "Vinicius B."
)]

struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Converts an assembly file to CHIP-8 ROM
    Assembler {
        /// Path to the assembly .txt file
        #[arg(short, long)]
        input: String,
        /// Output path for the .ch8 file
        #[arg(short, long)]
        output: Option<String>,
    },
    /// Converts a CHIP-8 ROM to readable assembly
    Disassembler {
        /// Path to the .ch8 ROM
        #[arg(short, long)]
        input: String,
        /// Output path for the .txt file
        #[arg(short, long)]
        output: Option<String>
    },
}

fn main() {


    let cli = Cli::parse();
    match cli.command {
        Commands::Assembler { input, output } => {
            let output: String = match output {
                None => String::from("rom"),
                Some(path) => path,
            };

            println!(" === Starting Assembler mod === ");
            let contents = fs::read_to_string(input).unwrap_or_else(|e| {
                eprintln!("x: Error: Unable to open assembler file!");
                exit(1)
            });

            assembler(contents, output);
        }
        Commands::Disassembler { input, output } => {
            let output: String = match output {
                None => String::from("assembler"),
                Some(path) => path,
            };

            println!(" === Starting Disassembler mod === ");
            disassembler(input, output);
        }
    }
}