use crate::opcodes;

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

pub fn parse(opcode: u16) -> String {
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
