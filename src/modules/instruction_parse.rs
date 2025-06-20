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
pub fn parse_instruction(tokens: Vec<&str>) -> Result<(u8, u8), &'static str> {
    let instruction_pattern = instruction_type(&tokens)?;
    let opcode = valid_and_assemble(&tokens, instruction_pattern)?;
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



#[cfg(test)]
pub mod tests_errors {
    use crate::modules::instruction_parse::parse_instruction;

    fn error_test(str: &str) -> bool{
        let teste: Vec<&str>= str.split_whitespace().collect();
        parse_instruction(teste).is_err()
    }

    #[test]
    fn cls_error() {
        assert!(error_test("Cls"));
        assert!(error_test("Cls I"));
    }
    #[test]
    fn ret_error() {
        assert!(error_test("Ret"));
        assert!(error_test("RET vx"));
    }
    #[test]
    fn jpaddr_error() {
        assert!(error_test("jp 0x200"));
        assert!(error_test("JP 0x200 vx"));
        assert!(error_test("JP 200"));
        assert!(error_test("JP 0x1200"));
    }
    #[test]
    fn call_error() {
        assert!(error_test("call 0x200"));
        assert!(error_test("CALL vx 0x200"));
        assert!(error_test("CALL 200"));
        assert!(error_test("CALL 0x1200"));
    }

    #[test]
    fn jpv0_error() {
        assert!(error_test("JP v0, 0x200"));
        assert!(error_test("JP V0, 0x200 v"));
        assert!(error_test("JP V0, 200"));
        assert!(error_test("JP V0, 0x1200"));
    }

    #[test]
    fn ldi_error() {
        assert!(error_test("LD i, 0x200"));
        assert!(error_test("LD I, 0x200 v"));
        assert!(error_test("LD I, 200"));
        assert!(error_test("LD I, 0x1200"));
    }
    #[test]
    fn ldvv_error() {
        assert!(error_test("LD V0 V1"));
        assert!(error_test("LD V1 V2 0x200"));
        assert!(error_test("LD V1, V17"));
    }

    #[test]
    fn skp_error() {
        assert!(error_test("SKp V0"));
        assert!(error_test("SKP V1 V2 0x200"));
        assert!(error_test("SKP V17"));
    }

    #[test]
    fn se_byte_error() {
        assert!(error_test("Se V0, 0x200"));
        assert!(error_test("Se V0, 0x200 asd"));
        assert!(error_test("SKP V17, 0x001"));
        assert!(error_test("SKP V1, 0x5000"));
    }

    #[test]
    fn f_reg_label_error() {
        assert!(error_test("LD v0, DT"));
        assert!(error_test("SKP V1 DT"));
        assert!(error_test("SKP V17, DT"));
    }

    #[test]
    fn f_label_reg_error() {
        assert!(error_test("LD Dt, V9"));
        assert!(error_test("LD DT V9"));
        assert!(error_test("LD DT V9"));
    }
}


#[cfg(test)]
pub mod tests_sucess {
    use crate::modules::instruction_parse::parse_instruction;

    fn sucess(str: &str) -> Result<(u8, u8), &str>{
        let teste: Vec<&str>= str.split_whitespace().collect();
        parse_instruction(teste)
    }

    #[test]
    fn cls() { assert_eq!(sucess("CLS").unwrap(), (0x00u8, 0xE0u8)); }
    #[test]
    fn ret() { assert_eq!(sucess("RET").unwrap(), (0x00u8, 0xEEu8)); }

    #[test]
    fn jpone() { assert_eq!(sucess("JP 0x200").unwrap(), (0x12u8, 0x00u8)); }

    #[test]
    fn call() { assert_eq!(sucess("CALL 0x200").unwrap(), (0x22u8, 0x00u8)); }

    #[test]
    fn jpb() { assert_eq!(sucess("JP V0, 0x200").unwrap(), (0xB2u8, 0x00u8)); }

    #[test]
    fn ldi() { assert_eq!(sucess("LD I, 0x200").unwrap(), (0xA2u8, 0x00u8)); }
    #[test]
    fn skp() { assert_eq!(sucess("SKP V2").unwrap(), (0xE2u8, 0x9Eu8)); }

    #[test]
    fn sknp() { assert_eq!(sucess("SKNP V2").unwrap(), (0xE2u8, 0xA1u8)); }

    #[test]
    fn sebyte() { assert_eq!(sucess("SE V1, 0x01").unwrap(), (0x31u8, 0x01u8)); }
    #[test]
    fn snebyte() { assert_eq!(sucess("SNE V1, 0x01").unwrap(), (0x41u8, 0x01u8)); }
    #[test]
    fn ldbyte() { assert_eq!(sucess("LD V1, 0x01").unwrap(), (0x61u8, 0x01u8)); }
    #[test]
    fn addbyte() { assert_eq!(sucess("ADD V1, 0x01").unwrap(), (0x71u8, 0x01u8)); }

    #[test]
    fn rndbyte() { assert_eq!(sucess("RND V1, 0x01").unwrap(), (0xC1u8, 0x01u8)); }

    #[test]
    fn sevv() { assert_eq!(sucess("SE V1, V2").unwrap(), (0x51u8, 0x20u8)); }

    #[test]
    fn ldvv() { assert_eq!(sucess("LD V1, V2").unwrap(), (0x81u8, 0x20u8)); }

    #[test]
    fn or() { assert_eq!(sucess("OR V1, V2").unwrap(), (0x81u8, 0x21u8)); }

    #[test]
    fn and() { assert_eq!(sucess("AND V1, V2").unwrap(), (0x81u8, 0x22u8)); }

    #[test]
    fn xor() { assert_eq!(sucess("XOR V1, V2").unwrap(), (0x81u8, 0x23u8)); }

    #[test]
    fn addvv() { assert_eq!(sucess("ADD V1, V2").unwrap(), (0x81u8, 0x24u8)); }

    #[test]
    fn sub() { assert_eq!(sucess("SUB V1, V2").unwrap(), (0x81u8, 0x25u8)); }

    #[test]
    fn subn() { assert_eq!(sucess("SUBN V1, V2").unwrap(), (0x81u8, 0x27u8)); }

    #[test]
    fn snevv() { assert_eq!(sucess("SNE VF, V2").unwrap(), (0x9Fu8, 0x20u8)); }

    #[test]
    fn f_ld_reg_dt() { assert_eq!(sucess("LD V6, DT").unwrap(), (0xF6u8, 0x07u8)); }

    #[test]
    fn f_ld_reg_k() { assert_eq!(sucess("LD V6, K").unwrap(), (0xF6u8, 0x0Au8)); }

    #[test]
    fn f_ld_reg_i() { assert_eq!(sucess("LD VB, [I]").unwrap(), (0xFBu8, 0x65u8)); }

    #[test]
    fn f_ld_dt_vx() { assert_eq!(sucess("LD DT, VA").unwrap(), (0xFAu8, 0x15u8)); }

    #[test]
    fn f_ld_f_vx() { assert_eq!(sucess("LD F, VD").unwrap(), (0xFDu8, 0x29u8)); }

    #[test]
    fn f_add_i_vx() { assert_eq!(sucess("ADD I, VD").unwrap(), (0xFDu8, 0x1Eu8)); }

    #[test]
    fn f_ld_b_vx() { assert_eq!(sucess("LD B, VD").unwrap(), (0xFDu8, 0x33u8)); }

    #[test]
    fn f_ld_i_vx() { assert_eq!(sucess("LD [I], VD").unwrap(), (0xFDu8, 0x55u8)); }

    #[test]
    fn shr_vx_vy() { assert_eq!(sucess("SHR V0 {, V1}").unwrap(), (0x80u8, 0x16u8)); }

    #[test]
    fn shl_vx_vy() { assert_eq!(sucess("SHL V0 {, V1}").unwrap(), (0x80u8, 0x1Eu8)); }

    #[test]
    fn drw() {
        let result = sucess("DRW V0, V1, 0x2").unwrap();
        assert_eq!(result, (0xD0u8, 0x12u8));
    }
}
