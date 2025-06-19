#[cfg(test)]
pub mod tests {
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
}