# Documentação de Opcodes CHIP-8

## Visão Geral
O CHIP-8 possui duas estruturas principais para interpretar opcodes:
- **Estrutura de Tokens**: Define quantos tokens podem ser encontrados em uma linha
- **Estrutura de Opcode**: Define como a família do opcode se parece

## Funcionamento
Os tokens são utilizados para determinar qual função de extração será aplicada.

## Fluxo de Processamento
1. A função analisa padrões específicos no token
2. Com base nesses padrões, determina-se a família do opcode

## Estrutura de Classificação
O enum que representa as famílias de opcodes utiliza máscaras de bits,
permitindo agrupar instruções similares em categorias específicas.

## Exemplo de Agrupamento
Instruções que compartilham características semelhantes são agrupadas
na mesma família através de máscaras de bits correspondentes.

## Formato dos Opcodes

### Instruções com Um Token
| Opcode | Instrução | Descrição | Família | 
|--------|-----------|-----------|---------|
| 00E0   | CLS       | Limpa a tela | Simple  | 
| 00EE   | RET       | Retorno de subrotina | Simple  | 

### Instruções com Dois Tokens
#### Valor Imediato
| Opcode | Instrução | Descrição | Família | 
|--------|-----------|-----------|---------|
| 1nnn   | JP addr   | Pula para endereço | U12Address |
| 2nnn   | CALL addr | Chama subrotina | U12Address |

#### Registradores
| Opcode | Instrução | Descrição | Família |
|--------|-----------|-----------|---------|
| Ex9E   | SKP Vx    | Pula se tecla em Vx estiver pressionada | Keyboard | 
| ExA1   | SKNP Vx   | Pula se tecla em Vx não estiver pressionada | Keyboard | 

### Instruções com Três Tokens
#### Operações com Registradores e Valores
| Opcode | Instrução | Descrição | Família |
|--------|-----------|-----------|---------|
| Bnnn   | JP V0, addr | Pula para V0 + addr | U12Address |
| Annn   | LD I, addr  | Carrega addr em I | U12Address |
| 3xkk   | SE Vx, byte | Pula se Vx == byte | LoadByte |
| 4xkk   | SNE Vx, byte| Pula se Vx != byte | LoadByte |
| Cxkk   | RND Vx, byte| Gera número aleatório AND byte | LoadByte |
| 7xkk   | ADD Vx, byte| Adiciona byte a Vx | LoadByte |
| 6xkk   | LD Vx, byte | Carrega byte em Vx | LoadByte |

#### Operações entre Registradores
| Opcode | Instrução | Descrição | Família |
|--------|-----------|-----------|---------|
| 5xy0   | SE Vx, Vy  | Pula se Vx == Vy | Logical |
| 8xy0   | LD Vx, Vy  | Vx = Vy | Logical |
| 8xy1   | OR Vx, Vy  | Vx = Vx OR Vy | Logical |
| 8xy2   | AND Vx, Vy | Vx = Vx AND Vy | Logical |
| 8xy3   | XOR Vx, Vy | Vx = Vx XOR Vy | Logical |
| 8xy4   | ADD Vx, Vy | Vx = Vx + Vy | Logical |
| 8xy5   | SUB Vx, Vy | Vx = Vx - Vy | Logical |
| 8xy7   | SUBN Vx, Vy| Vx = Vy - Vx | Logical |
| 9xy0   | SNE Vx, Vy | Pula se Vx != Vy | Logical |

### Instruções com Timer e Registrador I
| Opcode | Instrução | Descrição | Família |
|--------|-----------|-----------|---------|
| Fx65   | LD Vx, [I]   | Lê registradores V0 até Vx começando do endereço I | FRegLabel |
| Fx07   | LD Vx, DT    | Define Vx com o valor atual do delay timer | FRegLabel |
| Fx0A   | LD Vx, K     | Aguarda pressionar tecla e armazena em Vx | FRegLabel |
| Fx15   | LD DT, Vx    | Define delay timer com valor de Vx | FLabelReg |
| Fx18   | LD ST, Vx    | Define sound timer com valor de Vx | FLabelReg |
| Fx29   | LD F, Vx     | Define I como localização do sprite do dígito Vx | FLabelReg |
| Fx1E   | ADD I, Vx    | Adiciona Vx ao registrador I | FLabelReg |
| Fx33   | LD B, Vx     | Armazena representação BCD de Vx em I, I+1, I+2 | FLabelReg |
| Fx55   | LD [I], Vx   | Armazena registradores V0 até Vx começando no endereço I | FLabelReg |

### Instruções com Quatro Tokens
| Opcode | Instrução | Descrição | Família |
|--------|-----------|-----------|---------|
| 8xy6   | SHR Vx {, Vy} | Shift right | Logical |
| 8xyE   | SHL Vx {, Vy} | Shift left | Logical |
| Dxyn   | DRW Vx, Vy, n | Desenha sprite | Draw |

