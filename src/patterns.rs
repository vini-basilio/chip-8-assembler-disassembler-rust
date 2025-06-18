#[derive(Debug)]
pub enum InstructionKinds {
    Simples,
    U12Address,
    LoadByte,
    Keyboard,
    FRegLabel,
    FLabelReg,
    Logical,
    Draw,
}


#[derive(Debug)]
pub enum Opcode {
    // Simples
    Cls = 0x00E0,
    Ret = 0x00EE,
    // U12Address
    JpB = 0xB000,
    JpOne = 0x1000,
    Call = 0x2000,
    // LoadByte
    Se = 0x3000,
    Sne = 0x4000,
    Rnd = 0xC000,
    AddByte = 0x6000,
    LdByte = 0x7000,
    // Keyboard
    Skp = 0xE09E,
    Sknp = 0xE0A1,
    // LoadByte
    SeRegReg = 0x5000,
    LdRegReg = 0x8000,
    Or = 0x8001,
    And = 0x8002,
    Xor = 0x8003,
    AddRegReg = 0x804,
    Sub = 0x8005,
    Shr =  0x8006,
    Subn = 0x8007,
    Shl = 0x800E,
    SneReg = 0x9000,
    // FRegLabel
    RegFromMemo = 0xF065,
    DtReg = 0xF007,
    WaitKey = 0xF00A,
    // FLabelReg
    SetDt= 0xF015,
    SetSt= 0xF018,
    SetI = 0xF01E,
    AddIReg = 0xF29,
    StoreBcd = 0xF033,
    StoreRegMemo = 0xF055,
    Draw = 0xD000,
    Error = 0x0000,
}
