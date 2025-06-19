use crate::modules::patterns::{InstructionKinds, Opcode};

pub fn two_token(tokens: &[&str]) -> Result<InstructionKinds,  &'static str>{
    match tokens[0] {
        "JP" |  "CALL" => Ok(InstructionKinds::U12Address),
        _  if tokens[1].starts_with('V') => Ok(InstructionKinds::Keyboard),
        _ => Err("Instrução de dois tokens inválida"),
    }
}
pub fn three_token(tokens: &[&str])-> Result<InstructionKinds,  &'static str>{
    let allowed_first_tokens = [
        "ADD", "JP", "SE", "SNE", "RND", "LD",
        "OR", "AND", "XOR", "SUB", "SUBN", "SNE",
        "SHR", "SHL"];

    if ! allowed_first_tokens.contains(&tokens[0]) {
        return  Err("Instrução de três tokens inválida");
    }
    let t1 = tokens[1];
    let t2 = tokens[2];

    // BNNN é o único edge case que não funciona nesse sistema
    if tokens[0] == "JP" && t1.starts_with("V") && t2.starts_with("0x") {
        return Ok(InstructionKinds::U12Address)
    }

    match (t1.starts_with("V"), t2.starts_with("0x"), t2.starts_with("V")){
        (true, false, false) => Ok(InstructionKinds::FRegLabel),
        (true, true, false) => Ok(InstructionKinds::LoadByte),
        (true, false, true) => Ok(InstructionKinds::Logical),
        (false, false, true) => Ok(InstructionKinds::FLabelReg),
        (false, true, false) => Ok(InstructionKinds::U12Address),
        _ => Err("Instrução de três tokens inválida"),
    }
}
pub fn four_token(tokens: &[&str])-> Result<InstructionKinds,  &'static str>{
    match tokens[3] {
        "SHR" | "SHL" if tokens[1].starts_with('V') => Ok(InstructionKinds::LogicalExceptions),
        "DRW"  => Ok(InstructionKinds::Draw),
        _ => Err("Instrução de quatro tokens inválida"),
    }
}

pub fn convert_hexa_two_nibble(opcode: u16) -> (u8, u8){
    let n1 = ((opcode & 0xFF00) >> 8) as u8;
    let n2 = (opcode & 0x00FF) as u8;
    (n1, n2)
}

pub fn valid_u12_address(address: &str) -> Result<u16,  &'static str> {
    let cleaned = address.trim_start_matches("0x");
    match u16::from_str_radix(cleaned, 16) {
        Ok(n) if n <= 0x0FFF => Ok(n),
        Ok(_) => Err("Endereço de 12 bits maior que número máximo"),
        Err(_e) => Err("Erro ao converter o endereço de 12 bit"),
    }
}

pub fn valid_u8_address(address: &str) -> Result<u16,  &'static str> {
    let cleaned = address.trim_start_matches("0x");
    match u16::from_str_radix(cleaned, 16) {
        Ok(n) if n <= 0x00FF => Ok(n),
        Ok(_) => Err("Endereço de 8 bits maior que número máximo"),
        Err(_e) => Err("Erro ao converter o endereço de 8 bit"),
    }
}

pub fn valid_reg(reg: &str) -> Result<u16,  &'static str> {
    let cleaned = reg.replace(&[',', 'V'], &"");
    match u16::from_str_radix(&cleaned, 16) {
        Ok(n) if n <= 0x000F => Ok(n),
        Ok(_) => Err("Registrador não encontrado"),
        Err(_e) => Err("Erro ao converter o registrador"),
    }
}

pub fn handle_reg(s:&str, shift: i8, should_has_comma: bool) ->Result<u16,  &'static str> {
    if should_has_comma && !s.ends_with(','){
    return Err("Esperava encontrar uma vírgula")
    }
    let reg = valid_reg(s)?;
    Ok(reg << shift)
}

pub fn instruction_simple_opcode(name :&str)  ->Result<u16, &'static str> {
    match name {
        "CLS" => Ok(Opcode::Cls.value()),
        "RET" => Ok(Opcode::Ret.value()),
        _ => Err("OPCODE: a instrução 'simple', mas o opcode é desconhecido"),
    }
}

pub fn instruction_u12addr_opcode(tokens: &[&str])  ->Result<u16, &'static str> {
    match tokens[0] {
        "JP" => {
            if tokens[1] == "V0," {
                let addr_token_two = valid_u12_address(tokens[2])?;
                return Ok(Opcode::JpB.value() | addr_token_two);
            }
            if tokens[1].starts_with("0x") {
                let addr_token_one = valid_u12_address(tokens[1])?;
                return Ok(Opcode::JpOne.value() | addr_token_one);
            }
            Err("OPCODE: a instrução 'u12addr', mas o opcode é desconhecido")
        },
        "LD" if tokens[1] == "I," => {
            let addr_token_two = valid_u12_address(tokens[2])?;
            Ok(Opcode::LdI.value() | addr_token_two)
        },
        "CALL" if tokens[1].starts_with("0x") => {
            let addr_token_one = valid_u12_address(tokens[1])?;
            Ok(Opcode::Call.value() | addr_token_one)
        },
        _ => Err("OPCODE: a instrução 'u12addr', mas o opcode é desconhecido"),
    }
}

pub fn instruction_logical_opcode(tokens: &[&str])  ->Result<u16, &'static str> {
    let regs = handle_reg(tokens[1], 8, true)? | handle_reg(tokens[2], 4, false)?;
    match tokens[0] {
        "SE" => Ok(Opcode::SeRegReg.value()  | regs ),
        "LD" => Ok(Opcode::LdRegReg.value()  | regs ),
        "OR" => Ok(Opcode::Or.value()  | regs ),
        "AND" => Ok(Opcode::And.value()  | regs ),
        "XOR" => Ok(Opcode::Xor.value()  | regs ),
        "ADD" => Ok(Opcode::AddRegReg.value()  | regs ),
        "SUB" => Ok(Opcode::Sub.value()  | regs ),
        "SUBN" => Ok(Opcode::Subn.value()  | regs ),
        "SNE" => Ok(Opcode::SneReg.value()  | regs ),
        _ =>  Err("OPCODE: a instrução 'logical', mas o opcode é desconhecido"),
    }
}

pub fn instruction_freglabel_opcode(tokens: &[&str])  ->Result<u16, &'static str> {
    let reg = handle_reg(tokens[2], 8, false)?;
    match tokens[1] {
        "DT," =>  Ok(Opcode::SetDt.value()  | reg ),
        "ST," => Ok(Opcode::SetSt.value()  | reg ),
        "F," => Ok(Opcode::SetI.value()  | reg ),
        "I," => Ok(Opcode::AddIReg.value()  | reg ),
        "B," => Ok(Opcode::StoreBcd.value()  | reg ),
        "[I]," => Ok(Opcode::StoreRegMemo.value()  | reg ),
        _ =>  Err("OPCODE: a instrução 'freglabel', mas o opcode é desconhecido"),
    }
}

pub fn instruction_flabelreg_opcode(tokens: &[&str])  ->Result<u16, &'static str> {
    let reg = handle_reg(tokens[1], 8, true)?;
    match tokens[2] {
        "[I]" => Ok(Opcode::RegFromMemo.value()  | reg ),
        "DT" => Ok(Opcode::DtReg.value()  | reg ),
        "K" => Ok(Opcode::WaitKey.value()  | reg ),
        _ =>  Err("OPCODE: a instrução 'flabelreg', mas o opcode é desconhecido"),
    }
}

pub fn instruction_loadbyte_opcode(tokens: &[&str])  ->Result<u16, &'static str> {
    let addr = valid_u8_address(tokens[2])?;
    let reg = handle_reg(tokens[1], 8, true)?;
    match tokens[0] {
        "SE" => Ok(Opcode::Se.value()  | reg | addr),
        "SNE" => Ok(Opcode::Sne.value()  | reg | addr),
        "RND" => Ok(Opcode::Rnd.value()  | reg | addr),
        "ADD" => Ok(Opcode::AddByte.value()  | reg | addr),
        "LD" => Ok(Opcode::LdByte.value()  | reg | addr),
        _ =>Err("OPCODE: a instrução 'loadbyte', mas o opcode é desconhecido"),
    }
}