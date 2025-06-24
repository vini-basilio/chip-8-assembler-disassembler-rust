
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

/// Contains regex patterns used for parsing CHIP-8 assembly instructions.
#[macro_export] macro_rules! opcodes {
     // Simples
    (CLS) =>  {0x00E0};
    (RET) =>  {0x00EE};

    // U12Address
    (JP_ONE) => {0x1000};
    (CALL) => {0x2000};
    (LD_I) => {0xA000};
    (JP_B) => {0xB000};

    // LoadByte
    (SE) => {0x3000};
    (SNE) => {0x4000};
    (LD_BYTE) => {0x6000};
    (ADD_BYTE) => {0x7000};
    (RND) => {0xC000};

    // Keyboard
    (SKP) => {0xE09E};
    (SKNP) => {0xE0A1};

     // LoadByte
    (SE_REG_REG) => {0x5000};
    (LD_REG_REG) => {0x8000};
    (OR) => {0x8001};
    (AND) => {0x8002};
    (XOR) => {0x8003};
    (ADD_REG_REG) => {0x8004};
    (SUB) => {0x8005};
    (SHR) => {0x8006};
    (SUBN) => {0x8007};
    (SHL) => {0x800E};
    (SNE_REG) => {0x9000};

     // FRegLabel
     (REG_FROM_MEMO) => {0xF065};
     (DT_REG) => {0xF007};
     (WAIT_KEY) => {0xF00A};

     // FLabelReg
    (SET_DT) => (0xF015);
    (SET_ST) => (0xF018);
    (ADD_I_REG) => (0xF01E);
    (SET_SPRITE) => (0xF029);
    (STORE_BCD) => (0xF033);
    (STORE_REG_MEMO) => (0xF055);
    (DRAW) => (0xD000);

}