use std::collections::{HashSet, VecDeque};
use std::fs::File;
use std::io::{BufReader, Read, Write};
use std::process::exit;
use std::time::Duration;
use indicatif::{ProgressBar};
use crate::modules::disassembler::utils::{next_opcode, parse};

pub fn disassembler(addr: String, output_addr: String) {
    const ROM_START_AT: usize = 0x200;

    let file  = File::open(addr).unwrap_or_else(|e| {
        eprintln!("Não foi possível abrir o arquivo");
        exit(1)
    });

    let reader = BufReader::new(file);
    let mut buffer = Vec::new();

    for byte_or_error in reader.bytes() {
        match byte_or_error {
            Ok(b)=>  buffer.push(b),
            Err(e) => {
                eprintln!("Erro ao ler o arquivo da ROM {:?}", e);
                exit(1);
            },
        }
    }

    let max_address = buffer.len() + ROM_START_AT;

    let bar = ProgressBar::new_spinner();
    bar.enable_steady_tick(Duration::from_millis(10));

    println!("Mapeando o código");
    let codemap= map_rom(ROM_START_AT, &mut buffer, max_address);

    println!("Gerando o arquivo final");

    let result = render_rom(ROM_START_AT, buffer, max_address, codemap);

    bar.finish_and_clear();
    match salve_file(output_addr, result) {
        Ok(_) => {
            println!("Terminado!");
            exit(0);
        },
        Err(e) => {
            eprintln!("Erro ao criar o arquivo para salvar {:?}", e);
            exit(1);
        },
    }
}

pub fn salve_file(output_addr: String, result: Vec<String>) -> Result<(), String> {
    let mut path = std::path::PathBuf::from(output_addr);
    path.set_extension("txt");
    let file = File::create(path);
    match file {
        Ok(mut f) => {
            for line in result {
                writeln!(&mut f, "{}", line).expect("Erro ao escrever o arquivo no arquivo final");
            }
            Ok(())
        },
        Err(s) => Err(s.to_string()) ,
    }
}

pub fn render_rom(rom_start_at: usize, mut buffer:  Vec<u8>, max_address: usize, codemap: HashSet<usize>) -> Vec<String>  {
    let mut result: Vec<String> = Vec::new();
    let mut pc = rom_start_at;
    while pc < max_address {
        if !codemap.contains(&pc) {
            result.push(format!("0x{:02X}", buffer[pc - rom_start_at]));
            pc += 1;
            continue;
        }

        if let Some(opcode) = next_opcode(pc as u16, rom_start_at as u16, &mut buffer) {
            result.push(parse(opcode));
            pc += 2;
        } else {
            break;
        }
    }
    result
}

pub fn map_rom(rom_start_at: usize, mut buffer: &mut Vec<u8>, max_address: usize) ->  HashSet<usize> {
    let mut segments: VecDeque<usize> = VecDeque::new();
    let mut codemap: HashSet<usize> = HashSet::new();
    segments.push_back(rom_start_at);

    while let Some(mut pc) = segments.pop_front() {
        while pc < max_address && !codemap.contains(&pc) {
            match next_opcode(pc as u16, rom_start_at as u16, &mut buffer) {
                None => { break }
                Some(opcode) => {
                    codemap.insert(pc);
                    codemap.insert(pc + 1);
                    pc += 2;

                    match opcode & 0xF000 {
                        0x1000 => {
                            pc = (opcode & 0x0FFF) as usize;
                        }
                        0x2000 => {
                            segments.push_front(pc);
                            pc = (opcode & 0x0FFF) as usize;
                        }
                        0x3000 | 0x4000 | 0x5000 | 0x9000 => {
                            segments.push_front(pc + 2);
                        }
                        0xE000 => {
                            match opcode & 0xF0FF {
                                0xE09E | 0xE0A1 => segments.push_front(pc + 2),
                                _ => {}
                            }
                        }
                        0xB000 => {
                            eprintln!("Instrução JP V0, addr não suportada");
                            exit(1);
                        }
                        _ => {}
                    }
                }
            }
        }
    }
    codemap
}
