# Testes de Opcodes CHIP-8

## Status de Testes por Instrução

### Instruções com Um Token
- [x] **00E0 - CLS**
    - ✅ Erro léxico
    - ✅ Erro tokens
    - ✅ Happy path
  
- [x] **00EE - RET**
    - ✅ Erro léxico
    - ✅ Erro tokens
    - ✅ Happy path
  
### Instruções com Dois Tokens
#### Valor Imediato
- [x] **1nnn - JP addr**
  - ✅ Erro léxico
  - ✅ Erro tokens
  - ✅ Erro formato do endereço
  - ✅ Erro limite do endereço
  - ✅ Happy path

- [x] **2nnn - CALL addr**
  - ✅ Erro léxico
  - ✅ Erro tokens
  - ✅ Erro formato do endereço
  - ✅ Erro limite do endereço
  - ✅ Happy path
  
- [x] **Bnnn - JP V0, addr** 
  - ✅ Erro léxico
  - ✅ Erro tokens
  - ✅ Erro formato do endereço
  - ✅ Erro limite do endereço
  - ✅ Happy path
  
#### Teclado
- [x] **Ex9E - SKP Vx**
  - ✅ Erro léxico
  - ✅ Erro tokens
  - ✅ Erro no nome do registrator
  - ✅ Happy path

- [x] **ExA1 - SKNP Vx**
  - ✅ Happy path

### Instruções com Três Tokens
#### Valor Imediato
- [x] **Annn - LD I, addr**
  - ✅ Erro léxico
  - ✅ Erro tokens
  - ✅ Erro formato do endereço
  - ✅ Erro limite do endereço
  - ✅ Happy path  

#### Lógicas e aritméticas 
- [x] **5xy0 - SE Vx, Vy**
  - ✅ Happy path

- [x] **8xy0 - LD Vx, Vy**
  - ✅ Erro léxico
  - ✅ Erro tokens
  - ✅ Erro no nome do registrator

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

### Carregar um byte
- [x] **3xkk - SE Vx, byte**
  - ✅ Erro léxico
  - ✅ Erro tokens
  - ✅ Erro no nome do registrator
  - ✅ Erro limite do endereço
  - ✅ Happy path
  
- [x] **4xkk - SNE Vx, byte**
  - ✅ Happy path

- [x] **7xkk - ADD Vx, byte**
  - ✅ Happy path

- [x] **6xkk - LD Vx, byte**
  - ✅ Happy path

- [x] **Cxkk - RND Vx, byte**
  - ✅ Happy path
### Temporizadores
- [x] **Fx07 - LD Vx, DT**
  - ✅ Erro léxico
  - ✅ Erro tokens
  - ✅ Erro no nome do registrator
  - ✅ Happy path
 
- [x] **Fx0A - LD Vx, K**
  - ✅ Happy path

- [x] **Fx65 - LD Vx, [I]**
  - ✅ Happy path

- [x] **Fx15 - LD DT, Vx**
  - ✅ Erro léxico
  - ✅ Erro tokens
  - ✅ Erro no nome do registrator
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

### Instruções com Quatro Tokens

### Testes Pendentes
#### Instruções que precisam ser testadas

- [ ] 8xy6 - SHR Vx {, Vy}
- [ ] 8xyE - SHL Vx {, Vy}
- [ ] Dxyn - DRW Vx, Vy, nibble

## Legenda
- ✅ Teste implementado
- ❌ Teste falhou
- [ ] Teste pendente
- [x] Conjunto de testes completo