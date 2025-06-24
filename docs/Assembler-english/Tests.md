# CHIP-8 Opcode Tests

## Test Status by Instruction

### Instructions with One Token
- [x] **00E0 - CLS**
  - ✅ Lexical error
  - ✅ Token error
  - ✅ Happy path

- [x] **00EE - RET**
  - ✅ Lexical error
  - ✅ Token error
  - ✅ Happy path

### Instructions with Two Tokens
#### Immediate Value
- [x] **1nnn - JP addr**
  - ✅ Lexical error
  - ✅ Token error
  - ✅ Address format error
  - ✅ Address limit error
  - ✅ Happy path

- [x] **2nnn - CALL addr**
  - ✅ Lexical error
  - ✅ Token error
  - ✅ Address format error
  - ✅ Address limit error
  - ✅ Happy path

- [x] **Bnnn - JP V0, addr**
  - ✅ Lexical error
  - ✅ Token error
  - ✅ Address format error
  - ✅ Address limit error
  - ✅ Happy path

#### Keyboard
- [x] **Ex9E - SKP Vx**
  - ✅ Lexical error
  - ✅ Token error
  - ✅ Invalid register name
  - ✅ Happy path

- [x] **ExA1 - SKNP Vx**
  - ✅ Happy path

### Instructions with Three Tokens
#### Immediate Value
- [x] **Annn - LD I, addr**
  - ✅ Lexical error
  - ✅ Token error
  - ✅ Address format error
  - ✅ Address limit error
  - ✅ Happy path

#### Logic and Arithmetic
- [x] **5xy0 - SE Vx, Vy**
  - ✅ Happy path

- [x] **8xy0 - LD Vx, Vy**
  - ✅ Lexical error
  - ✅ Token error
  - ✅ Invalid register name

- [x] **8xy1 - OR Vx, Vy**
  - ✅ Happy path

- [x] **8xy2 - AND Vx, Vy**
  - ✅ Happy path

- [x] **8xy3 - XOR Vx, Vy**
  - ✅ Happy path

- [x] **8xy4 - ADD Vx, Vy**
  - ✅ Happy path

- [x] **8xy5 - SUB Vx, Vy**
  - ✅ Happy path

- [x] **8xy7 - SUBN Vx, Vy**
  - ✅ Happy path

- [x] **9xy0 - SNE Vx, Vy**
  - ✅ Happy path

### Load a Byte
- [x] **3xkk - SE Vx, byte**
  - ✅ Lexical error
  - ✅ Token error
  - ✅ Invalid register name
  - ✅ Address limit error
  - ✅ Happy path

- [x] **4xkk - SNE Vx, byte**
  - ✅ Happy path

- [x] **7xkk - ADD Vx, byte**
  - ✅ Happy path

- [x] **6xkk - LD Vx, byte**
  - ✅ Happy path

- [x] **Cxkk - RND Vx, byte**
  - ✅ Happy path

### Timers
- [x] **Fx07 - LD Vx, DT**
  - ✅ Lexical error
  - ✅ Token error
  - ✅ Invalid register name
  - ✅ Happy path

- [x] **Fx0A - LD Vx, K**
  - ✅ Happy path

- [x] **Fx65 - LD Vx, [I]**
  - ✅ Happy path

- [x] **Fx15 - LD DT, Vx**
  - ✅ Lexical error
  - ✅ Token error
  - ✅ Invalid register name
  - ✅ Happy path

- [x] **Fx18 - LD ST, Vx**
  - ✅ Happy path

- [x] **Fx29 - LD F, Vx**
  - ✅ Happy path

- [x] **Fx1E - ADD I, Vx**
  - ✅ Happy path

- [x] **Fx33 - LD B, Vx**
  - ✅ Happy path

- [x] Fx55 - LD [I], Vx
  - ✅ Happy path

### Instructions with Four Tokens
- [x] 8xy6 - SHR Vx {, Vy}
  - ✅ Happy path
- [x] 8xyE - SHL Vx {, Vy}
  - ✅ Happy path
- [x] Dxyn - DRW Vx, Vy, nibble
  - ✅ Happy path

### Pending Tests
#### Instructions that still need testing

## Legend
- ✅ Test implemented
- ❌ Test failed
- [ ] Test pending
- [x] Test set complete
