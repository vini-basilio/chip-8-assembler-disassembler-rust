use crate::modules::patterns::{InstructionKinds, Opcode};
/// Analisa uma instrução CHIP-8 e retorna seu opcode correspondente
///
/// # Argumentos
/// * `tokens` - Um slice de strings representando os componentes da instrução
///
/// # Retornos
/// * `Ok((u8, u8))` - Uma tupla com o opcode para ser salvo
/// * `Err(&str)` - Mensagem de erro se a instrução for inválida
pub fn parse_instruction(tokens: &[&str]) -> Result<(u8, u8), &'static str> {
    let instruction_pattern = instruction_type(tokens)?;
    let opcode = valid_and_assemble(tokens, instruction_pattern)?;
    Ok(opcode)
}
fn instruction_type(tokens: &[&str]) -> Result<InstructionKinds, &'static str> {
    match tokens.len() {
        1 =>  Ok(InstructionKinds::Simple),
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
fn four_token(tokens: &[&str])-> Result<InstructionKinds,  &'static str>{
    match tokens[3] {
        "SHR" | "SHL" if tokens[1].starts_with('V') => Ok(InstructionKinds::Logical),
        "DRW"  => Ok(InstructionKinds::Draw),
        _ => Err("Instrução de quatro tokens inválida"),
    }
}

fn convert_hexa_two_nibble(opcode: u16) -> (u8, u8){
    let n1 = ((opcode & 0xFF00) >> 8) as u8;
    let n2 = (opcode & 0x00FF) as u8;
    (n1, n2)
}

fn valid_u12_address(address: &str) -> Result<u16,  &'static str> {
    let cleaned = address.trim_start_matches("0x");
    match u16::from_str_radix(cleaned, 16) {
        Ok(n) if n <= 0x0FFF => Ok(n),
        Ok(_) => Err("Endereço de 12 bits maior que número máximo"),
        Err(_e) => Err("Erro {} ao converter o endereço de 12 bit"),
    }
}

fn valid_reg(reg: &str) -> Result<u16,  &'static str> {
    let cleaned = reg.trim_start_matches("V");
    match u16::from_str_radix(cleaned, 16) {
        Ok(n) if n <= 0x000F => Ok(n),
        Ok(_) => Err("Registrador não encontrado"),
        Err(_e) => Err("Erro ao converter o registrador"),
    }
}

/// Processes a register string, validates it, and applies a left shift operation.
///
/// # Arguments
/// * `s` - A string slice representing the register input.
/// * `shift` - An 8-bit signed integer representing the number of bits to shift the register value to the left.
///
/// # Returns
/// * `Ok(u16)` - The resulting 16-bit unsigned integer after the valid register value is shifted.
/// * `Err(&'static str)` - Returns a static string slice error if the register string is invalid.
///
/// # Errors
/// This function will return an error if:
/// * The `s` input is not a valid register string as determined by the `valid_reg` function.
///
/// # Example
/// ```
/// let result = handle_reg("V1", 4);
/// match result {
///     Ok(value) => println!("Shifted value: {}", value),
///     Err(err) => println!("Error: {}", err),
/// }
/// ```
fn handle_reg(s:&str, shift: i8) ->Result<u16,  &'static str> {
    let reg = valid_reg(s)?;
     Ok(reg << shift)
}

fn valid_and_assemble(tokens: &[&str], instruction_kind: InstructionKinds) -> Result<(u8, u8),  &'static str>{
    let error_msg = "A instrução possuí uma formatação conhecida, mas não um opcode conhecido";
    match instruction_kind {
       
        InstructionKinds::Simple => {
            match tokens[0] {
                "CLS" => Ok(convert_hexa_two_nibble(Opcode::Cls.value())),
                "RET" => Ok(convert_hexa_two_nibble(Opcode::Ret.value())),
                _ => Err(error_msg),
            }
        }
        InstructionKinds::U12Address => {
            match tokens[0] {
                "JP" => {
                    if tokens[1] == "V0," { 
                        let address = valid_u12_address(tokens[2])?;
                        return Ok(convert_hexa_two_nibble(Opcode::JpB.value() | address));
                    }
                    if tokens[1].starts_with("0x") {
                        let address = valid_u12_address(tokens[1])?;
                        return Ok(convert_hexa_two_nibble(Opcode::JpOne.value() | address));
                    }
                    Err(error_msg)
                },
                "LD" if tokens[1] == "I" => {
                    let address = valid_u12_address(tokens[2])?;
                    Ok(convert_hexa_two_nibble(Opcode::LdI.value() | address))
                },
                "CALL" if tokens[1].starts_with("0x") => {
                    let address = valid_u12_address(tokens[1])?;
                    Ok(convert_hexa_two_nibble(Opcode::Call.value() | address))
                } ,
                _ => Err(error_msg),
            }
        }
        InstructionKinds::Keyboard => {
            match tokens[0] {
                "SKP" => {
                    let reg = handle_reg(tokens[1], 8)?;
                    Ok(convert_hexa_two_nibble(Opcode::Skp.value() | reg))
                }
                "SKNP" => {
                    let reg = handle_reg(tokens[1], 8)?;
                    Ok(convert_hexa_two_nibble(Opcode::Sknp.value()  | reg))
                }
                _ => Err(error_msg),
            }
        },
        InstructionKinds::Logical => {
            match tokens[0] {
                "SE" => {
                    let regs = handle_reg(tokens[1], 8)? | handle_reg(tokens[2], 4)?;
                    Ok(convert_hexa_two_nibble(Opcode::SeRegReg.value()  | regs ))
                },
                "LD" => {
                    let regs = handle_reg(tokens[1], 8)? | handle_reg(tokens[2], 4)?;
                    Ok(convert_hexa_two_nibble(Opcode::LdRegReg.value()  | regs ))
                },
                "OR" => {
                    let regs = handle_reg(tokens[1], 8)? | handle_reg(tokens[2], 4)?;
                    Ok(convert_hexa_two_nibble(Opcode::Or.value()  | regs ))
                },
                "AND" => {
                    let regs = handle_reg(tokens[1], 8)? | handle_reg(tokens[2], 4)?;
                    Ok(convert_hexa_two_nibble(Opcode::And.value()  | regs ))
                },
                "XOR" => {
                let regs = handle_reg(tokens[1], 8)? | handle_reg(tokens[2], 4)?;
                Ok(convert_hexa_two_nibble(Opcode::Xor.value()  | regs ))
                },
                "ADD" => {
                    let regs = handle_reg(tokens[1], 8)? | handle_reg(tokens[2], 4)?;
                    Ok(convert_hexa_two_nibble(Opcode::AddRegReg.value()  | regs ))
                },
                "SUB" => {
                    let regs = handle_reg(tokens[1], 8)? | handle_reg(tokens[2], 4)?;
                    Ok(convert_hexa_two_nibble(Opcode::Sub.value()  | regs ))
                },
                "SUBN" => {
                    let regs = handle_reg(tokens[1], 8)? | handle_reg(tokens[2], 4)?;
                    Ok(convert_hexa_two_nibble(Opcode::Subn.value()  | regs ))
                },
                "SNE" => {
                let regs = handle_reg(tokens[1], 8)? | handle_reg(tokens[2], 4)?;
                Ok(convert_hexa_two_nibble(Opcode::SneReg.value()  | regs ))
                },
                "SHR" => {
                    let regs = handle_reg(tokens[1], 8)? | handle_reg(tokens[2], 4)?;
                    Ok(convert_hexa_two_nibble(Opcode::Shr.value()  | regs ))
                },
                "SHL" =>{
                    let regs = handle_reg(tokens[1], 8)? | handle_reg(tokens[2], 4)?;
                    Ok(convert_hexa_two_nibble(Opcode::Shl.value()  | regs ))
                },
                _ => Err(error_msg),
            }
        },
        InstructionKinds::FRegLabel=> {
            match tokens[2] {
                "[I]" =>{
                    let reg = handle_reg(tokens[1], 8)?;
                    Ok(convert_hexa_two_nibble(Opcode::RegFromMemo.value()  | reg ))
                },
                "DT" => {
                    let reg = handle_reg(tokens[1], 8)?;
                    Ok(convert_hexa_two_nibble(Opcode::DtReg.value()  | reg ))
                }
                "K" => {
                    let reg = handle_reg(tokens[1], 8)?;
                    Ok(convert_hexa_two_nibble(Opcode::WaitKey.value()  | reg ))
                }
                _ => Err(error_msg),
            }
        },
        InstructionKinds::FLabelReg => {
            match tokens[1] {
                "DT," => {
                    let reg = handle_reg(tokens[2], 8)?;
                    Ok(convert_hexa_two_nibble(Opcode::SetDt.value()  | reg ))
                }
                // "ST," => Ok(Opcode::SetSt),
                // "F," => Ok(Opcode::SetI),
                // "I," => Ok(Opcode::AddIReg),
                // "B," => Ok(Opcode::StoreBcd),
                // "[I]," => Ok(Opcode::StoreRegMemo),
                _ => Err(error_msg)
            }
        },
        InstructionKinds::LoadByte => {
            match tokens[0] {
                // "SE" => Ok(Opcode::Se),
                // "SNE" => Ok(Opcode::Sne),
                // "RND" => Ok(Opcode::Rnd),
                // "ADD" => Ok(Opcode::AddByte),
                // "LD" => Ok(Opcode::LdByte),
                _ => Err(error_msg)
            }
        },
        InstructionKinds::Draw => {
            // if tokens[1].starts_with("V") &&  tokens[2].starts_with("V")
            // { return Ok(Opcode::Draw); }
            Err(error_msg)
        }
    }
}

