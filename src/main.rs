mod modules;

use std::{fs};
use std::process::exit;
use crate::modules::instruction_parse::parse_instruction;

fn main() {
    let contents = fs::read_to_string("assembler_file_test.txt")
        .expect("Should have been able to read the file");

    let mut machine_code: Vec<(u8, u8)> = Vec::new();

    for line in contents.lines() {
        println!("Lendo: {}", line);
        let tokens: Vec<&str> = line.split_whitespace().collect();
        match parse_instruction(tokens) {
            Ok(op) => { machine_code.push(op) },
            Err(e)=> {
                eprintln!("ERRO DO TIPO {}", e) ;
                exit(1);
            },
        }
    }

    let mut counter = 0;
    let size_line = 3;
    for op in &machine_code  {
        if counter % size_line == 0 {
            println!("");
        }
        print!("{:#04x} ", op.0);
        print!("{:#04x} ", op.1);

        counter += 1;
    }
}
