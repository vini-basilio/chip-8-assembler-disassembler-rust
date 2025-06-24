use std::fs::File;
use std::io::Write;
use std::process::exit;
use indicatif::{ProgressBar, ProgressStyle};
use crate::modules::assembler::instruction_parse::parse_instruction;
pub fn assembler(contents: String, output_addr: String)  {
    let mut machine_code: Vec<u8> = Vec::new();

    let bar = ProgressBar::new(contents.len() as u64);
    bar.set_style(ProgressStyle::with_template("[{bar:50}] {pos:>3}/{len:3}")
        .unwrap()
        .progress_chars("#>-"));

    let mut acc = 0;

    for line in contents.lines() {

        bar.set_position(acc);
        acc += 1;

        let tokens: Vec<&str> = line.split_whitespace().collect();
        if tokens[0].starts_with("0x") {
            let cleaned = &tokens[0].trim_start_matches("0x");
            match u16::from_str_radix(cleaned, 16) {
                Ok(n) => {
                    machine_code.push(n as u8);
                    continue;
                },
                Err(_e) => {
                    eprintln!("Erro ao converter o asset para bit");
                    exit(1);
                }
                ,
            }
        }

        match parse_instruction(tokens) {
            Ok(op) => {
                machine_code.push(op.0);
                machine_code.push(op.1);
            },
            Err(e) => {
                eprintln!("ERRO DO TIPO {}", e);
                exit(1);
            },
        }

    }
    bar.finish();

    println!("-> Saving...");

    let mut path = std::path::PathBuf::from(output_addr);
    path.set_extension("ch8");

    let mut file =  File::create(path);

    match &mut file {
        Ok(f) => {
            let _ = f.write(&machine_code);
            println!("Done!");
            exit(0);
        },
        Err(e) => {
            eprintln!("x: Error: Unable to create ROM file") ;
            eprintln!("x: Description: {:?}", e);
            exit(1);
        },
    }
}