# CHIP-8 Opcodes Documentation

## Overview
CHIP-8 has two main structures for interpreting opcodes:
- **Token Structure**: Defines how many tokens can be found in a line
- **Opcode Structure**: Defines what the opcode family looks like

## Operation
Tokens are used to determine which extraction function will be applied.

## Processing Flow
1. The function analyzes specific patterns in the token
2. Based on these patterns, the opcode family is determined

## Classification Structure
The enum representing opcode families uses bit masks,
allowing similar instructions to be grouped into specific categories.

## Grouping Example
Instructions that share similar characteristics are grouped
in the same family through corresponding bit masks.

## Opcode Format

### Single Token Instructions
| Opcode | Instruction | Description | Family |
|--------|-------------|-------------|---------|
| 00E0   | CLS        | Clear screen | Simple |
| 00EE   | RET        | Return from subroutine | Simple |

### Two Token Instructions
#### Immediate Value
| Opcode | Instruction | Description | Family |
|--------|-------------|-------------|---------|
| 1nnn   | JP addr    | Jump to address | U12Address |
| 2nnn   | CALL addr  | Call subroutine | U12Address |

#### Registers
| Opcode | Instruction | Description | Family |
|--------|-------------|-------------|---------|
| Ex9E   | SKP Vx     | Skip if key in Vx is pressed | Keyboard |
| ExA1   | SKNP Vx    | Skip if key in Vx is not pressed | Keyboard |

### Three Token Instructions
#### Operations with Registers and Values
| Opcode | Instruction | Description | Family |
|--------|-------------|-------------|---------|
| Bnnn   | JP V0, addr | Jump to V0 + addr | U12Address |
| Annn   | LD I, addr  | Load addr into I | U12Address |
| 3xkk   | SE Vx, byte | Skip if Vx == byte | LoadByte |
| 4xkk   | SNE Vx, byte| Skip if Vx != byte | LoadByte |
| Cxkk   | RND Vx, byte| Generate random number AND byte | LoadByte |
| 7xkk   | ADD Vx, byte| Add byte to Vx | LoadByte |
| 6xkk   | LD Vx, byte | Load byte into Vx | LoadByte |

#### Operations Between Registers
| Opcode | Instruction | Description | Family |
|--------|-------------|-------------|---------|
| 5xy0   | SE Vx, Vy  | Skip if Vx == Vy | Logical |
| 8xy0   | LD Vx, Vy  | Vx = Vy | Logical |
| 8xy1   | OR Vx, Vy  | Vx = Vx OR Vy | Logical |
| 8xy2   | AND Vx, Vy | Vx = Vx AND Vy | Logical |
| 8xy3   | XOR Vx, Vy | Vx = Vx XOR Vy | Logical |
| 8xy4   | ADD Vx, Vy | Vx = Vx + Vy | Logical |
| 8xy5   | SUB Vx, Vy | Vx = Vx - Vy | Logical |
| 8xy7   | SUBN Vx, Vy| Vx = Vy - Vx | Logical |
| 9xy0   | SNE Vx, Vy | Skip if Vx != Vy | Logical |

### Timer and I Register Instructions
| Opcode | Instruction | Description | Family |
|--------|-------------|-------------|---------|
| Fx65   | LD Vx, [I]   | Read registers V0 through Vx starting at address I | FRegLabel |
| Fx07   | LD Vx, DT    | Set Vx to the current value of delay timer | FRegLabel |
| Fx0A   | LD Vx, K     | Wait for key press and store in Vx | FRegLabel |
| Fx15   | LD DT, Vx    | Set delay timer to value in Vx | FLabelReg |
| Fx18   | LD ST, Vx    | Set sound timer to value in Vx | FLabelReg |
| Fx29   | LD F, Vx     | Set I to location of sprite for digit Vx | FLabelReg |
| Fx1E   | ADD I, Vx    | Add Vx to register I | FLabelReg |
| Fx33   | LD B, Vx     | Store BCD representation of Vx in I, I+1, I+2 | FLabelReg |
| Fx55   | LD [I], Vx   | Store registers V0 through Vx starting at address I | FLabelReg |

### Four Token Instructions
| Opcode | Instruction | Description | Family |
|--------|-------------|-------------|---------|
| 8xy6   | SHR Vx {, Vy} | Shift right | Logical |
| 8xyE   | SHL Vx {, Vy} | Shift left | Logical |
| Dxyn   | DRW Vx, Vy, n | Draw sprite | Draw |