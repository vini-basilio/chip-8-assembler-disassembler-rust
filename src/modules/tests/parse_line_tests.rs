#[cfg(test)]
pub mod tests_errors {
    use crate::modules::tests::error_dataset::TestsErrorCollection;
    use crate::modules::instruction_parse::parse_instruction;

    fn error_test(str: &str) -> bool{
        let teste: Vec<&str>= str.split_whitespace().collect();
        parse_instruction(&*teste).is_err()
    }

    #[test]
    fn cls_error() {
        assert!(error_test(TestsErrorCollection::ClsCase.value()));
        assert!(error_test(TestsErrorCollection::ClsTokens.value()));
    }
    #[test]
    fn ret_error() {
        assert!(error_test(TestsErrorCollection::RetCase.value()));
        assert!(error_test(TestsErrorCollection::RetTokens.value()));
    }
    #[test]
    fn jpaddr_error() {
        assert!(error_test(TestsErrorCollection::JpAddrCase.value()));
        assert!(error_test(TestsErrorCollection::JpAddrTokens.value()));
        assert!(error_test(TestsErrorCollection::JpAddrAddressFormat.value()));
        assert!(error_test(TestsErrorCollection::JpAddrAddressSize.value()));
    }
    #[test]
    fn call_error() {
        assert!(error_test(TestsErrorCollection::CallCase.value()));
        assert!(error_test(TestsErrorCollection::CallTokens.value()));
        assert!(error_test(TestsErrorCollection::CallAddressFormat.value()));
        assert!(error_test(TestsErrorCollection::CallAddressSize.value()));
    }
    
    #[test]
    fn jpv0_error() {
        assert!(error_test(TestsErrorCollection::JpV0Case.value()));
        assert!(error_test(TestsErrorCollection::JpV0Tokens.value()));
        assert!(error_test(TestsErrorCollection::JpV0AddressFormat.value()));
        assert!(error_test(TestsErrorCollection::JpV0AddressSize.value()));
    }
    
    #[test]
    fn ldi_error() {
        assert!(error_test(TestsErrorCollection::LdICase.value()));
        assert!(error_test(TestsErrorCollection::LdITokens.value()));
        assert!(error_test(TestsErrorCollection::LdIAddressFormat.value()));
        assert!(error_test(TestsErrorCollection::LdIAddressSize.value()));
    }
    #[test]
    fn ldvv_error() {
        assert!(error_test(TestsErrorCollection::LdVxVyCase.value()));
        assert!(error_test(TestsErrorCollection::LdVxVyTokens.value()));
        assert!(error_test(TestsErrorCollection::LdVxVyRegName.value()));
    }

    #[test]
    fn skp_error() {
        assert!(error_test(TestsErrorCollection::SkpCase.value()));
        assert!(error_test(TestsErrorCollection::SkpTokens.value()));
        assert!(error_test(TestsErrorCollection::SkpRegName.value()));
    }

    #[test]
    fn se_byte_error() {
        assert!(error_test(TestsErrorCollection::SeByteCase.value()));
        assert!(error_test(TestsErrorCollection::SeByteTokens.value()));
        assert!(error_test(TestsErrorCollection::SeByteRegName.value()));
        assert!(error_test(TestsErrorCollection::SeByteAddressSize.value()));
    }

    #[test]
    fn f_reg_label_error() {
        assert!(error_test(TestsErrorCollection::FRegLabelCase.value()));
        assert!(error_test(TestsErrorCollection::FRegLabelTokens.value()));
        assert!(error_test(TestsErrorCollection::FRegLabelRegName.value()));
    }

    #[test]
    fn f_label_reg_error() {
        assert!(error_test(TestsErrorCollection::FLabelRegCase.value()));
        assert!(error_test(TestsErrorCollection::FLabelRegTokens.value()));
        assert!(error_test(TestsErrorCollection::FLabelRegRegName.value()));
    }
}


#[cfg(test)]
pub mod tests_sucess {
    use crate::modules::instruction_parse::parse_instruction;

    fn sucess(str: &str) -> Result<(u8, u8), &str>{
        let teste: Vec<&str>= str.split_whitespace().collect();
        parse_instruction(&*teste)
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