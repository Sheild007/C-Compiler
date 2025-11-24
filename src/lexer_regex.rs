

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
    Colon,
    Plus,
    Minus,
    Mult,
    Div,
    Mod,
    Xor,
    Not,
    Question,
    Dot,
    Arrow,
    PlusPlus,
    MinusMinus,
    PlusAssign,
    MinusAssign,
    MultAssign,
    DivAssign,
    ModAssign,
    LShiftAssign,
    RShiftAssign,
    AndAssign,
    XorAssign,
    OrAssign,
    LShift,
    RShift,
    Hash,
    Comment(String),
    BlockComment(String),
    Preprocessor(String),
    Enum,
    Struct,
    Typedef,
    Static,
    Const,
    Volatile,
    Extern,
    Auto,
    Register,
    Case,
    Default,
    Break,
    Continue,
    Goto,
    Switch,
    Do,
    Union,
    Signed,
    Unsigned,
    Short,
    Long,
    Double,
    Char,
    Void,
    Error(String),
}

pub fn lex_with_regex(input: &str) -> Vec<Token> {
    let mut tokens = Vec::new();
    let re = Regex::new(
        r#"(?P<ws>\s+)|(?P<comment>//.*)|(?P<blockcomment>/\*.*?\*/)|(?P<preprocessor>#[a-zA-Z_][a-zA-Z0-9_]*)|(?P<function>fn)\b|(?P<return>return)\b|(?P<if>if)\b|(?P<else>else)\b|(?P<while>while)\b|(?P<for>for)\b|(?P<int>int)\b|(?P<float>float)\b|(?P<string>string)\b|(?P<bool>bool)\b|(?P<enum>enum)\b|(?P<struct>struct)\b|(?P<typedef>typedef)\b|(?P<static>static)\b|(?P<const>const)\b|(?P<volatile>volatile)\b|(?P<extern>extern)\b|(?P<auto>auto)\b|(?P<register>register)\b|(?P<case>case)\b|(?P<default>default)\b|(?P<break>break)\b|(?P<continue>continue)\b|(?P<goto>goto)\b|(?P<switch>switch)\b|(?P<do>do)\b|(?P<union>union)\b|(?P<signed>signed)\b|(?P<unsigned>unsigned)\b|(?P<short>short)\b|(?P<long>long)\b|(?P<double>double)\b|(?P<char>char)\b|(?P<void>void)\b|(?P<floatlit>\d+\.\d+)|(?P<intlit>\d+)|(?P<stringlit>"([^\\"]|\\.)*")|(?P<equalsop>==)|(?P<notequalsop>!=)|(?P<lesseqop><=)|(?P<greatereqop>>=)|(?P<andop>&&)|(?P<orop>\|\|)|(?P<assignop>=)|(?P<lshiftop><<)|(?P<rshiftop>>{2})|(?P<lessop><)|(?P<greaterop>>)|(?P<bitandop>&)|(?P<bitorop>\|)|(?P<plusop>\+)|(?P<minusop>-)|(?P<multop>\*)|(?P<divop>/)|(?P<modop>%)|(?P<xorop>\^)|(?P<notop>~)|(?P<questionop>\?)|(?P<dotop>\.)|(?P<arrowop>->)|(?P<plusplusop>\+\+)|(?P<minusminusop>--)|(?P<plusassignop>\+=)|(?P<minusassignop>-=)|(?P<multassignop>\*=)|(?P<divassignop>/=)|(?P<modassignop>%=)|(?P<lshiftassignop><<=)|(?P<rshiftassignop>>=)|(?P<andassignop>&=)|(?P<xorassignop>\^=)|(?P<orassignop>\|=)|(?P<hashop>#)|(?P<identifier>[a-zA-Z_][a-zA-Z0-9_]*)|(?P<parenl>\()|(?P<parenr>\))|(?P<bracel>\{)|(?P<bracer>\})|(?P<bracketl>\[)|(?P<bracketr>\])|(?P<comma>,)|(?P<semicolon>;)|(?P<colon>:)|(?P<quotes>")"#
    ).unwrap();
    let mut pos = 0;
    while pos < input.len() {
        if let Some(m) = re.find(&input[pos..]) {
            let s = &input[pos + m.start()..pos + m.end()];
            let caps = re.captures(s).unwrap();
            if caps.name("ws").is_some() {
                // skip whitespace
                pos += m.end();
                continue;
            } else if let Some(_) = caps.name("comment") {
                tokens.push(Token::Comment(s.to_string()));
            } else if let Some(_) = caps.name("blockcomment") {
                tokens.push(Token::BlockComment(s.to_string()));
            } else if let Some(pp) = caps.name("preprocessor") {
                tokens.push(Token::Preprocessor(pp.as_str().to_string()));
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
            } else if let Some(_) = caps.name("enum") {
                tokens.push(Token::Enum);
            } else if let Some(_) = caps.name("struct") {
                tokens.push(Token::Struct);
            } else if let Some(_) = caps.name("typedef") {
                tokens.push(Token::Typedef);
            } else if let Some(_) = caps.name("static") {
                tokens.push(Token::Static);
            } else if let Some(_) = caps.name("const") {
                tokens.push(Token::Const);
            } else if let Some(_) = caps.name("volatile") {
                tokens.push(Token::Volatile);
            } else if let Some(_) = caps.name("extern") {
                tokens.push(Token::Extern);
            } else if let Some(_) = caps.name("auto") {
                tokens.push(Token::Auto);
            } else if let Some(_) = caps.name("register") {
                tokens.push(Token::Register);
            } else if let Some(_) = caps.name("case") {
                tokens.push(Token::Case);
            } else if let Some(_) = caps.name("default") {
                tokens.push(Token::Default);
            } else if let Some(_) = caps.name("break") {
                tokens.push(Token::Break);
            } else if let Some(_) = caps.name("continue") {
                tokens.push(Token::Continue);
            } else if let Some(_) = caps.name("goto") {
                tokens.push(Token::Goto);
            } else if let Some(_) = caps.name("switch") {
                tokens.push(Token::Switch);
            } else if let Some(_) = caps.name("do") {
                tokens.push(Token::Do);
            } else if let Some(_) = caps.name("union") {
                tokens.push(Token::Union);
            } else if let Some(_) = caps.name("signed") {
                tokens.push(Token::Signed);
            } else if let Some(_) = caps.name("unsigned") {
                tokens.push(Token::Unsigned);
            } else if let Some(_) = caps.name("short") {
                tokens.push(Token::Short);
            } else if let Some(_) = caps.name("long") {
                tokens.push(Token::Long);
            } else if let Some(_) = caps.name("double") {
                tokens.push(Token::Double);
            } else if let Some(_) = caps.name("char") {
                tokens.push(Token::Char);
            } else if let Some(_) = caps.name("void") {
                tokens.push(Token::Void);
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
            } else if let Some(_) = caps.name("colon") {
                tokens.push(Token::Colon);
            } else if let Some(_) = caps.name("plusop") {
                tokens.push(Token::Plus);
            } else if let Some(_) = caps.name("minusop") {
                tokens.push(Token::Minus);
            } else if let Some(_) = caps.name("multop") {
                tokens.push(Token::Mult);
            } else if let Some(_) = caps.name("divop") {
                tokens.push(Token::Div);
            } else if let Some(_) = caps.name("modop") {
                tokens.push(Token::Mod);
            } else if let Some(_) = caps.name("xorop") {
                tokens.push(Token::Xor);
            } else if let Some(_) = caps.name("notop") {
                tokens.push(Token::Not);
            } else if let Some(_) = caps.name("questionop") {
                tokens.push(Token::Question);
            } else if let Some(_) = caps.name("dotop") {
                tokens.push(Token::Dot);
            } else if let Some(_) = caps.name("arrowop") {
                tokens.push(Token::Arrow);
            } else if let Some(_) = caps.name("plusplusop") {
                tokens.push(Token::PlusPlus);
            } else if let Some(_) = caps.name("minusminusop") {
                tokens.push(Token::MinusMinus);
            } else if let Some(_) = caps.name("plusassignop") {
                tokens.push(Token::PlusAssign);
            } else if let Some(_) = caps.name("minusassignop") {
                tokens.push(Token::MinusAssign);
            } else if let Some(_) = caps.name("multassignop") {
                tokens.push(Token::MultAssign);
            } else if let Some(_) = caps.name("divassignop") {
                tokens.push(Token::DivAssign);
            } else if let Some(_) = caps.name("modassignop") {
                tokens.push(Token::ModAssign);
            } else if let Some(_) = caps.name("lshiftassignop") {
                tokens.push(Token::LShiftAssign);
            } else if let Some(_) = caps.name("rshiftassignop") {
                tokens.push(Token::RShiftAssign);
            } else if let Some(_) = caps.name("andassignop") {
                tokens.push(Token::AndAssign);
            } else if let Some(_) = caps.name("xorassignop") {
                tokens.push(Token::XorAssign);
            } else if let Some(_) = caps.name("orassignop") {
                tokens.push(Token::OrAssign);
            } else if let Some(_) = caps.name("lshiftop") {
                tokens.push(Token::LShift);
            } else if let Some(_) = caps.name("rshiftop") {
                tokens.push(Token::RShift);
            } else if let Some(_) = caps.name("hashop") {
                tokens.push(Token::Hash);
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
