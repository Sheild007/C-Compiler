mod rules;

use regex::Regex;
use std::fs;
use rules::{RULES, Token};

fn lex(mut input: &str) -> Vec<Token> {
    let mut tokens = Vec::new();

    while !input.is_empty() {
        input = input.trim_start();
        if input.is_empty() { break; }

        let mut matched = false;

        for rule in RULES.iter() {
            if let Some(m) = rule.regex.find(input) {
                let lexeme = m.as_str();

                // Special check: invalid identifier like `2abc`
                if Regex::new(r"^\d+[a-zA-Z_]").unwrap().is_match(input) {
                    tokens.push(Token::Error(format!("Invalid identifier: {}", lexeme)));
                    input = &input[m.end()..];
                    matched = true;
                    break;
                }

                tokens.push((rule.token_type)(lexeme));
                input = &input[m.end()..];
                matched = true;
                break;
            }
        }

        if !matched {
            tokens.push(Token::Error(format!("Unexpected character: {}", &input[..1])));
            input = &input[1..];
        }
    }

    tokens
}

fn main() {
    let code = fs::read_to_string("code.c").expect("Failed to read file");

    let tokens = lex(&code);
    for token in tokens {
        println!("T_{:?}", token);
    }
}
