use std::fs::File;
use std::io::Write;
use std::process::exit;
use crate::modules::assembler::instruction_parse::parse_instruction;

pub fn assembler(contents: String, output_addr: String)  {
    let mut machine_code: Vec<u8> = Vec::new();

    for line in contents.lines() {
        println!("Lendo: {}", line);
        let tokens: Vec<&str> = line.split_whitespace().collect();
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

    display_binariy(&machine_code, 8);
    let mut path = std::path::PathBuf::from(output_addr);
    path.set_extension("ch8");

    let mut file =  File::create(path);

    match &mut file {
        Ok(f) => {
            f.write(&machine_code);
            exit(0);
        },
        Err(e) => {
            eprintln!("Erro ao criar o arquivo {:?}", e);
            exit(1);
        },
    }

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
