# Testes de Opcodes CHIP-8

## Status de Testes por Instrução

### Instruções com Um Token
- [x] **00E0 (CLS)**
    - ✅ Happy path
    - ✅ Erro léxico
- [x] **00EE (RET)**
    - ✅ Happy path
    - ✅ Erro léxico

### Instruções com Dois Tokens
#### Valor Imediato
- [x] **1nnn (JP addr)**
    - ✅ Happy path
    - ✅ Erro léxico
- [x] **2nnn (CALL addr)**
    - ✅ Happy path
    - ✅ Erro léxico

#### Registradores
- [x] **Ex9E (SKP Vx)**
    - ✅ Happy path
    - ✅ Erro de registrador inválido
- [x] **ExA1 (SKNP Vx)**
    - ✅ Happy path

### Instruções com Três Tokens
- [x] **Bnnn (JP V0, addr)**
    - ✅ Happy path
    - ✅ Erro léxico
- [x] **Annn (LD I, addr)**
    - ✅ Happy path
    - ✅ Erro léxico
- [x] **5xy0 (SE Vx, Vy)**
    - ✅ Happy path
- [x] **8xy0 (LD Vx, Vy)**
    - ✅ Happy path
- [x] **Fx65 (LD Vx, [I])**
    - ✅ Happy path
- [x] **Fx15 (LD DT, Vx)**
    - ✅ Happy path

### Testes Pendentes
#### Instruções que precisam ser testadas:
- [ ] 3xkk - SE Vx, byte
- [ ] 4xkk - SNE Vx, byte
- [ ] Cxkk - RND Vx, byte
- [ ] 7xkk - ADD Vx, byte
- [ ] 6xkk - LD Vx, byte
- [ ] 8xy1 - OR Vx, Vy
- [ ] 8xy2 - AND Vx, Vy
- [ ] 8xy3 - XOR Vx, Vy
- [ ] 8xy4 - ADD Vx, Vy
- [ ] 8xy5 - SUB Vx, Vy
- [ ] 8xy7 - SUBN Vx, Vy
- [ ] 9xy0 - SNE Vx, Vy
- [ ] Fx07 - LD Vx, DT
- [ ] Fx0A - LD Vx, K
- [ ] Fx18 - LD ST, Vx
- [ ] Fx29 - LD F, Vx
- [ ] Fx1E - ADD I, Vx
- [ ] Fx33 - LD B, Vx
- [ ] Fx55 - LD [I], Vx
- [ ] 8xy6 - SHR Vx {, Vy}
- [ ] 8xyE - SHL Vx {, Vy}
- [ ] Dxyn - DRW Vx, Vy, nibble

## Legenda
- ✅ Teste implementado
- ❌ Teste falhou
- [ ] Teste pendente
- [x] Conjunto de testes completo