

use regex::Regex;

#[derive(Debug, Clone, PartialEq)]
#[allow(dead_code)]
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

pub fn lex_with_regex(input: &str) -> Vec<Token> {
    let mut tokens = Vec::new();
    let re = Regex::new(
        r#"(?P<ws>\s+)|(?P<comment>//.*)|(?P<function>fn)\b|(?P<return>return)\b|(?P<if>if)\b|(?P<else>else)\b|(?P<while>while)\b|(?P<for>for)\b|(?P<int>int)\b|(?P<float>float)\b|(?P<string>string)\b|(?P<bool>bool)\b|(?P<floatlit>\d+\.\d+)|(?P<intlit>\d+)|(?P<stringlit>"([^\\"]|\\.)*")|(?P<equalsop>==)|(?P<notequalsop>!=)|(?P<lesseqop><=)|(?P<greatereqop>>=)|(?P<andop>&&)|(?P<orop>\|\|)|(?P<assignop>=)|(?P<lessop><)|(?P<greaterop>>)|(?P<bitandop>&)|(?P<bitorop>\|)|(?P<identifier>[a-zA-Z_][a-zA-Z0-9_]*)|(?P<parenl>\()|(?P<parenr>\))|(?P<bracel>\{)|(?P<bracer>\})|(?P<bracketl>\[)|(?P<bracketr>\])|(?P<comma>,)|(?P<semicolon>;)|(?P<quotes>")"#
    ).unwrap();
    let mut pos = 0;
    while pos < input.len() {
        if let Some(m) = re.find(&input[pos..]) {
            let s = &input[pos + m.start()..pos + m.end()];
            let caps = re.captures(s).unwrap();
            if caps.name("ws").is_some() {
                // skip
            } else if let Some(_) = caps.name("comment") {
                tokens.push(Token::Comment(s.to_string()));
            } else if let Some(_) = caps.name("function") {
                tokens.push(Token::Function);
            } else if let Some(_) = caps.name("int") {
                tokens.push(Token::Int);
            } else if let Some(_) = caps.name("float") {
                tokens.push(Token::Float);
            } else if let Some(_) = caps.name("string") {
                tokens.push(Token::String);
            } else if let Some(_) = caps.name("bool") {
                tokens.push(Token::Bool);
            } else if let Some(_) = caps.name("return") {
                tokens.push(Token::Return);
            } else if let Some(_) = caps.name("if") {
                tokens.push(Token::If);
            } else if let Some(_) = caps.name("else") {
                tokens.push(Token::Else);
            } else if let Some(_) = caps.name("while") {
                tokens.push(Token::While);
            } else if let Some(_) = caps.name("for") {
                tokens.push(Token::For);
            } else if let Some(id) = caps.name("identifier") {
                tokens.push(Token::Identifier(id.as_str().to_string()));
            } else if let Some(lit) = caps.name("intlit") {
                tokens.push(Token::IntLit(lit.as_str().parse().unwrap()));
            } else if let Some(lit) = caps.name("floatlit") {
                tokens.push(Token::FloatLit(lit.as_str().parse().unwrap()));
            } else if let Some(lit) = caps.name("stringlit") {
                let s = &lit.as_str()[1..lit.as_str().len()-1];
                tokens.push(Token::StringLit(s.to_string()));
            } else if let Some(_) = caps.name("assignop") {
                tokens.push(Token::AssignOp);
            } else if let Some(_) = caps.name("equalsop") {
                tokens.push(Token::EqualsOp);
            } else if let Some(_) = caps.name("notequalsop") {
                tokens.push(Token::NotEqualsOp);
            } else if let Some(_) = caps.name("lesseqop") {
                tokens.push(Token::LessEqOp);
            } else if let Some(_) = caps.name("greatereqop") {
                tokens.push(Token::GreaterEqOp);
            } else if let Some(_) = caps.name("lessop") {
                tokens.push(Token::LessOp);
            } else if let Some(_) = caps.name("greaterop") {
                tokens.push(Token::GreaterOp);
            } else if let Some(_) = caps.name("andop") {
                tokens.push(Token::AndOp);
            } else if let Some(_) = caps.name("orop") {
                tokens.push(Token::OrOp);
            } else if let Some(_) = caps.name("bitandop") {
                tokens.push(Token::BitAndOp);
            } else if let Some(_) = caps.name("bitorop") {
                tokens.push(Token::BitOrOp);
            } else if let Some(_) = caps.name("parenl") {
                tokens.push(Token::ParenL);
            } else if let Some(_) = caps.name("parenr") {
                tokens.push(Token::ParenR);
            } else if let Some(_) = caps.name("bracel") {
                tokens.push(Token::BraceL);
            } else if let Some(_) = caps.name("bracer") {
                tokens.push(Token::BraceR);
            } else if let Some(_) = caps.name("bracketl") {
                tokens.push(Token::BracketL);
            } else if let Some(_) = caps.name("bracketr") {
                tokens.push(Token::BracketR);
            } else if let Some(_) = caps.name("comma") {
                tokens.push(Token::Comma);
            } else if let Some(_) = caps.name("semicolon") {
                tokens.push(Token::Semicolon);
            } else if let Some(_) = caps.name("quotes") {
                tokens.push(Token::Quotes);
            } else {
                tokens.push(Token::Error(format!("Unknown token: {}", s)));
            }
            pos += m.end();
        } else {
            tokens.push(Token::Error(format!("Unknown sequence at {}", pos)));
            break;
        }
    }
    tokens
}
