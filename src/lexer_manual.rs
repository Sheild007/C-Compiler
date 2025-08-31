// Lexer without regex or third-party libraries
// Pure state machine and string matching

#[derive(Debug, Clone, PartialEq)]
pub enum Token {
    Function,
    Int,
    Float,
    String,
    Bool,
    Identifier(String),
    IntLit(i64),
    FloatLit(f64),
    StringLit(String),
    BoolLit(bool),
    Return,
    If,
    Else,
    While,
    For,
    AssignOp,
    EqualsOp,
    NotEqualsOp,
    LessEqOp,
    GreaterEqOp,
    LessOp,
    GreaterOp,
    AndOp,
    OrOp,
    BitAndOp,
    BitOrOp,
    ParenL,
    ParenR,
    BraceL,
    BraceR,
    BracketL,
    BracketR,
    Comma,
    Semicolon,
    Quotes,
    Comment(String),
    Error(String),
}

fn is_keyword(s: &str) -> Option<Token> {
    match s {
        "fn" => Some(Token::Function),
        "int" => Some(Token::Int),
        "float" => Some(Token::Float),
        "string" => Some(Token::String),
        "bool" => Some(Token::Bool),
        "return" => Some(Token::Return),
        "if" => Some(Token::If),
        "else" => Some(Token::Else),
        "while" => Some(Token::While),
        "for" => Some(Token::For),
        _ => None,
    }
}

pub fn lex_manual(input: &str) -> Vec<Token> {
    let mut tokens = Vec::new();
    let chars: Vec<char> = input.chars().collect();
    let mut i = 0;
    while i < chars.len() {
        let c = chars[i];
        if c.is_whitespace() {
            i += 1;
            continue;
        }
        // Comments
        if c == '/' && i+1 < chars.len() && chars[i+1] == '/' {
            let start = i;
            i += 2;
            while i < chars.len() && chars[i] != '\n' {
                i += 1;
            }
            let comment = &input[start..i];
            tokens.push(Token::Comment(comment.to_string()));
            continue;
        }
        // Identifiers/keywords
        if c.is_ascii_alphabetic() || c == '_' {
            let start = i;
            i += 1;
            while i < chars.len() && (chars[i].is_ascii_alphanumeric() || chars[i] == '_') {
                i += 1;
            }
            let word = &input[start..i];
            if let Some(tok) = is_keyword(word) {
                tokens.push(tok);
            } else {
                // Check for invalid identifier (starts with number)
                if word.chars().next().unwrap().is_ascii_digit() {
                    tokens.push(Token::Error(format!("Invalid identifier: {}", word)));
                } else {
                    tokens.push(Token::Identifier(word.to_string()));
                }
            }
            continue;
        }
        // Numbers
        if c.is_ascii_digit() {
            let start = i;
            let mut is_float = false;
            i += 1;
            while i < chars.len() && chars[i].is_ascii_digit() {
                i += 1;
            }
            if i < chars.len() && chars[i] == '.' {
                is_float = true;
                i += 1;
                while i < chars.len() && chars[i].is_ascii_digit() {
                    i += 1;
                }
            }
            let num = &input[start..i];
            if is_float {
                if let Ok(f) = num.parse() {
                    tokens.push(Token::FloatLit(f));
                } else {
                    tokens.push(Token::Error(format!("Invalid float: {}", num)));
                }
            } else {
                if let Ok(n) = num.parse() {
                    tokens.push(Token::IntLit(n));
                } else {
                    tokens.push(Token::Error(format!("Invalid int: {}", num)));
                }
            }
            continue;
        }
        // String literal
        if c == '"' {
            let _start = i;
            i += 1;
            let mut s = String::new();
            let mut escape = false;
            while i < chars.len() {
                let ch = chars[i];
                if escape {
                    match ch {
                        'n' => s.push('\n'),
                        't' => s.push('\t'),
                        '"' => s.push('"'),
                        '\\' => s.push('\\'),
                        _ => s.push(ch),
                    }
                    escape = false;
                } else if ch == '\\' {
                    escape = true;
                } else if ch == '"' {
                    i += 1;
                    break;
                } else {
                    s.push(ch);
                }
                i += 1;
            }
            tokens.push(Token::StringLit(s));
            continue;
        }
        // Operators and delimiters
        let two = if i+1 < chars.len() { format!("{}{}", chars[i], chars[i+1]) } else { String::new() };
        match two.as_str() {
            "==" => { tokens.push(Token::EqualsOp); i += 2; continue; },
            "!=" => { tokens.push(Token::NotEqualsOp); i += 2; continue; },
            "<=" => { tokens.push(Token::LessEqOp); i += 2; continue; },
            ">=" => { tokens.push(Token::GreaterEqOp); i += 2; continue; },
            "&&" => { tokens.push(Token::AndOp); i += 2; continue; },
            "||" => { tokens.push(Token::OrOp); i += 2; continue; },
            _ => {}
        }
        match c {
            '=' => { tokens.push(Token::AssignOp); },
            '<' => { tokens.push(Token::LessOp); },
            '>' => { tokens.push(Token::GreaterOp); },
            '&' => { tokens.push(Token::BitAndOp); },
            '|' => { tokens.push(Token::BitOrOp); },
            '(' => { tokens.push(Token::ParenL); },
            ')' => { tokens.push(Token::ParenR); },
            '{' => { tokens.push(Token::BraceL); },
            '}' => { tokens.push(Token::BraceR); },
            '[' => { tokens.push(Token::BracketL); },
            ']' => { tokens.push(Token::BracketR); },
            ',' => { tokens.push(Token::Comma); },
            ';' => { tokens.push(Token::Semicolon); },
            '"' => { tokens.push(Token::Quotes); },
            _ => { tokens.push(Token::Error(format!("Unknown char: {}", c))); },
        }
        i += 1;
    }
    tokens
}
