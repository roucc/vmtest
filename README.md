# README

## Overview

This project is a simple Rust-based virtual machine (VM) that executes instructions from an assembly-like language. The VM supports stack-based operations, memory manipulation, and branching.

Features:
- **Two-Pass Assembly**: First pass records labels; second pass resolves addresses.
- **Instruction Set**: `inc`, `dec`, `push`, `pop`, `jmp`, `jz`, `jnz`, `call`, `ret`, `nand`, `halt`, `pick`, `poke`, `swap`, `load`, and `store`.
- **Stack**: Used for pushing, popping, and manipulation of operands.
- **Memory**: A fixed-size (256 bytes) array for load/store operations.

## Prerequisites

- **Rust**: Install from [rust-lang.org](https://www.rust-lang.org/tools/install).

## Building and Running

1. **Clone** the repository.
2. **Create** an assembly file (e.g., `code.asm`).
3. **Build and run**:
   ```bash
   cargo build
   cargo run code.asm
   ```
   Replace `code.asm` with your assembly source.

## Assembly Syntax

- Labels:
  ```
  loop:
      inc
      jnz loop
  ```
- Comments start with `;`. Example:
  ```
  ; this is a comment
  ```
- Instructions without operands:
  ```
  inc
  dec
  halt
  ```
- Instructions with operands:
  ```
  push 10
  jmp loop
  load 42
  store 42
  ```
- The assembler records labels on pass 1, then resolves jumps on pass 2.

## Execution Flow

- The VM uses a program counter (`pc`) to track the current instruction.
- Stack operations modify the top of the stack.
- Conditional jumps (`jz`, `jnz`) check the top stack value.
- Calls push the return address, and `ret` pops it.
- `halt` terminates execution.
