# Rust Book Exercises

This repository contains my solutions and practice exercises from  
**The Rust Programming Language** (commonly known as *The Rust Book*).

The goal of this repository is to learn Rust by following the official book
and implementing the examples and exercises chapter by chapter.

📘 Official book: https://doc.rust-lang.org/book/

---

## 📂 Repository Structure

Exercises are organized by chapter inside the `exercises/` directory.

├── my-projects
  ├── guessing_game
  ├── hello_cargo
  ├── hello_world
  ├── README.md

Each folder is either:
- a standalone Rust file (`main.rs`), or
- a Cargo project (`Cargo.toml` + `src/`)

---

## 🚀 Running the Exercises

### Using Cargo (recommended)
If the exercise folder contains a `Cargo.toml`:

```bash
cd guessing_game
cargo run
