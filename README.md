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
└── programs/                   # Example programs and test cases
    ├── mnemonic.m              # Factorial example
    ├── jg.test.m               # Test for JG (Jump Greater)
    ├── jnz_test.m              # Test for JNZ (Jump Not Zero)
    └── ...
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

## Assembler Language

The assembler supports a **human-readable mnemonic syntax**, allowing programs to be written without manually managing numeric addresses.

### Core Syntax Features

1.  **Variables (`VAR`)**:
    *   Variables must be explicitly declared before use.
    *   Syntax: `VAR variable_name`
    *   Example: `VAR count`

2.  **Labels (`label:`)**:
    *   Used as targets for jump instructions.
    *   Syntax: `label_name:`
    *   Example: `loop_start:`

3.  **Comments (`;`)**:
    *   Everything after a semicolon is ignored.
    *   Example: `LOADM x ; Load x into accumulator`

4.  **Mandatory HALT**:
    *   Every program **must** contain at least one `HALT` instruction.
    *   The assembler will raise a `Missing Halt Command` error if it is missing.

### Supported Instructions

*   **I/O**:
    *   `READ <var>`: Read integer input into variable.
    *   `READI`: Read integer input into accumulator.
    *   `WRITE <var>`: Write variable value to output.
    *   `WRITEA`: Write accumulator value to output.
*   **Memory**:
    *   `LOADM <var>`: Load value from memory to accumulator.
    *   `LOADI <val>`: Load immediate value to accumulator.
    *   `STORE <var>`: Store accumulator value to memory.
*   **Arithmetic (Memory)**:
    *   `ADDM <var>`, `SUBM <var>`, `MULM <var>`, `DIVM <var>`, `MODM <var>`
*   **Arithmetic (Immediate)**:
    *   `ADDI <val>`, `SUBI <val>`, `MULI <val>`, `DIVI <val>`, `MODI <val>`
*   **Control Flow**:
    *   `JMP <label>`: Unconditional jump.
    *   `JZ <label>`: Jump if accumulator is zero.
    *   `JN <label>`: Jump if accumulator is negative.
    *   `JNZ <label>`: Jump if accumulator is **not** zero.
    *   `JG <label>`: Jump if accumulator is greater than zero.
*   **Program Control**:
    *   `HALT`: Stop program execution (Required).

---

## Running and Testing

The project includes several example programs in the `programs/` directory to demonstrate functionality and test specific instructions.

### Basic Usage

To run a program, use `cargo run` followed by the path to the source file:

```bash
cargo run -- programs/mnemonic.m
```

### Debug Mode

To see the internal state (registers, memory dump) during execution, add the `--debug` flag:

```bash
cargo run -- programs/mnemonic.m --debug
```

### Running Test Programs

The `programs/` directory contains test files for validating specific instructions:

**1. Factorial Example (`mnemonic.m`)**
Calculates the factorial of an input number.
```bash
cargo run -- programs/mnemonic.m
```

**2. Jump Greater Test (`jg.test.m`)**
Tests the `JG` (Jump if Greater) instruction logic.
```bash
cargo run -- programs/jg.test.m
```

**3. Jump Not Zero Test (`jnz_test.m`)**
Tests the `JNZ` (Jump if Not Zero) instruction logic.
```bash
cargo run -- programs/jnz_test.m
```

**4. Missing Halt Check**
If you try to run a program without `HALT`, the assembler will reject it:
```bash
# Example if you created a file 'no_halt.m' without HALT
cargo run -- programs/no_halt.m
# Output: error: Missing Halt Command
```

---

## Example Program: Factorial (`programs/mnemonic.m`)

```text
VAR n           ; variable to store input number n
VAR fact        ; variable to store factorial result

READ   n        ; read input value into variable n
LOADI  1        ; load constant 1 into accumulator
STORE  fact     ; initialize fact = 1
LOADM  n        ; load n into accumulator

loop:
JZ     display  ; if accumulator == 0, jump to display
LOADM  fact
MULM   n
STORE  fact
LOADM  n
SUBI   1
STORE  n
JMP    loop

display:
WRITE  fact
HALT            ; MANDATORY: stop program execution
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