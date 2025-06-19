#[cfg(test)]
pub mod tests {
    use crate::modules::tests::error_dataset::TestsErrorCollection;
    use crate::modules::instruction_parse::parse_instruction;
    #[test]
    fn error_cls_case() {
        let instruction = TestsErrorCollection::ClsCase.value();
        let teste: Vec<&str>= instruction.split_whitespace().collect();
        let result = parse_instruction(&*teste);
        assert!(result.is_err());
    }
    #[test]
    fn error_cls_token() {
        let instruction = TestsErrorCollection::ClsTokens.value();
        let teste: Vec<&str>= instruction.split_whitespace().collect();
        let result = parse_instruction(&*teste);
        assert!(result.is_err());
    }
    #[test]
    fn error_ret_case() {
        let instruction = TestsErrorCollection::RetCase.value();
        let teste: Vec<&str>= instruction.split_whitespace().collect();
        let result = parse_instruction(&*teste);
        assert!(result.is_err());
    }
    #[test]
    fn error_ret_token() {
        let instruction = TestsErrorCollection::RetTokens.value();
        let teste: Vec<&str>= instruction.split_whitespace().collect();
        let result = parse_instruction(&*teste);
        assert!(result.is_err());
    }
    #[test]
    fn error_jpaddr_case() {
        let instruction = TestsErrorCollection::JpAddrCase.value();
        let teste: Vec<&str>= instruction.split_whitespace().collect();
        let result = parse_instruction(&*teste);
        assert!(result.is_err());
    }
    #[test]
    fn error_jpaddr_token() {
        let instruction = TestsErrorCollection::JpAddrTokens.value();
        let teste: Vec<&str>= instruction.split_whitespace().collect();
        let result = parse_instruction(&*teste);
        assert!(result.is_err());
    }
    #[test]
    fn error_jpaddr_addr_format() {
        let instruction = TestsErrorCollection::JpAddrAddressFormat.value();
        let teste: Vec<&str>= instruction.split_whitespace().collect();
        let result = parse_instruction(&*teste);
        assert!(result.is_err());
    }
    #[test]
    fn error_jpaddr_addr_size() {
        let instruction = TestsErrorCollection::JpAddrAddressSize.value();
        let teste: Vec<&str>= instruction.split_whitespace().collect();
        let result = parse_instruction(&*teste);
        assert!(result.is_err());
    }
    #[test]
    fn error_call_case() {
        let instruction = TestsErrorCollection::CallCase.value();
        let teste: Vec<&str>= instruction.split_whitespace().collect();
        let result = parse_instruction(&*teste);
        assert!(result.is_err());
    }
    #[test]
    fn error_call_token() {
        let instruction = TestsErrorCollection::CallTokens.value();
        let teste: Vec<&str>= instruction.split_whitespace().collect();
        let result = parse_instruction(&*teste);
        assert!(result.is_err());
    }
    #[test]
    fn error_call_addr_format() {
        let instruction = TestsErrorCollection::CallAddressFormat.value();
        let teste: Vec<&str>= instruction.split_whitespace().collect();
        let result = parse_instruction(&*teste);
        assert!(result.is_err());
    }
    #[test]
    fn error_call_addr_size() {
        let instruction = TestsErrorCollection::CallAddressSize.value();
        let teste: Vec<&str>= instruction.split_whitespace().collect();
        let result = parse_instruction(&*teste);
        assert!(result.is_err());
    }
    #[test]
    fn error_jpv0_case() {
        let instruction = TestsErrorCollection::JpV0Case.value();
        let teste: Vec<&str>= instruction.split_whitespace().collect();
        let result = parse_instruction(&*teste);
        assert!(result.is_err());
    }
    #[test]
    fn error_jpv0_token() {
        let instruction = TestsErrorCollection::JpV0Tokens.value();
        let teste: Vec<&str>= instruction.split_whitespace().collect();
        let result = parse_instruction(&*teste);
        assert!(result.is_err());
    }
    #[test]
    fn error_jpv0_addr_format() {
        let instruction = TestsErrorCollection::JpV0AddressFormat.value();
        let teste: Vec<&str>= instruction.split_whitespace().collect();
        let result = parse_instruction(&*teste);
        assert!(result.is_err());
    }
    #[test]
    fn error_jpv0_addr_size() {
        let instruction = TestsErrorCollection::JpV0AddressSize.value();
        let teste: Vec<&str>= instruction.split_whitespace().collect();
        let result = parse_instruction(&*teste);
        assert!(result.is_err());
    }
    #[test]
    fn error_ldi_case() {
        let instruction = TestsErrorCollection::LdICase.value();
        let teste: Vec<&str>= instruction.split_whitespace().collect();
        let result = parse_instruction(&*teste);
        assert!(result.is_err());
    }
    #[test]
    fn error_ldi_token() {
        let instruction = TestsErrorCollection::LdITokens.value();
        let teste: Vec<&str>= instruction.split_whitespace().collect();
        let result = parse_instruction(&*teste);
        assert!(result.is_err());
    }
    #[test]
    fn error_ldi_addr_format() {
        let instruction = TestsErrorCollection::LdIAddressFormat.value();
        let teste: Vec<&str>= instruction.split_whitespace().collect();
        let result = parse_instruction(&*teste);
        assert!(result.is_err());
    }
    #[test]
    fn error_ldi_addr_size() {
        let instruction = TestsErrorCollection::LdIAddressSize.value();
        let teste: Vec<&str>= instruction.split_whitespace().collect();
        let result = parse_instruction(&*teste);
        assert!(result.is_err());
    }
    #[test]
    fn error_ldvv_case() {
        let instruction = TestsErrorCollection::LdVxVyCase.value();
        let teste: Vec<&str>= instruction.split_whitespace().collect();
        let result = parse_instruction(&*teste);
        assert!(result.is_err());
    }
    #[test]
    fn error_ldvv_token() {
        let instruction = TestsErrorCollection::LdVxVyTokens.value();
        let teste: Vec<&str>= instruction.split_whitespace().collect();
        let result = parse_instruction(&*teste);
        assert!(result.is_err());
    }

    #[test]
    fn error_ldvv_reg_name() {
        let instruction = TestsErrorCollection::LdVxVyRegName.value();
        let teste: Vec<&str>= instruction.split_whitespace().collect();
        let result = parse_instruction(&*teste);
        assert!(result.is_err());
    }
}