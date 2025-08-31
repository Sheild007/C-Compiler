# Rust Lexer with Regex Rules

This project is a simple **lexer (tokenizer)** written in Rust.  
It reads source code from a `code.c` file in the project root and generates tokens using **regex-based rules**.

## Project Structure
hello_world/
│-- Cargo.toml
│-- src/
│   ├── main.rs      # Entry point, handles file reading
│   ├── rules.rs     # Token definitions & regex rules
│   └── lib.rs       # (optional, for modularization)
│-- code.c           # Source code file to tokenize

## Where to Change Code
- Regex token rules → `src/rules.rs`  
  Add or modify regex patterns here to support new keywords, operators, identifiers, literals, etc.
  - Add keywords (if, else, while)  
  - Add operators (+, -, *, /)  
  - Add new token types (e.g., Boolean, HexNumber)  

- File reading logic → `src/main.rs`  
  The lexer reads from `code.c` in the root directory by default.  
  Change this path if you want to read from another file or take input dynamically.

- Token handling / output → `src/main.rs`  
  Currently tokens are printed to the console.  
  You can modify this to:
  - Store tokens in a vector  
  - Write tokens to a file  
  - Pass tokens to a parser for further compilation steps  

## Running the Project
1. Install Rust → https://www.rust-lang.org/tools/install  
2. Clone this project or create a new one with:  
   cargo new hello_world  
3. Place your code in `code.c` in the project root.  
4. Run the lexer:  
   cargo run  

