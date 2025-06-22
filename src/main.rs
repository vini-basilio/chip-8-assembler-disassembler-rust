use std::fs;
use clap::{Parser, Subcommand};
use crate::modules::assembler::assembler::assembler;

mod modules;

#[derive(Parser)]
#[command(
    version,
    about = "CLI para assembler e disassembler de ROMs CHIP-8",
    author = "vinicius basilio"
)]

struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Converte um arquivo de assembly para ROM CHIP-8
    Assembler {
        /// Caminho para o arquivo .txt de assembly
        #[arg(short, long)]
        input: String,
        /// Caminho de saída para o arquivo .ch8
        #[arg(short, long)]
        output: Option<String>,
    },
    /// Converte uma ROM CHIP-8 para assembly legível
    Disassembler {
        /// Caminho para a ROM .ch8
        #[arg(short, long)]
        input: String,
        /// Caminho de saída para o arquivo .txt
        #[arg(short, long)]
        output: Option<String>,
    },
}

fn main() {

    let cli = Cli::parse();
    match cli.command {
        Commands::Assembler { input, output } => {
            let output: String = match output {
                None => String::from("saida"),
                Some(path) => path,
            };

            println!("Montando: {:?} -> {:?}", input, output);
            let contents = fs::read_to_string(input).expect("Não foi possível abrir o arquivo");
            assembler(contents, output);
        }
        Commands::Disassembler { input, output } => {
            println!("Dissassemblando: {:?} -> {:?}", input, output);

        }
    }
}
