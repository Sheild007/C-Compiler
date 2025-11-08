# Rust Lexer Project

This project implements two different lexers for C-like programming languages as required for the Compiler Construction assignment:

1. **Regex-based Lexer** (`lexer_regex.rs`) - Uses regular expressions for token recognition
2. **Manual Lexer** (`lexer_manual.rs`) - Uses a state machine approach with manual string parsing (no third-party libraries)

## Assignment Requirements

This project satisfies the assignment requirements:
- ✅ **Version A**: With regex (using the `regex` crate)
- ✅ **Version B**: Without regex or third-party libraries (pure state machine)
- ✅ **File Output**: Tokens are stored in separate files
- ✅ **Comprehensive Token Support**: All required constructs are supported

## Features

- Tokenizes C-like source code
- Supports keywords, identifiers, literals, operators, and delimiters
- Handles comments, string literals, and escape sequences
- Provides detailed token output with position information
- **Outputs tokens to files** for both lexer versions

## Supported Token Types

### Keywords
- `fn`, `int`, `float`, `string`, `bool`, `return`, `if`, `else`, `while`, `for`

### Identifiers
- Variable and function names (validates that they don't start with numbers)

### Literals
- Integer literals (`42`, `100`)
- Float literals (`3.14`, `2.0`)
- String literals with escape sequences (`"Hello\nWorld"`)
- Boolean literals (`true`, `false`)

### Operators
- Assignment: `=`
- Comparison: `==`, `!=`, `<`, `>`, `<=`, `>=`
- Logical: `&&`, `||`
- Bitwise: `&`, `|`, `<<`, `>>`
- Arithmetic: `+`, `-`, `*`, `/`, `%`

### Delimiters
- Parentheses: `(`, `)`
- Braces: `{`, `}`
- Brackets: `[`, `]`
- Semicolons: `;`
- Commas: `,`
- Quotes: `"`

### Comments
- Single-line comments starting with `//`

## Usage

Due to rustup configuration issues, use the provided script to run the project:

```bash
./run.sh <source_file>
```

### Examples:

```bash
# Test with the assignment sample
./run.sh sample.c

# Test with comprehensive examples
./run.sh comprehensive_test.c

# Test with your own C files
./run.sh your_file.c
```

## Output

The program outputs tokens in two ways:

1. **Console Output**: Shows tokens from both lexers in debug format
2. **File Output**: Saves tokens to separate files in the required format:
   - `regex_tokens.txt` - Tokens from regex-based lexer
   - `manual_tokens.txt` - Tokens from manual lexer

### Token Format

Tokens are output in the format specified in the assignment:
- `T_FUNCTION`, `T_INT`, `T_FLOAT`, etc.
- `T_IDENTIFIER("variable_name")`
- `T_INTLIT(42)`, `T_FLOATLIT(3.14)`
- `T_STRINGLIT("Hello World")`
- `T_ASSIGNOP`, `T_EQUALSOP`, etc.

## Error Handling

- **Invalid identifiers**: Errors are thrown for variable names starting with numbers
- **Unknown characters**: Unrecognized characters are reported as errors
- **String escape sequences**: Properly handles `\n`, `\t`, `\"`, `\\`

## Dependencies

- `regex = "1.10"` - For the regex-based lexer only
- The manual lexer uses **no third-party libraries**

## Building

To build the project:
```bash
PATH=/usr/bin:$PATH cargo build
```

To check for compilation errors:
```bash
PATH=/usr/bin:$PATH cargo check
```

## Project Structure

```
src/
├── main.rs          # Main program with file output functionality
├── lexer_regex.rs   # Regex-based lexer (Version A)
└── lexer_manual.rs  # Manual state machine lexer (Version B)
```

## Sample Output

For the input:
```c
fn int my_fn(int x, float y) {
    string my_str = "hmm";
    bool my_bool = x == 40;
    return x;
}
```

The lexer outputs tokens like:
```
T_FUNCTION
T_INT
T_IDENTIFIER("my_fn")
T_PARENL
T_INT
T_IDENTIFIER("x")
T_COMMA
T_FLOAT
T_IDENTIFIER("y")
T_PARENR
T_BRACEL
T_STRING
T_IDENTIFIER("my_str")
T_ASSIGNOP
T_STRINGLIT("hmm")
T_SEMICOLON
T_BOOL
T_IDENTIFIER("my_bool")
T_ASSIGNOP
T_IDENTIFIER("x")
T_EQUALSOP
T_INTLIT(40)
T_SEMICOLON
T_RETURN
T_IDENTIFIER("x")
T_SEMICOLON
T_BRACER
```
