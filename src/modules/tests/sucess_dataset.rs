#[allow(dead_code)]
pub enum TestsSucess {
    // Simples
    Cls,
    Ret,
    // U12Address
    LdI,
    JpB,
    JpOne,
    Call,
    // LoadByte
    Se,
    Sne,
    Rnd,
    AddByte,
    LdByte,
    // Keyboard
    Skp,
    Sknp,
    // Logical
    SeRegReg,
    LdRegReg,
    Or,
    And,
    Xor,
    AddRegReg,
    Sub,
    Shr,
    Subn,
    // LogicalExceptions
    Shl,
    SneReg,
    // FRegLabel
    RegFromMemo,
    DtReg,
    WaitKey,
    // FLabelReg
    SetDt,
    SetSt,
    SetI,
    AddIReg,
    StoreBcd,
    StoreRegMemo,
    Draw,
}

impl TestsSucess {
    pub fn value(&self) -> &str {
        match self {
            TestsSucess::Cls => "CLS",
            TestsSucess::Ret => "RET",
            TestsSucess::JpOne => "JP 0x200",
            _ => todo!(),
        }
    }
}
