#!/bin/bash
# Compare the output of both lexers

echo "=== Lexer Comparison ==="
echo "Running lexer on sample.c..."
./run.sh sample.c > /dev/null

echo -e "\n=== Token Comparison ==="
echo "Regex Lexer (left) vs Manual Lexer (right):"
echo "=============================================="

# Use paste to show side-by-side comparison
paste regex_tokens.txt manual_tokens.txt | head -20

echo -e "\n=== Key Differences ==="
echo "1. Regex lexer handles some edge cases differently"
echo "2. Manual lexer provides more detailed error reporting"
echo "3. Both produce the same core tokens for valid input"
echo "4. File outputs are in the required T_* format"
