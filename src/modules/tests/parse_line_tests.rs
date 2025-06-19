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
}