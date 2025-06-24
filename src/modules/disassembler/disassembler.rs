use std::collections::{HashSet, VecDeque};
use std::fs::File;
use std::io::{BufReader, Read, Write};
use std::process::exit;
use indicatif::{ProgressBar, ProgressStyle};
use crate::{opcodes};

pub fn disassembler(contents: String, output_addr: String, rom_start_at: usize) {
    let reader = BufReader::new(File::open(contents).unwrap());
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

    let max_address = buffer.len() + rom_start_at;
    let mut result: Vec<String> = Vec::new();
    // fila pra guardar segmentos de código (endereços)
    let mut segments: VecDeque<usize> = VecDeque::new();
    // endereços-alvo (labels)
    let mut labels: HashSet<usize> = HashSet::new();
    // mapa de código (endereços já processados)
    let mut codemap: HashSet<usize> = HashSet::new();

    segments.push_back(rom_start_at);
    
    while let Some(mut pc) = segments.pop_front() {
        while pc < max_address && !codemap.contains(&pc) {

           let opcode = next_opcode(pc as u16, rom_start_at as u16, &mut buffer).unwrap();
            codemap.insert(pc); // marca o endereço correto do opcode
            codemap.insert(pc + 1);
            pc += 2;

            match opcode & 0xF000 {
                0x1000 => {
                    pc = (opcode & 0x0FFF) as usize;
                    // labels.insert(pc);
                }
                0x2000 => {
                    segments.push_front(pc); // salva onde estava
                    pc = (opcode & 0x0FFF) as usize;
                    // labels.insert(pc);
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
                // 0xA000 => {
                //     labels.insert((opcode & 0x0FFF) as usize);
                // }
                0xB000 => {
                    eprintln!("Instrução JP V0, addr não suportada");
                    exit(1);
                }
                _ => {}
            }
        }
    }

    let mut pc = rom_start_at;
    while pc < max_address {
        // if labels.contains(&pc) {
        //     result.push(format!("L{:03X}:", pc));
        // }

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


    let mut path = std::path::PathBuf::from(output_addr);
    path.set_extension("txt");

    let mut file =  File::create(path);
    println!("Salvando...");
    match &mut file {
        Ok(f) => {
            for line in result {
                writeln!(f, "{}", line).expect("Erro ao escrever o arquivo no arquivo final");
            }
            println!("Terminado!");
            exit(0);
        },
        Err(e) => {
            eprintln!("Erro ao criar o arquivo para salvar {:?}", e);
            exit(1);
        },
    }
}
pub fn next_opcode(pc: u16, start_rom: u16, buffer: &mut Vec<u8>) -> Option<u16> {
    let offset = (pc - start_rom) as usize;
    if offset + 1 < buffer.len() {
        let high = buffer[offset] as u16;
        let low = buffer[offset + 1] as u16;
        Some((high << 8) | low)
    } else {
        None
    }
}

fn parse(opcode: u16) -> String {
    match opcode & 0xF000 {
        // Simples
        0x0000 => match &opcode {
                opcodes!(CLS) => String::from("CLS"),
                opcodes!(RET) => String::from("RET"),
                _ => String::from("Error"),
        }
        // U12Address
        opcodes!(JP_ONE) => format!("JP 0x{:03X}",    opcode & 0xFFF),
        opcodes!(CALL) => format!("CALL 0x{:03X}",    opcode & 0xFFF),
        opcodes!(LD_I) => format!("LD I, 0x{:03X}", (opcode & 0xFFF)),
        // LoadByte
        opcodes!(SE) => format!("SE V{:01X}, 0x{:02X}",          (opcode & 0x0F00) >> 8, opcode & 0x00FF),
        opcodes!(SNE) => format!("SNE V{:01X}, 0x{:02X}",        (opcode & 0x0F00) >> 8, opcode & 0x00FF),
        opcodes!(LD_BYTE) => format!("LD V{:01X}, 0x{:02X}", (opcode & 0x0F00) >> 8, opcode & 0x00FF),
        opcodes!(ADD_BYTE) => format!("ADD V{:01X}, 0x{:02X}",   (opcode & 0x0F00) >> 8, opcode & 0x00FF),
        opcodes!(RND) => format!("RND V{:01X}, 0x{:02X}",        (opcode & 0x0F00) >> 8, opcode & 0x00FF),
        // Keyboard
        0xE000 => {
            match opcode & 0xF0FF {
                opcodes!(SKP) =>  format!("SKP V{:01X}",  (opcode & 0x0F00) >> 8),
                opcodes!(SKNP) => format!("SKNP V{:01X}", (opcode & 0x0F00) >> 8),
                _ => String::from("Error"),
            }
        },
        // LoadByte
        0x5000 => format!("SE V{:01X}, V{:01X}", (opcode & 0x0F00) >> 8, (opcode & 0x00F0) >> 4),
        0x8000 => {
            match opcode & 0xF00F {
                opcodes!(LD_REG_REG) =>   format!("LD V{:01X}, V{:01X}",   (opcode & 0x0F00) >> 8, (opcode & 0x00F0) >> 4),
                opcodes!(OR) =>           format!("OR V{:01X}, V{:01X}",   (opcode & 0x0F00) >> 8, (opcode & 0x00F0) >> 4),
                opcodes!(AND) =>          format!("AND V{:01X}, V{:01X}",  (opcode & 0x0F00) >> 8, (opcode & 0x00F0) >> 4),
                opcodes!(XOR) =>          format!("XOR V{:01X}, V{:01X}",  (opcode & 0x0F00) >> 8, (opcode & 0x00F0) >> 4),
                opcodes!(ADD_REG_REG) =>  format!("ADD V{:01X}, V{:01X}",  (opcode & 0x0F00) >> 8, (opcode & 0x00F0) >> 4),
                opcodes!(SUB) =>          format!("SUB V{:01X}, V{:01X}",  (opcode & 0x0F00) >> 8, (opcode & 0x00F0) >> 4),
                opcodes!(SHR) =>          format!("SHR V{:01X}, V{:01X}",  (opcode & 0x0F00) >> 8, (opcode & 0x00F0) >> 4),
                opcodes!(SUBN) =>         format!("SUBN V{:01X}, V{:01X}", (opcode & 0x0F00) >> 8, (opcode & 0x00F0) >> 4),
                opcodes!(SHL) =>          format!("SHL V{:01X}, V{:01X}",  (opcode & 0x0F00) >> 8, (opcode & 0x00F0) >> 4),
                _ => String::from("Error"),
            }
        },
        opcodes!(SNE_REG) => format!("SE V{:01X}, V{:01X}",  (opcode & 0x0F00) >> 8, (opcode & 0x00F0) >> 4),
        0xF000 => {
            match opcode & 0xF0FF {
                // FRegLabel
                opcodes!(REG_FROM_MEMO) => format!("LD V{:01X}, [I]",  (opcode & 0x0F00) >> 8),
                opcodes!(DT_REG) => format!("LD V{:01X}, DT",          (opcode & 0x0F00) >> 8),
                opcodes!(WAIT_KEY) => format!("LD V{:01X}, K",         (opcode & 0x0F00) >> 8),
                // FLabelReg
                opcodes!(SET_DT) => format!("LD DT, V{:01X}",          (opcode & 0x0F00) >> 8),
                opcodes!(SET_ST) => format!("LD  ST, V{:01X}",         (opcode & 0x0F00) >> 8),
                opcodes!(ADD_I_REG) => format!("ADD I, V{:01X}",       (opcode & 0x0F00) >> 8),
                opcodes!(SET_SPRITE) => format!("LD F, V{:01X}",       (opcode & 0x0F00) >> 8),
                opcodes!(STORE_BCD) => format!("LD B, V{:01X}",        (opcode & 0x0F00) >> 8),
                opcodes!(STORE_REG_MEMO) => format!("LD [I], V{:01X}", (opcode & 0x0F00) >> 8),
                _ => String::from("Error"),
            }
        }
        0xD000 => format!("DRW V{:01X}, V{:01X}, 0x{:01X}", (opcode & 0x0F00) >> 8, (opcode & 0x00F0) >> 4, opcode & 0x000F),
            _ => String::from("Error"),
    }
}
