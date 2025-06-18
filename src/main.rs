fn main() {
    let instrutions_set: Vec<String> = Vec::new();
    let result = instruction_type(&["ADD", "I", "VX"]);
    match result {
        Ok(e) => println!("{:?}", e),
        Err(_) => todo!()
    }
}

#[derive(Debug)]
enum InstructionKinds {
    Simples,
    U12Address,
    LoadByte,
    Keyboard,
    FRegLabel,
    FLabelReg,
    Logical,
}

fn instruction_type(tokens: &[&str]) -> Result<InstructionKinds, &'static str> {
    match tokens.len() {
        1 => one_token(tokens),
        2 => two_token(tokens),
        3 => three_token(tokens),
        4 => four_token(tokens),
        _ => Err("Número de tokens inválido"),
    }
}

fn one_token(tokens: &[&str])-> Result<(InstructionKinds),  &'static str>{
    match tokens[0] {
        "CLS" | "RET" => Ok(InstructionKinds::Simples),
        _ => Err("Instrução de um token inválida"),
    }
}
fn two_token(tokens: &[&str])-> Result<(InstructionKinds),  &'static str>{
    match tokens[0] {
        "JP" |  "CALL" => Ok(InstructionKinds::U12Address),
        "SKP" | "SKNP" if tokens[1].starts_with('V') => Ok(InstructionKinds::Keyboard),
        _ => Err("Instrução de dois tokens inválida"),
    }
}
fn three_token(tokens: &[&str])-> Result<(InstructionKinds),  &'static str>{
    let t1 = tokens[1];
    let t2 = tokens[2];

    match (t1.starts_with("V"), t2.starts_with("0x"), t2.starts_with("V")){
        (true, true, false) => Ok(InstructionKinds::LoadByte),
        (true, false, true) => Ok(InstructionKinds::Logical),
        (true, false, false) => Ok(InstructionKinds::FRegLabel),
        (false, false, true) => Ok(InstructionKinds::FLabelReg),
        (false, true, false) => Ok(InstructionKinds::U12Address),
        _ => Err("Instrução de três tokens inválida"),
    }
}
fn four_token(tokens: &[&str])-> Result<(InstructionKinds),  &'static str>{
    match tokens[3] {
        "SHR" | "SHL" if tokens[1].starts_with('V') => Ok(InstructionKinds::Logical),
        "DRW" if tokens[3].starts_with('V') => Ok(InstructionKinds::Keyboard),
        _ => Err("Instrução de quatro tokens inválida"),
    }
}