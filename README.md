# CHIP-8 Assembler  e Disassembler em Rust

## Sobre o Projeto
Um assembler e disassembler para CHIP-8 implementado em Rust, focado em performance e confiabilidade. 
O projeto atualmente está em desenvolvimento e será portado para a lingua inglesa.

## Status do Projeto
**Em Desenvolvimento** 

- ✅ Core do Assembler
-  Interface CLI com Clap

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

## Documentação
Para mais informações sobre as instruções suportadas e detalhes de implementação, consulte:
- [Documentação Geral](docs/Documentacao.md)
- [Documentação de Testes](docs/Testes.md)

## Contato
[Linkedin](www.linkedin.com/in/vinícius-basílio-93481b254)

## Agradecimentos
- [Documentação CHIP-8](http://devernay.free.fr/hacks/chip8/C8TECH10.HTM)
