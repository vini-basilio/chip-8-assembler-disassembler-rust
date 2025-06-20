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
                _ => Err("OPCODE: a instrução 'keyboard', mas o opcode é desconhecido"),
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
            let opcode = instruction_loadbyte_opcode(tokens)?;
            Ok(convert_hexa_two_nibble(opcode))
        },
        InstructionKinds::LogicalExceptions => {
            if tokens[1].starts_with("V") && tokens[2] == "{," && tokens[3].starts_with("V") {
                let regs = handle_reg(tokens[1], 8, false)?
                    | handle_reg(tokens[3], 4, false)?;

                return match tokens[0] {
                    "SHR" => Ok(convert_hexa_two_nibble(Opcode::Shr.value() | regs)),
                    "SHL" => Ok(convert_hexa_two_nibble(Opcode::Shl.value()  | regs)),
                    _ => Err("OPCODE: a instrução 'logicalExpections', mas o opcode é desconhecido"),
                }
            }
            Err("OPCODE: a instrução 'logicalExpections', mas o opcode é desconhecido")
        }
        InstructionKinds::Draw => {
            let regs = handle_reg(tokens[1], 8, true)? | 
                handle_reg(tokens[2], 4, true)?;
            
            let addr = valid_u8_address(tokens[3])?;
            Ok(convert_hexa_two_nibble(Opcode::Draw.value() | regs | addr))
        }
    }
}

