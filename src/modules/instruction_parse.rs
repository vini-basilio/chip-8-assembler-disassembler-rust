use crate::modules::patterns::{InstructionKinds, Opcode};
use crate::modules::utils::*;

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



fn valid_and_assemble(tokens: &[&str], instruction_kind: InstructionKinds) -> Result<(u8, u8),  &'static str>{
    let error_msg = "A instrução possuí uma formatação conhecida, mas não um opcode conhecido";
    match instruction_kind {
        InstructionKinds::Simple => {
            let opcode = instruction_simple_opcode(&tokens[0])?;
            Ok(convert_hexa_two_nibble(opcode))
        },
        InstructionKinds::U12Address => {
            let opcode = instruction_u12addr_opcode(tokens)?;
            Ok(convert_hexa_two_nibble(opcode))
        }
        InstructionKinds::Keyboard => {
            let reg = handle_reg(tokens[1], 8, false)?;
            match tokens[0] {
                "SKP" => Ok(convert_hexa_two_nibble(Opcode::Skp.value() | reg)),
                "SKNP" => Ok(convert_hexa_two_nibble(Opcode::Sknp.value()  | reg)),
                _ => Err(error_msg),
            }
        },
        InstructionKinds::Logical => {
            let opcode = instruction_logical_opcode(tokens)?;
            Ok(convert_hexa_two_nibble(opcode))
        },
        InstructionKinds::FRegLabel=> {
            let opcode = instruction_flabelreg_opcode(tokens)?;
            Ok(convert_hexa_two_nibble(opcode))
        },
        InstructionKinds::FLabelReg => {
            let opcode = instruction_freglabel_opcode(tokens)?;
            Ok(convert_hexa_two_nibble(opcode))
        },
        InstructionKinds::LoadByte => {
            match tokens[0] {
                "SE" => {
                    let addr = valid_u8_address(tokens[0]);
                    let reg = handle_reg(tokens[2], 8, true)?;
                    Ok(convert_hexa_two_nibble(Opcode::Se.value()  | reg ))
                }
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

