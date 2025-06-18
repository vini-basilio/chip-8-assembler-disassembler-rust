use crate::patterns::{InstructionKinds, Opcode};
/// Analisa uma instrução CHIP-8 e retorna seu opcode correspondente
///
/// # Argumentos
/// * `tokens` - Um slice de strings representando os componentes da instrução
///
/// # Retornos
/// * `Ok(Opcode)` - O opcode correspondente à instrução
/// * `Err(&str)` - Mensagem de erro se a instrução for inválida
pub fn parse_instruction(tokens: &[&str]) -> Result<Opcode, &'static str> {
    let instruction_pattern = instruction_type(tokens)?;
    let opcode = valid_and_opcode(tokens, instruction_pattern)?;
    Ok(opcode)
}
fn instruction_type(tokens: &[&str]) -> Result<InstructionKinds, &'static str> {
    match tokens.len() {
        1 =>  Ok(InstructionKinds::Simples),
        2 => two_token(tokens),
        3 => three_token(tokens),
        4 => four_token(tokens),
        _ => Err("Número de tokens inválido"),
    }
}

fn two_token(tokens: &[&str])-> Result<InstructionKinds,  &'static str>{
    match tokens[0] {
        "JP" |  "CALL" => Ok(InstructionKinds::U12Address),
        _  if tokens[1].starts_with('V') => Ok(InstructionKinds::Keyboard),
        _ => Err("Instrução de dois tokens inválida"),
    }
}
fn three_token(tokens: &[&str])-> Result<InstructionKinds,  &'static str>{
    let t1 = tokens[1];
    let t2 = tokens[2];

    // BNNN é o único edge case que não funciona nesse sistema
    if tokens[0] == "JP" && t1.starts_with("V") && t2.starts_with("0x") {
        return Ok(InstructionKinds::U12Address)
    }

    match (t1.starts_with("V"), t2.starts_with("0x"), t2.starts_with("V")){
        (true, true, false) => Ok(InstructionKinds::LoadByte),
        (true, false, true) => Ok(InstructionKinds::Logical),
        (true, false, false) => Ok(InstructionKinds::FRegLabel),
        (false, false, true) => Ok(InstructionKinds::FLabelReg),
        (false, true, false) => Ok(InstructionKinds::U12Address),
        _ => Err("Instrução de três tokens inválida"),
    }
}
fn four_token(tokens: &[&str])-> Result<InstructionKinds,  &'static str>{
    match tokens[3] {
        "SHR" | "SHL" if tokens[1].starts_with('V') => Ok(InstructionKinds::Logical),
        "DRW"  => Ok(InstructionKinds::Draw),
        _ => Err("Instrução de quatro tokens inválida"),
    }
}

fn valid_and_opcode(tokens: &[&str], instruction_kinds: InstructionKinds) -> Result<Opcode,  &'static str>{
    let error_msg = "A instrução possuí uma formatação conhecida, mas não um opcode conhecido";
    match instruction_kinds {
        InstructionKinds::Draw => {
            if tokens[1].starts_with("V") &&  tokens[2].starts_with("V")
            { return Ok(Opcode::Draw); }
            Err(error_msg)
        }
        InstructionKinds::Simples => {
            match tokens[0] {
                "CLS" => Ok(Opcode::Cls),
                "RET" => Ok(Opcode::Ret),
                _ => Err(error_msg),
            }
        }
        InstructionKinds::U12Address => {
            match tokens[0] {
                "JP" => {
                    if tokens[1] == "V0," { return Ok(Opcode::JpB) }
                    if tokens[1].starts_with("0x") { return Ok(Opcode::JpOne) }
                    Err(error_msg)
                },
                "LD" if tokens[1] == "I" => Ok(Opcode::JpOne),
                "CALL" if tokens[1].starts_with("0x") => Ok(Opcode::Call) ,
                _ => Err(error_msg),
            }
        }
        InstructionKinds::Keyboard => {
            match tokens[0] {
                "SKP" => Ok(Opcode::Skp),
                "SKNP" => Ok(Opcode::Sknp),
                _ => Err(error_msg),
            }
        },
        InstructionKinds::Logical => {
            match tokens[0] {
                "SE" => Ok(Opcode::SeRegReg),
                "LD" => Ok(Opcode::LdRegReg),
                "OR" => Ok(Opcode::Or),
                "AND" => Ok(Opcode::And),
                "XOR" => Ok(Opcode::Xor),
                "ADD" => Ok(Opcode::AddRegReg),
                "SUB" => Ok(Opcode::Sub),
                "SUBN" => Ok(Opcode::Subn),
                "SNE" => Ok(Opcode::SneReg),
                "SHR" => Ok(Opcode::Shr),
                "SHL" => Ok(Opcode::Shl),
                _ => Err(error_msg),
            }
        },
        InstructionKinds::FRegLabel=> {
            match tokens[2] {
                "[I]" => Ok(Opcode::RegFromMemo),
                "DT" => Ok(Opcode::DtReg),
                "K" => Ok(Opcode::WaitKey),
                _ => Err(error_msg),
            }
        },
        InstructionKinds::FLabelReg => {
            match tokens[1] {
                "DT" => Ok(Opcode::SetDt),
                "ST" => Ok(Opcode::SetSt),
                "F" => Ok(Opcode::SetI),
                "I" => Ok(Opcode::AddIReg),
                "B" => Ok(Opcode::StoreBcd),
                "[I]" => Ok(Opcode::StoreRegMemo),
                _ => Err(error_msg)
            }
        },
        InstructionKinds::LoadByte => {
            match tokens[0] {
                "SE" => Ok(Opcode::Se),
                "SNE" => Ok(Opcode::Sne),
                "RND" => Ok(Opcode::Rnd),
                "ADD" => Ok(Opcode::AddByte),
                "LD" => Ok(Opcode::LdByte),
                _ => Err(error_msg)
            }
        },
    }
}