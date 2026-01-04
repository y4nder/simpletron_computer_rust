# Simpletron Computer (Rust) 🦀💻

![Rust](https://img.shields.io/badge/language-Rust-orange)
![CLI](https://img.shields.io/badge/type-CLI-blue)
![Learning Project](https://img.shields.io/badge/status-learning%20project-yellow)

**Author:** Yander
*(Leander Lorenz B. Lubguban)*

---

## Overview

The **Simpletron Computer** is a command-line virtual machine inspired by early stored-program computers and the classic *Simpletron Machine Language (SML)* model.

This project is a **Rust reimplementation and extension** of my original Python-based Simpletron. Unlike the original numeric-only version, this implementation introduces a **proper assembler pipeline**, symbolic programs, and a modular virtual machine architecture.

The project is primarily a **learning exercise in systems programming**, focusing on:

* Instruction encoding and decoding
* Memory modeling
* CPU execution cycles
* Assembler design
* Idiomatic Rust abstractions for low-level systems

---

## High-Level Architecture

The project is divided into three major layers:

1. **Assembler**

   * Parses mnemonic-based source code
   * Resolves symbols and labels
   * Encodes instructions into numeric SML

2. **Virtual Machine (VM)**

   * Memory subsystem
   * CPU / processor
   * Instruction execution logic

3. **Orchestrator**

   * Coordinates program loading
   * Runs the fetch–decode–execute cycle
   * Handles debugging and output

---

## Project Structure

```text
.
├── assembler
│   ├── encoder.rs              # Converts parsed instructions into numeric SML
│   ├── instruction.rs          # Assembler-level instruction definitions
│   ├── mod.rs
│   ├── parser
│   │   ├── lowlevel_parser.rs  # Numeric / low-level instruction parsing
│   │   ├── mnemonic_parser.rs  # Mnemonic-based assembler parser
│   │   ├── mod.rs
│   │   └── parser_interface.rs # Common parser abstraction
│   └── symbol_table.rs         # Variable and label resolution
│
├── cli.rs                      # Command-line interface (argument parsing)
├── lib.rs                      # Library entry point
├── main.rs                     # CLI entry point
├── orchestrator.rs             # Program execution coordinator
│
└── vm
    ├── error
    │   ├── kinds.rs            # Error classifications
    │   └── mod.rs
    │
    ├── instruction.rs          # VM-level instruction representation
    │
    ├── loader
    │   ├── mod.rs
    │   └── parsed_instruction.rs # Loader-facing instruction format
    │
    ├── memory
    │   ├── memory_interface.rs # Memory abstraction
    │   ├── memory_loader.rs    # Loads assembled programs into memory
    │   ├── memory_payload.rs   # Memory cell representation
    │   ├── single_list.rs      # Concrete memory implementation
    │   └── mod.rs
    │
    ├── operation.rs            # Opcode definitions and mapping
    │
    └── processor
        ├── mod.rs
        ├── processor_interface.rs # CPU abstraction
        └── simple_processor.rs    # Accumulator-based CPU implementation
```

---

## Instruction Format (Machine Level)

At the VM level, each instruction is a **4-digit signed integer**:

```
[OPCODE][OPERAND]
```

* **OPCODE** (first two digits): operation to execute
* **OPERAND** (last two digits): memory address or immediate value

Example:

```
1008 → READ input into memory address 08
```

---

## Assembler Language (Current)

The assembler supports a **human-readable mnemonic syntax**, allowing programs to be written without manually managing numeric addresses.

### Features

* Explicit variable declarations using `VAR`
* Labels using `label:` syntax
* Symbol resolution via a symbol table
* Translation to numeric SML

### Supported Instructions (Partial)

* I/O: `READ`, `WRITE`
* Memory: `LOADM`, `LOADI`, `STORE`
* Arithmetic: `ADDM`, `SUBI`, `MULM`
* Control flow: `JMP`, `JZ`
* Program control: `HALT`

---

## Example Program: Factorial

```text
; ------------------------------------------------------------
; Author: Leander Lorenz B. Lubguban BSCS 3-A
; Program: Factorial using Simpletron (current assembler)
;
; Description:
; - Reads an integer n from input
; - Computes n! using a loop
; - Stores the result in variable `fact`
; - Outputs the factorial result
;
; Requirements:
; - Explicit VAR declarations
; - Explicit LOADM / LOADI / MULM
; - Labels use `label:` syntax
; ------------------------------------------------------------

; --------------------
; Variable declarations
; --------------------
VAR n           ; variable to store input number n
VAR fact        ; variable to store factorial result

; --------------------
; Initialization
; --------------------
READ   n        ; read input value into variable n
LOADI  1        ; load constant 1 into accumulator
STORE  fact     ; initialize fact = 1
LOADM  n        ; load n into accumulator (for loop condition)

; --------------------
; Loop: while (n != 0)
; --------------------
loop:
JZ     display  ; if accumulator == 0, jump to display (end loop)

LOADM  fact     ; load current factorial value
MULM   n        ; multiply by n
STORE  fact     ; store updated factorial back to fact

LOADM  n        ; load n
SUBI   1        ; n = n - 1
STORE  n        ; store updated n

JMP    loop     ; repeat loop

; --------------------
; Output result
; --------------------
display:
WRITE  fact     ; output factorial result
HALT            ; stop program execution

```

This program:

1. Reads an integer `n`
2. Computes `n!` using a loop
3. Stores the result in `fact`
4. Outputs the final value

---

## Example Output

```text
*** Welcome to Simpletron ***
*** Program Loaded Succesfully ***

Enter a number: 3
Memory[15] = 6
REGISTERS:
accumulator: +0000
program counter: 13
instruction_register: +4300
opereration_code: +43
operand: +00
Memory Dump:
             0        1        2        3        4        5        6        7        8        9
    00    +1014    +2201    +2115    +2014    +4212    +2015    +3414    +2115    +2014    +3601
    10    +2114    +4004    +1115    +4300    +0000    +0006    +0000    +0000    +0000    +0000
    20    +0000    +0000    +0000    +0000    +0000    +0000    +0000    +0000    +0000    +0000
    30    +0000    +0000    +0000    +0000    +0000    +0000    +0000    +0000    +0000    +0000
    40    +0000    +0000    +0000    +0000    +0000    +0000    +0000    +0000    +0000    +0000
    50    +0000    +0000    +0000    +0000    +0000    +0000    +0000    +0000    +0000    +0000
    60    +0000    +0000    +0000    +0000    +0000    +0000    +0000    +0000    +0000    +0000
    70    +0000    +0000    +0000    +0000    +0000    +0000    +0000    +0000    +0000    +0000
    80    +0000    +0000    +0000    +0000    +0000    +0000    +0000    +0000    +0000    +0000
    90    +0000    +0000    +0000    +0000    +0000    +0000    +0000    +0000    +0000    +0000
```

A full **memory dump** is printed after execution when debugging is enabled.

---

## Command-Line Interface

```text
simpletron_rust [OPTIONS] <FILENAME>
```

### Arguments

* `<FILENAME>` — Path to the assembler source file

### Options

* `--debug` — Display CPU registers and memory state
* `-h, --help` — Show help
* `-V, --version` — Show version

### Usage

```bash
cargo run mnemonic.m
cargo run mnemonic.m --debug
```

---

## Design Goals

* Practice **idiomatic Rust** in a systems context
* Understand **assembler and VM pipelines**
* Model a simple **CPU–memory architecture**
* Explore **error handling and abstractions** in low-level software
* Build a foundation for future extensions

---

## Future Work

Potential future directions include:

* Multiple CPU implementations
* Alternative memory models
* Extended instruction sets
* Optimization passes in the assembler
* Better diagnostics and tracing tools

---

## Status

🚧 **Active learning project**

This project is intentionally iterative. The architecture and abstractions evolve as understanding of compilers, virtual machines, and Rust deepens.
