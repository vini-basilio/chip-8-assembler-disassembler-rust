pub enum TestsErrorCollection {
    ClsCase,
    ClsTokens,
    RetCase,
    RetTokens,
    JpAddrCase,
    JpAddrTokens,
    JpAddrAddressFormat,
    JpAddrAddressSize,
}

impl TestsErrorCollection {
    pub fn value(&self) -> &str {
        match self {
            TestsErrorCollection::ClsCase => "Cls",
            TestsErrorCollection::ClsTokens => "Cls I",
            TestsErrorCollection::RetCase => "Ret",
            TestsErrorCollection::RetTokens => "RET vx",
            TestsErrorCollection::JpAddrCase => "jp 0x200",
            TestsErrorCollection::JpAddrTokens => "JP 0x200 vx",
            TestsErrorCollection::JpAddrAddressFormat => "JP 200",
            TestsErrorCollection::JpAddrAddressSize => "JP 0x1200",
        }
    }
}