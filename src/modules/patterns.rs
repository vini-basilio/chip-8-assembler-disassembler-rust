
pub enum InstructionKinds {
    Simple,
    U12Address,
    LoadByte,
    Keyboard,
    FRegLabel,
    FLabelReg,
    Logical,
    LogicalExceptions,
    Draw,
}

pub enum Opcode {
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

impl Opcode {
    pub fn value(&self) -> u16 {
        match self {
            // Simples
            Opcode::Cls => 0x00E0,
            Opcode::Ret => 0x00EE,
            // U12Address
            Opcode::LdI => 0xA000,
            Opcode::JpB => 0xB000,
            Opcode::JpOne => 0x1000,
            Opcode::Call => 0x2000,
            // LoadByte
            Opcode::Se => 0x3000,
            Opcode::Sne => 0x4000,
            Opcode::Rnd => 0xC000,
            Opcode::AddByte => 0x6000,
            Opcode::LdByte => 0x7000,
            // Keyboard
            Opcode::Skp => 0xE09E,
            Opcode::Sknp => 0xE0A1,
            // LoadByte
            Opcode::SeRegReg => 0x5000,
            Opcode::LdRegReg => 0x8000,
            Opcode::Or => 0x8001,
            Opcode::And => 0x8002,
            Opcode::Xor => 0x8003,
            Opcode::AddRegReg => 0x8004,
            Opcode::Sub => 0x8005,
            Opcode::Shr => 0x8006,
            Opcode::Subn => 0x8007,
            Opcode::Shl => 0x800E,
            Opcode::SneReg => 0x9000,
            // FRegLabel
            Opcode::RegFromMemo => 0xF065,
            Opcode::DtReg => 0xF007,
            Opcode::WaitKey => 0xF00A,
            // FLabelReg
            Opcode::SetDt => 0xF015,
            Opcode::SetSt => 0xF018,
            Opcode::SetI => 0xF01E,
            Opcode::AddIReg => 0xF029,
            Opcode::StoreBcd => 0xF033,
            Opcode::StoreRegMemo => 0xF055,
            Opcode::Draw => 0xD000,
        }
    }
}