mod lexer_regex;
mod lexer_manual;

use std::fs;
use std::env;
use std::io::Write;

fn main() 
{ 
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        println!("Usage: {} <source_file>", args[0]);
        return;
    }
    let filename = &args[1];
    let code = fs::read_to_string(filename).expect("Failed to read file");
    
    // Run regex lexer
    println!("--- Tokens (Regex Lexer) ---");
    let tokens_regex = lexer_regex::lex_with_regex(&code);
    for t in &tokens_regex {
        println!("{:?}", t);
    }
    
    // Run manual lexer
    println!("\n--- Tokens (Manual Lexer) ---");
    let tokens_manual = lexer_manual::lex_manual(&code);
    for t in &tokens_manual {
        println!("{:?}", t);
    }
    
    // Write tokens to files
    write_regex_tokens_to_file(&tokens_regex, "regex_tokens.txt");
    write_manual_tokens_to_file(&tokens_manual, "manual_tokens.txt");
    
    println!("\nTokens have been written to:");
    println!("- regex_tokens.txt (Regex-based lexer)");
    println!("- manual_tokens.txt (Manual lexer)");
}

fn write_regex_tokens_to_file(tokens: &[lexer_regex::Token], filename: &str) {
    let mut file = fs::File::create(filename).expect("Failed to create file");
    
    for token in tokens {
        let token_str = match token {
            lexer_regex::Token::Function => "T_FUNCTION".to_string(),
            lexer_regex::Token::Int => "T_INT".to_string(),
            lexer_regex::Token::Float => "T_FLOAT".to_string(),
            lexer_regex::Token::String => "T_STRING".to_string(),
            lexer_regex::Token::Bool => "T_BOOL".to_string(),
            lexer_regex::Token::Identifier(s) => format!("T_IDENTIFIER(\"{}\")", s),
            lexer_regex::Token::IntLit(n) => format!("T_INTLIT({})", n),
            lexer_regex::Token::FloatLit(f) => format!("T_FLOATLIT({})", f),
            lexer_regex::Token::StringLit(s) => format!("T_STRINGLIT(\"{}\")", s),
            lexer_regex::Token::BoolLit(b) => format!("T_BOOLLIT({})", b),
            lexer_regex::Token::Return => "T_RETURN".to_string(),
            lexer_regex::Token::If => "T_IF".to_string(),
            lexer_regex::Token::Else => "T_ELSE".to_string(),
            lexer_regex::Token::While => "T_WHILE".to_string(),
            lexer_regex::Token::For => "T_FOR".to_string(),
            lexer_regex::Token::AssignOp => "T_ASSIGNOP".to_string(),
            lexer_regex::Token::EqualsOp => "T_EQUALSOP".to_string(),
            lexer_regex::Token::NotEqualsOp => "T_NOTEQUALSOP".to_string(),
            lexer_regex::Token::LessEqOp => "T_LESSEQOP".to_string(),
            lexer_regex::Token::GreaterEqOp => "T_GREATEREQOP".to_string(),
            lexer_regex::Token::LessOp => "T_LESSOP".to_string(),
            lexer_regex::Token::GreaterOp => "T_GREATEROP".to_string(),
            lexer_regex::Token::AndOp => "T_ANDOP".to_string(),
            lexer_regex::Token::OrOp => "T_OROP".to_string(),
            lexer_regex::Token::BitAndOp => "T_BITANDOP".to_string(),
            lexer_regex::Token::BitOrOp => "T_BITOROP".to_string(),
            lexer_regex::Token::ParenL => "T_PARENL".to_string(),
            lexer_regex::Token::ParenR => "T_PARENR".to_string(),
            lexer_regex::Token::BraceL => "T_BRACEL".to_string(),
            lexer_regex::Token::BraceR => "T_BRACER".to_string(),
            lexer_regex::Token::BracketL => "T_BRACKETL".to_string(),
            lexer_regex::Token::BracketR => "T_BRACKETR".to_string(),
            lexer_regex::Token::Comma => "T_COMMA".to_string(),
            lexer_regex::Token::Semicolon => "T_SEMICOLON".to_string(),
            lexer_regex::Token::Quotes => "T_QUOTES".to_string(),
            lexer_regex::Token::Comment(s) => format!("T_COMMENT(\"{}\")", s),
            lexer_regex::Token::Error(s) => format!("T_ERROR(\"{}\")", s),
        };
        writeln!(file, "{}", token_str).expect("Failed to write to file");
    }
}

fn write_manual_tokens_to_file(tokens: &[lexer_manual::Token], filename: &str) {
    let mut file = fs::File::create(filename).expect("Failed to create file");
    
    for token in tokens {
        let token_str = match token {
            lexer_manual::Token::Function => "T_FUNCTION".to_string(),
            lexer_manual::Token::Int => "T_INT".to_string(),
            lexer_manual::Token::Float => "T_FLOAT".to_string(),
            lexer_manual::Token::String => "T_STRING".to_string(),
            lexer_manual::Token::Bool => "T_BOOL".to_string(),
            lexer_manual::Token::Identifier(s) => format!("T_IDENTIFIER(\"{}\")", s),
            lexer_manual::Token::IntLit(n) => format!("T_INTLIT({})", n),
            lexer_manual::Token::FloatLit(f) => format!("T_FLOATLIT({})", f),
            lexer_manual::Token::StringLit(s) => format!("T_STRINGLIT(\"{}\")", s),
            lexer_manual::Token::BoolLit(b) => format!("T_BOOLLIT({})", b),
            lexer_manual::Token::Return => "T_RETURN".to_string(),
            lexer_manual::Token::If => "T_IF".to_string(),
            lexer_manual::Token::Else => "T_ELSE".to_string(),
            lexer_manual::Token::While => "T_WHILE".to_string(),
            lexer_manual::Token::For => "T_FOR".to_string(),
            lexer_manual::Token::AssignOp => "T_ASSIGNOP".to_string(),
            lexer_manual::Token::EqualsOp => "T_EQUALSOP".to_string(),
            lexer_manual::Token::NotEqualsOp => "T_NOTEQUALSOP".to_string(),
            lexer_manual::Token::LessEqOp => "T_LESSEQOP".to_string(),
            lexer_manual::Token::GreaterEqOp => "T_GREATEREQOP".to_string(),
            lexer_manual::Token::LessOp => "T_LESSOP".to_string(),
            lexer_manual::Token::GreaterOp => "T_GREATEROP".to_string(),
            lexer_manual::Token::AndOp => "T_ANDOP".to_string(),
            lexer_manual::Token::OrOp => "T_OROP".to_string(),
            lexer_manual::Token::BitAndOp => "T_BITANDOP".to_string(),
            lexer_manual::Token::BitOrOp => "T_BITOROP".to_string(),
            lexer_manual::Token::ParenL => "T_PARENL".to_string(),
            lexer_manual::Token::ParenR => "T_PARENR".to_string(),
            lexer_manual::Token::BraceL => "T_BRACEL".to_string(),
            lexer_manual::Token::BraceR => "T_BRACER".to_string(),
            lexer_manual::Token::BracketL => "T_BRACKETL".to_string(),
            lexer_manual::Token::BracketR => "T_BRACKETR".to_string(),
            lexer_manual::Token::Comma => "T_COMMA".to_string(),
            lexer_manual::Token::Semicolon => "T_SEMICOLON".to_string(),
            lexer_manual::Token::Quotes => "T_QUOTES".to_string(),
            lexer_manual::Token::Comment(s) => format!("T_COMMENT(\"{}\")", s),
            lexer_manual::Token::Error(s) => format!("T_ERROR(\"{}\")", s),
        };
        writeln!(file, "{}", token_str).expect("Failed to write to file");
    }
}
