# CHIP-8 Assembler and Disassembler in Rust | CHIP-8 Assembler e Disassembler em Rust

[English](#english) | [Português](#português)

# English

## About the Project
An assembler and disassembler for CHIP-8 implemented in Rust, focused on performance and reliability.

## Project Status
- ✅ Assembler Core
- ✅ CLI Interface with Clap
- ✅ Basic Assembler
- ✅ Basic Disassembler

## Features
- Complete support for CHIP-8 instruction set
- Assembly code parser for CHIP-8 ROM
- Binary object code generation
- Syntax validation
- Machine code parser for CHIP-8 assembly

## Prerequisites
- Rust 1.86.0 or higher
- Cargo (Rust package manager)

## Usage
Usage: chip-8-assembler-rust.exe COMMAND

Commands:
- assembler:     Convert an assembly file to CHIP-8 ROM
- disassembler:  Convert a CHIP-8 ROM to readable assembly
- help:          Print this help message. When choosing a CLI mode, pass the arguments

--input       "PATH TO INPUT FILE"
--output      "PATH TO OUTPUT FILE" (optional)

example:  chip-8-assembler-rust.exe assembler --input "my_rom.txt"
example:  chip-8-assembler-rust.exe disassembler --input "my_rom.ch8"

### Expected Format
This project is focused on being educational, so I focused only on the basics. The expected assembler file should only contain instructions as in the example below:

````txt
LD I, 0x21E
RND V2, 0x01
SE V2, 0x01
````

## Documentation
For more information about supported instructions and implementation details, see:
- [General Documentation](docs/Assembler-english/Documentation.md)
- [Test Documentation](docs/Assembler-english/Tests.md)

## Contact
[LinkedIn](www.linkedin.com/in/vinícius-basílio-93481b254)

## Acknowledgments
- [CHIP-8 Documentation](http://devernay.free.fr/hacks/chip8/C8TECH10.HTM)
- [Disassembler Algorithm](https://medium.com/@sebastien.luc.legrand/chip-8-emulation-part-3-09e439c2b7b0)

---

# Português

## Sobre o Projeto
Um assembler e disassembler para CHIP-8 implementado em Rust, focado em performance e confiabilidade. 
O projeto atualmente está em desenvolvimento e será portado para a lingua inglesa.

## Status do Projeto
- ✅ Core do Assembler
- ✅ Interface CLI com Clap
- ✅ Assembler básico
- ✅ Disassemler básico

## Funcionalidades
- Suporte completo ao conjunto de instruções CHIP-8
- Parser de código assembly para rom CHIP-8
- Geração de código objeto binário
- Validação de sintaxe
- Parser de código de máquina para assembly CHIP-8

## Pré-requisitos
- Rust 1.86.0 ou superior
- Cargo (gerenciador de pacotes do Rust)

## Uso
Uso: chip-8-assembler-rust.exe COMMAND

Commandos:  
- assembler:     Converte um arquivo de assembly para ROM CHIP-8  
- disassembler:  Converte uma ROM CHIP-8 para assembly legível  
- help:          Imprime essas mensagens. Ao escolher um modo de CLI, passe os argumentos

--input       "ENDEREÇO DO OBJETO DE MANIPULAÇÃO"  
--output      "ENDEREÇO DO OBJETO DE CRIADO" (opcional)

exemplo:  chip-8-assembler-rust.exe assembler --input "minha_rom.txt"    
exemplo:  chip-8-assembler-rust.exe disassembler --input "minha_rom.ch8"

### Tipo de formato esperado
Este projeto tem como foco ser educacional, por isso, dei foco apenas no básico. O arquivo de assembler esperado deve apenas conter instruções como no exemplo abaixo.

````txt
LD I, 0x21E
RND V2, 0x01
SE V2, 0x01
````

## Documentação
Para mais informações sobre as instruções suportadas e detalhes de implementação, consulte:
- [Documentação Geral](docs/Assembler-pt/Documentacao.md)
- [Documentação de Testes](docs/Assembler-pt/Testes.md)

## Contato
[Linkedin](www.linkedin.com/in/vinícius-basílio-93481b254)

## Agradecimentos
- [Documentação CHIP-8](http://devernay.free.fr/hacks/chip8/C8TECH10.HTM)
- [Algoritmo do disassembler](https://medium.com/@sebastien.luc.legrand/chip-8-emulation-part-3-09e439c2b7b0)
