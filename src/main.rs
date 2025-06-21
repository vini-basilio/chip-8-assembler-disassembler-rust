mod modules;

use std::path::Path;
use std::{fs};
use std::fs::File;
use std::io::{BufWriter, Write};
use std::process::exit;
use crate::modules::instruction_parse::parse_instruction;

fn main() -> std::io::Result<()>  {
    let contents = fs::read_to_string("ibm_logo.txt").expect("Should have been able to read the file");

    let mut machine_code: Vec<u8> = Vec::new();

    for line in contents.lines() {
        println!("Lendo: {}", line);
        let tokens: Vec<&str> = line.split_whitespace().collect();
        match parse_instruction(tokens) {
            Ok(op) => {
                machine_code.push(op.0);
                machine_code.push(op.1);
            },
            Err(e)=> {
                eprintln!("ERRO DO TIPO {}", e) ;
                exit(1);
            },
        }
    }

    display_binariy(&machine_code, 8);
    let path = Path::new("bar.ch8");
    let mut file = File::create(path)?;
    file.write(&machine_code);

    Ok(())

}

fn display_binariy(machine_code: &Vec<u8>, bytes_per_line: i32) {
    let mut counter = 0;
    for op in machine_code {
        if counter % bytes_per_line == 0 {
            println!("");
        }
        print!("{:#04x} ", op);
        counter += 1;
    }
}
