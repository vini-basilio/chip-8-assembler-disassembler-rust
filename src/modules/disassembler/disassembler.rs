use std::collections::VecDeque;
use std::fs::File;
use std::io::{BufReader, Read, Write};
use std::process::exit;
use indicatif::{ProgressBar, ProgressStyle};
use crate::{opcodes};

pub fn disassembler(contents: String, output_addr: String) {
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

    let bar = ProgressBar::new((buffer.len() - 2) as u64);
    bar.set_style(ProgressStyle::with_template("[{bar:50}] {pos:>3}/{len:3}")
        .unwrap()
        .progress_chars("#>-"));
    let mut acc = 0;


    let  rom_start_at = 0x200;
    let mut index = 0usize;
    let mut result: Vec<String> = Vec::new();
    let mut address_data: VecDeque<usize> = VecDeque::new();

    while index < buffer.len() - 2 {
        bar.inc(acc);
        acc += 2;

        if address_data.len() > 1 {
            let start_address = address_data[0].saturating_sub(rom_start_at);
            if index == start_address {
                let final_address = address_data[1].saturating_sub(rom_start_at);
                while index < final_address  {
                    result.push(format!("0x{:02X}", buffer[index]));
                    index += 1;
                }
                address_data.pop_front();
                address_data.pop_front();
                continue;
            }
        }
        let opcode = extract_opcode(&mut buffer, &mut index);
        result.push(format!("{}", parse(opcode, &mut address_data)));
    }


    bar.finish();


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

fn extract_opcode(buffer: &mut Vec<u8>, index: &mut usize) -> u16 {
    let high = buffer[*index] as u16;
    *index += 1;
    let low = buffer[*index] as u16;
    *index += 1;

    (high << 8) | low
}

fn parse(opcode: u16, address: &mut VecDeque<usize>) -> String {
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
        opcodes!(LD_I) => {
            address.push_back((opcode & 0x0FFF) as usize);
            format!("LD I, 0x{:03X}", (opcode & 0xFFF))
        },
        opcodes!(JP_B) => format!("JP V0, 0x{:03X}",  opcode & 0x0FFF),
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
        0xD000 => {
            if let Some(&start_address) = address.get(address.len() - 1) {
                let size = (opcode & 0x000F) as usize;
                let final_addr = start_address + size ;
                address.push_back(final_addr);
            }
            format!("DRW V{:01X}, V{:01X}, 0x{:01X}", (opcode & 0x0F00) >> 8, (opcode & 0x00F0) >> 4, opcode & 0x000F)
        },
            _ => String::from("Error"),
    }
}
