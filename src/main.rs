mod lexer_regex;
mod lexer_manual;
mod rules;
mod parser;

use regex::Regex;
use std::fs;
use std::env;
use std::io::Write;
use rules::{RULES, Token};

// Rules-based lexer using rules.rs
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
            lexer_regex::Token::Colon => "T_COLON".to_string(),
            lexer_regex::Token::Plus => "T_PLUS".to_string(),
            lexer_regex::Token::Minus => "T_MINUS".to_string(),
            lexer_regex::Token::Mult => "T_MULT".to_string(),
            lexer_regex::Token::Div => "T_DIV".to_string(),
            lexer_regex::Token::Mod => "T_MOD".to_string(),
            lexer_regex::Token::Xor => "T_XOR".to_string(),
            lexer_regex::Token::Not => "T_NOT".to_string(),
            lexer_regex::Token::Question => "T_QUESTION".to_string(),
            lexer_regex::Token::Dot => "T_DOT".to_string(),
            lexer_regex::Token::Arrow => "T_ARROW".to_string(),
            lexer_regex::Token::PlusPlus => "T_PLUSPLUS".to_string(),
            lexer_regex::Token::MinusMinus => "T_MINUSMINUS".to_string(),
            lexer_regex::Token::PlusAssign => "T_PLUSASSIGN".to_string(),
            lexer_regex::Token::MinusAssign => "T_MINUSASSIGN".to_string(),
            lexer_regex::Token::MultAssign => "T_MULTASSIGN".to_string(),
            lexer_regex::Token::DivAssign => "T_DIVASSIGN".to_string(),
            lexer_regex::Token::ModAssign => "T_MODASSIGN".to_string(),
            lexer_regex::Token::LShiftAssign => "T_LSHIFTASSIGN".to_string(),
            lexer_regex::Token::RShiftAssign => "T_RSHIFTASSIGN".to_string(),
            lexer_regex::Token::AndAssign => "T_ANDASSIGN".to_string(),
            lexer_regex::Token::XorAssign => "T_XORASSIGN".to_string(),
            lexer_regex::Token::OrAssign => "T_ORASSIGN".to_string(),
            lexer_regex::Token::LShift => "T_LSHIFT".to_string(),
            lexer_regex::Token::RShift => "T_RSHIFT".to_string(),
            lexer_regex::Token::Hash => "T_HASH".to_string(),
            lexer_regex::Token::Comment(s) => format!("T_COMMENT(\"{}\")", s),
            lexer_regex::Token::BlockComment(s) => format!("T_BLOCKCOMMENT(\"{}\")", s),
            lexer_regex::Token::Preprocessor(s) => format!("T_PREPROCESSOR(\"{}\")", s),
            lexer_regex::Token::Enum => "T_ENUM".to_string(),
            lexer_regex::Token::Struct => "T_STRUCT".to_string(),
            lexer_regex::Token::Typedef => "T_TYPEDEF".to_string(),
            lexer_regex::Token::Static => "T_STATIC".to_string(),
            lexer_regex::Token::Const => "T_CONST".to_string(),
            lexer_regex::Token::Volatile => "T_VOLATILE".to_string(),
            lexer_regex::Token::Extern => "T_EXTERN".to_string(),
            lexer_regex::Token::Auto => "T_AUTO".to_string(),
            lexer_regex::Token::Register => "T_REGISTER".to_string(),
            lexer_regex::Token::Case => "T_CASE".to_string(),
            lexer_regex::Token::Default => "T_DEFAULT".to_string(),
            lexer_regex::Token::Break => "T_BREAK".to_string(),
            lexer_regex::Token::Continue => "T_CONTINUE".to_string(),
            lexer_regex::Token::Goto => "T_GOTO".to_string(),
            lexer_regex::Token::Switch => "T_SWITCH".to_string(),
            lexer_regex::Token::Do => "T_DO".to_string(),
            lexer_regex::Token::Union => "T_UNION".to_string(),
            lexer_regex::Token::Signed => "T_SIGNED".to_string(),
            lexer_regex::Token::Unsigned => "T_UNSIGNED".to_string(),
            lexer_regex::Token::Short => "T_SHORT".to_string(),
            lexer_regex::Token::Long => "T_LONG".to_string(),
            lexer_regex::Token::Double => "T_DOUBLE".to_string(),
            lexer_regex::Token::Char => "T_CHAR".to_string(),
            lexer_regex::Token::Void => "T_VOID".to_string(),
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
            lexer_manual::Token::Colon => "T_COLON".to_string(),
            lexer_manual::Token::Comment(s) => format!("T_COMMENT(\"{}\")", s),
            lexer_manual::Token::Error(s) => format!("T_ERROR(\"{}\")", s),
        };
        writeln!(file, "{}", token_str).expect("Failed to write to file");
    }
}

fn main() {
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

    // Run rules-based lexer
    println!("\n--- Tokens (Rules-based Lexer) ---");
    let tokens_rules = lex(&code);
    for t in &tokens_rules {
        println!("T_{:?}", t);
    }

    // Write tokens to files
    write_regex_tokens_to_file(&tokens_regex, "regex_tokens.txt");
    write_manual_tokens_to_file(&tokens_manual, "manual_tokens.txt");

    println!("\nTokens have been written to:");
    println!("- regex_tokens.txt (Regex-based lexer)");
    println!("- manual_tokens.txt (Manual lexer)");

    // Parse using regex lexer tokens
    println!("\n--- Parsing AST ---");
    println!("Number of tokens: {}", tokens_regex.len());
    let mut parser = parser::Parser::new(tokens_regex);
    match parser.parse_translation_unit() {
        Ok(ast) => {
            println!("AST: {:#?}", ast);
        }
        Err(error) => {
            println!("Parse Error: {:?}", error);
        }
    }
}

