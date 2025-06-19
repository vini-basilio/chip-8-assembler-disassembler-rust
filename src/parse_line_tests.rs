#[cfg(test)]
pub mod tests {
    use crate::modules::instruction_parse::parse_instruction;
    #[test]
    fn jpone_error_tokens() {
        let teste = &["JP", " ", "0x200"];
        let result = parse_instruction(teste);

        assert!(result.is_err());
    }

    #[test]
    fn call_error_tokens() {
        let teste = &["CALL", " ", "0x200"];
        let result = parse_instruction(teste);

        assert!(result.is_err());
    }

    #[test]
    fn cls_error_tokens_lower_case() {
        let teste = &["ClS"];
        let result = parse_instruction(teste);

        assert!(result.is_err());
    }
    #[test]
    fn ret_error_tokens_lower_case() {
        let teste = &["rET"];
        let result = parse_instruction(teste);

        assert!(result.is_err());
    }

    #[test]
    fn ldi_error_tokens_lower_case() {
        let teste = &["LD", "I ", "0x200"];
        let result = parse_instruction(teste);

        assert!(result.is_err());
    }
    #[test]
    fn jpb_error_tokens() {
        let teste = &["JP", "V0", "VX"];
        let result = parse_instruction(teste);

        assert!(result.is_err());
    }
    #[test]
    fn cls_sucess() {
        let teste = &["CLS"];
        let result = parse_instruction(teste);

        assert_eq!(result.unwrap(), (0x00u8, 0xE0u8));
    }
    #[test]
    fn ret_sucess() {
        let teste = &["RET"];
        let result = parse_instruction(teste);

        assert_eq!(result.unwrap(), (0x00u8, 0xEEu8));
    }
    #[test]
    fn jp_sucess() {
        let teste = &["JP", "0x200"];
        let result = parse_instruction(teste);

        assert_eq!(result.unwrap(), (0x12u8, 0x00u8));
    }
    #[test]
    fn call_sucess() {
        let teste = &["CALL", "0x200"];
        let result = parse_instruction(teste);

        assert_eq!(result.unwrap(), (0x22u8, 0x00u8));
    }
    #[test]
    fn ldi_sucess() {
        let teste = &["LD", "I", "0x200"];
        let result = parse_instruction(teste);

        assert_eq!(result.unwrap(), (0xA2u8, 0x00u8));
    }

    #[test]
    fn skp_sucess() {
        let teste = &["SKP", "V2"];
        let result = parse_instruction(teste);

        assert_eq!(result.unwrap(), (0xE2u8, 0x9Eu8));
    }

    #[test]
    fn skp_reg_error() {
        let teste = &["SKP", "Vj"];
        let result = parse_instruction(teste);

        assert!(result.is_err());
    }

    #[test]
    fn sknp_sucess() {
        let teste = &["SKNP", "V2"];
        let result = parse_instruction(teste);

        assert_eq!(result.unwrap(), (0xE2u8, 0xA1u8));
    }
    #[test]
    fn se_reg_reg_sucess() {
        let teste = &["SE", "V1", "V2"];
        let result = parse_instruction(teste);

        assert_eq!(result.unwrap(), (0x51u8, 0x20u8));
    }
    #[test]
    fn ld_reg_reg_sucess() {
        let teste = &["LD", "V1", "V2"];
        let result = parse_instruction(teste);

        assert_eq!(result.unwrap(), (0x81u8, 0x20u8));
    }

    #[test]
    fn ld_reg_index_sucess() {
        let teste = &["LD", "V1", "[I]"];
        let result = parse_instruction(teste);

        assert_eq!(result.unwrap(), (0xF1u8, 0x65u8));
    }

    #[test]
    fn ld_dt_sucess() {
        let teste = &["LD", "DT,", "V1"];
        let result = parse_instruction(teste);

        assert_eq!(result.unwrap(), (0xF1u8, 0x15u8));
    }



}