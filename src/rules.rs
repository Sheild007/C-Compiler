use regex::Regex;
use lazy_static::lazy_static;

#[derive(Debug)]
pub enum Token {
    // Keywords
    KeywordInt,
    KeywordFloat,
    KeywordChar,
    KeywordVoid,
    KeywordReturn,
    KeywordIf,
    KeywordElse,
    KeywordWhile,
    KeywordFor,
    KeywordStruct,

    // Literals
    Identifier(String),
    Int(i64),
    Float(f64),
    StringLit(String),
    CharLit(char),

    // Symbols
    ParenL,
    ParenR,
    BraceL,
    BraceR,
    BracketL,
    BracketR,
    Semicolon,
    Comma,

    // Operators
    Plus,
    Minus,
    Star,
    Slash,
    Percent,
    EqEq,
    NotEq,
    Lt,
    Gt,
    LtEq,
    GtEq,
    AndAnd,
    OrOr,
    Not,
    Assign,
    Amp,
    Pipe,
    Caret,
    Shl,
    Shr,

    // Comments
    Comment(String),

    // Error handling
    Error(String),
}

pub struct Rule {
    pub regex: Regex,
    pub token_type: fn(&str) -> Token,
}

lazy_static! {
    pub static ref RULES: Vec<Rule> = vec![
        // ===== Keywords =====
        Rule { regex: Regex::new(r"^\bint\b").unwrap(),    token_type: |_| Token::KeywordInt },
        Rule { regex: Regex::new(r"^\bfloat\b").unwrap(),  token_type: |_| Token::KeywordFloat },
        Rule { regex: Regex::new(r"^\bchar\b").unwrap(),   token_type: |_| Token::KeywordChar },
        Rule { regex: Regex::new(r"^\bvoid\b").unwrap(),   token_type: |_| Token::KeywordVoid },
        Rule { regex: Regex::new(r"^\breturn\b").unwrap(), token_type: |_| Token::KeywordReturn },
        Rule { regex: Regex::new(r"^\bif\b").unwrap(),     token_type: |_| Token::KeywordIf },
        Rule { regex: Regex::new(r"^\belse\b").unwrap(),   token_type: |_| Token::KeywordElse },
        Rule { regex: Regex::new(r"^\bwhile\b").unwrap(),  token_type: |_| Token::KeywordWhile },
        Rule { regex: Regex::new(r"^\bfor\b").unwrap(),    token_type: |_| Token::KeywordFor },
        Rule { regex: Regex::new(r"^\bstruct\b").unwrap(), token_type: |_| Token::KeywordStruct },

        // ===== Literals =====
        Rule {
            regex: Regex::new(r#"^"([^"\\]|\\.)*""#).unwrap(),
            token_type: |s| Token::StringLit(s.to_string()),
        },
        Rule {
            regex: Regex::new(r"^'([^'\\]|\\.)'").unwrap(),
            token_type: |s| Token::CharLit(s.chars().nth(1).unwrap()),
        },
        Rule {
            regex: Regex::new(r"^\d+\.\d+").unwrap(),
            token_type: |s| Token::Float(s.parse::<f64>().unwrap()),
        },
        Rule {
            regex: Regex::new(r"^\d+").unwrap(),
            token_type: |s| Token::Int(s.parse::<i64>().unwrap()),
        },
        Rule {
            regex: Regex::new(r"^[a-zA-Z_]\w*").unwrap(),
            token_type: |s| Token::Identifier(s.to_string()),
        },

        // ===== Operators =====
        Rule { regex: Regex::new(r"^==").unwrap(), token_type: |_| Token::EqEq },
        Rule { regex: Regex::new(r"^!=").unwrap(), token_type: |_| Token::NotEq },
        Rule { regex: Regex::new(r"^<=").unwrap(), token_type: |_| Token::LtEq },
        Rule { regex: Regex::new(r"^>=").unwrap(), token_type: |_| Token::GtEq },
        Rule { regex: Regex::new(r"^&&").unwrap(), token_type: |_| Token::AndAnd },
        Rule { regex: Regex::new(r"^\|\|").unwrap(), token_type: |_| Token::OrOr },
        Rule { regex: Regex::new(r"^<<").unwrap(), token_type: |_| Token::Shl },
        Rule { regex: Regex::new(r"^>>").unwrap(), token_type: |_| Token::Shr },
        Rule { regex: Regex::new(r"^\+").unwrap(), token_type: |_| Token::Plus },
        Rule { regex: Regex::new(r"^-").unwrap(), token_type: |_| Token::Minus },
        Rule { regex: Regex::new(r"^\*").unwrap(), token_type: |_| Token::Star },
        Rule { regex: Regex::new(r"^/").unwrap(), token_type: |_| Token::Slash },
        Rule { regex: Regex::new(r"^%").unwrap(), token_type: |_| Token::Percent },
        Rule { regex: Regex::new(r"^=").unwrap(), token_type: |_| Token::Assign },
        Rule { regex: Regex::new(r"^<").unwrap(), token_type: |_| Token::Lt },
        Rule { regex: Regex::new(r"^>").unwrap(), token_type: |_| Token::Gt },
        Rule { regex: Regex::new(r"^!").unwrap(), token_type: |_| Token::Not },
        Rule { regex: Regex::new(r"^&").unwrap(), token_type: |_| Token::Amp },
        Rule { regex: Regex::new(r"^\|").unwrap(), token_type: |_| Token::Pipe },
        Rule { regex: Regex::new(r"^\^").unwrap(), token_type: |_| Token::Caret },

        // ===== Symbols =====
        Rule { regex: Regex::new(r"^\(").unwrap(), token_type: |_| Token::ParenL },
        Rule { regex: Regex::new(r"^\)").unwrap(), token_type: |_| Token::ParenR },
        Rule { regex: Regex::new(r"^\{").unwrap(), token_type: |_| Token::BraceL },
        Rule { regex: Regex::new(r"^\}").unwrap(), token_type: |_| Token::BraceR },
        Rule { regex: Regex::new(r"^\[").unwrap(), token_type: |_| Token::BracketL },
        Rule { regex: Regex::new(r"^\]").unwrap(), token_type: |_| Token::BracketR },
        Rule { regex: Regex::new(r"^;").unwrap(),  token_type: |_| Token::Semicolon },
        Rule { regex: Regex::new(r"^,").unwrap(),  token_type: |_| Token::Comma },

        // ===== Comments =====
        Rule {
            regex: Regex::new(r"^//.*").unwrap(),
            token_type: |s| Token::Comment(s.to_string()),
        },
        Rule {
            regex: Regex::new(r"^/\*.*?\*/").unwrap(),
            token_type: |s| Token::Comment(s.to_string()),
        },
    ];
}
