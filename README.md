# Simpletron Computer (Rust) 🦀💻

![Rust](https://img.shields.io/badge/language-Rust-orange)
![CLI](https://img.shields.io/badge/type-CLI-blue)
![Learning Project](https://img.shields.io/badge/status-learning%20project-yellow)

**Author:** Yander (Leander Lorenz B. Lubguban)

---

## Overview

The **Simpletron Computer** is a small virtual machine inspired by early stored-program computers. This project is a **Rust port** of a Python-based Simpletron made by me, rewritten to explore **instruction decoding, memory modeling, and CPU execution** using Rust.

The current implementation executes **numeric Simpletron Machine Language (SML)** instructions, closely resembling classical low-level machine operation.

---

## Instruction Format

Each instruction is a **4-digit signed integer**:

```
[OPCODE][OPERAND]
```

* **OPCODE** (first two digits): operation to execute
* **OPERAND** (last two digits): memory address or value

Example:

```
1008 → READ input into memory address 08
```

---

## Example Program (Add Two Numbers)

```text
; this program accepts 2 inputs and adds both the inputs 

00  1008    ; read input → address 08
01  1009    ; read input → address 09
02  2008    ; load address 08 to accumulator
03  3009    ; add address 09 to accumulator
04  2110    ; store accumulator → address 10
05  1110    ; write address 10
06  1200    ; write accumulator
07  4300    ; halt
08  0000    ; variable A
09  0000    ; variable B
10  0000    ; result C
```

---

## Command-Line Interface

```text
simpletron_rust [OPTIONS] <FILENAME>
```

### Arguments

* `<FILENAME>` — Path to the `.sml` program file

### Options

* `--debug` — Show CPU and memory state during execution
* `-h, --help` — Print help
* `-V, --version` — Print version

### Usage

```bash
cargo run -- programs/add.sml
cargo run -- programs/add.sml --debug
```

---

## Project Goals

* Practice **Rust fundamentals** through a systems-oriented project
* Model a simple **CPU–memory architecture**
* Explore low-level **instruction execution and control flow**
