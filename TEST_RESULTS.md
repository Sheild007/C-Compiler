# Parser Test Results

## Test Files Created

### 1. `comprehensive_valid.c`
A comprehensive C code file that tests all supported features of the parser:
- Preprocessor directives (#include)
- Global variable declarations (int, float, char, double)
- Static and const variables
- Function declarations
- Function definitions with various features:
  - Simple functions with return
  - Functions with multiple parameters
  - Functions with if-else statements
  - Functions with while loops
  - Functions with for loops
  - Functions with nested loops and conditions
  - Functions with break statements
  - Functions with complex expressions
  - Functions with logical operators (&&, ||)
  - Functions with bitwise operators (&, |, ^)
  - Functions with comparison operators (==, !=, <, >, <=, >=)
- Main function with:
  - Local variable declarations
  - Variable assignments
  - Function calls
  - Complex expressions
  - Complex control flow (nested if-else)
  - Loops with break
  - While loops

### 2. `buggy_code.c`
A C code file with 20 different types of bugs to test error detection:
1. Missing semicolon after variable declaration
2. Missing type specifier
3. Missing identifier after type
4. Missing expression after assignment operator
5. Missing operand after operator
6. Missing closing parenthesis
7. Missing closing brace
8. Missing semicolon after return
9. Missing expression in return
10. Missing condition in if statement
11. Missing condition in while loop
12. Missing semicolon after for loop init
13. Missing expression after assignment operator
14. Missing operand after comparison
15. Missing operand after arithmetic operator
16. Missing closing parenthesis in function call
17. Missing closing brace in block
18. Missing type in function parameter
19. Missing identifier in function parameter
20. Missing semicolon after variable declaration

## How to Test

### Option 1: Using cargo run
```bash
# Test valid code
cargo run comprehensive_valid.c

# Test buggy code
cargo run buggy_code.c
```

### Option 2: Using test script
```bash
# Make script executable
chmod +x test_parser.sh

# Run tests
./test_parser.sh
```

### Option 3: Build and run directly
```bash
# Build the project
cargo build

# Run the executable
./target/debug/cc-compiler comprehensive_valid.c
./target/debug/cc-compiler buggy_code.c
```

## Expected Results

### Valid Code Test
The parser should successfully parse `comprehensive_valid.c` and produce an AST without any parse errors.

### Buggy Code Test
The parser should detect various errors in `buggy_code.c` and report appropriate error messages such as:
- `ExpectedIdentifier` - when identifier is missing
- `ExpectedTypeToken` - when type specifier is missing
- `ExpectedExpr` - when expression is missing
- `ExpectedIntLit` / `ExpectedFloatLit` / `ExpectedStringLit` - when literal is missing
- `FailedToFindToken` - when required token is missing
- `UnexpectedToken` - when unexpected token is found
- `UnexpectedEOF` - when end of file is reached unexpectedly

## Notes

- The valid code file is comprehensive and tests all supported features
- The buggy code file contains 20 different types of errors to test error detection
- Both files are ready to be tested with your parser

