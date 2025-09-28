// mod.rs: Implements the parser logic and entry points for MiniC source code.
pub mod ast;
use ast::*;
use crate::lexer_regex::Token;

pub struct Parser {
    pub tokens: Vec<Token>,
    pub pos: usize,
}

impl Parser {
    pub fn new(tokens: Vec<Token>) -> Self {
        Parser { tokens, pos: 0 }
    }

    pub fn parse_translation_unit(&mut self) -> Result<TranslationUnit, ParseError> {
        let mut preprocessor_list = Vec::new();
        let mut external_declarations = Vec::new();

        while self.pos < self.tokens.len() {
            match &self.tokens[self.pos] {
                Token::Comment(_comment) => {
                    // Comments are skipped in AST as per grammar
                    self.pos += 1;
                }
                Token::Preprocessor(directive_type) => {
                    let directive_type = directive_type.strip_prefix('#').unwrap_or(directive_type).to_string();
                    match self.parse_preprocessor_directive(&directive_type) {
                        Ok(directive) => preprocessor_list.push(directive),
                        Err(e) => return Err(e),
                    }
                }
                Token::Error(error_msg) => {
                    // Return parse error for lexer errors
                    return Err(ParseError::UnexpectedToken(format!("Lexer error: {}", error_msg)));
                }
                _ => {
                    if let Some(decl) = self.parse_external_declaration() {
                        external_declarations.push(decl);
                    } else {
                        // Check for specific error patterns when parsing fails
                        if let Err(e) = self.check_for_specific_errors() {
                            return Err(e);
                        }
                        
                        // Check if this is a parse error (missing expression, etc.)
                        // Commented out to prevent false positives on valid C code
                        /*
                        if self.pos < self.tokens.len() {
                            match &self.tokens[self.pos] {
                                Token::AssignOp => {
                                    // Missing expression after assignment operator
                                    return Err(ParseError::ExpectedExpr);
                                }
                                Token::Semicolon => {
                                    // Unexpected semicolon - might be missing expression
                                    return Err(ParseError::UnexpectedToken("Semicolon".to_string()));
                                }
                                _ => {
                                    // Skip unrecognized token and continue parsing
                                    self.pos += 1;
                                }
                            }
                        } else {
                            // End of tokens - check for UnexpectedEOF
                            return Err(ParseError::UnexpectedEOF);
                        }
                        */
                        
                        // Skip unrecognized token and continue parsing
                        if self.pos < self.tokens.len() {
                            self.pos += 1;
                        } else {
                            // End of tokens
                            break;
                        }
                    }
                }
            }
        }

        Ok(TranslationUnit {
            preprocessor_list,
            external_declarations,
        })
    }

    fn parse_preprocessor_directive(&mut self, directive_type: &str) -> Result<PreprocessorDirective, ParseError> {
        self.pos += 1; // Skip the preprocessor token
        match directive_type {
            "include" => {
                if self.pos < self.tokens.len() {
                    match &self.tokens[self.pos] {
                        Token::StringLit(s) => {
                            self.pos += 1;
                            Ok(PreprocessorDirective::Include(s.clone()))
                        }
                        Token::LessOp => {
                            // Handle #include <header.h>
                            self.pos += 1; // Skip <
                            let mut header = String::new();
                            while self.pos < self.tokens.len() && self.tokens[self.pos] != Token::GreaterOp {
                                match &self.tokens[self.pos] {
                                    Token::Identifier(id) => {
                                        header.push_str(id);
                                        self.pos += 1;
                                    }
                                    Token::Dot => {
                                        header.push('.');
                                        self.pos += 1;
                                    }
                                    _ => {
                                        self.pos += 1;
                                    }
                                }
                            }
                            if self.pos < self.tokens.len() && self.tokens[self.pos] == Token::GreaterOp {
                                self.pos += 1; // Skip >
                            }
                            Ok(PreprocessorDirective::Include(header))
                        }
                        _ => Err(ParseError::UnexpectedToken(format!("{:?}", self.tokens[self.pos]))),
                    }
                } else {
                    Err(ParseError::UnexpectedEOF)
                }
            }
            "define" => {
                if self.pos < self.tokens.len() {
                    match &self.tokens[self.pos] {
                        Token::Identifier(id) => {
                            let ident = id.clone();
                            self.pos += 1;
                            let replacement_list = self.parse_replacement_list();
                            Ok(PreprocessorDirective::Define(ident, replacement_list))
                        }
                        _ => Err(ParseError::ExpectedIdentifier),
                    }
                } else {
                    Err(ParseError::UnexpectedEOF)
                }
            }
            "ifdef" => {
                if self.pos < self.tokens.len() {
                    match &self.tokens[self.pos] {
                        Token::Identifier(id) => {
                            self.pos += 1;
                            Ok(PreprocessorDirective::Ifdef(id.clone()))
                        }
                        _ => Err(ParseError::ExpectedIdentifier),
                    }
                } else {
                    Err(ParseError::UnexpectedEOF)
                }
            }
            "ifndef" => {
                if self.pos < self.tokens.len() {
                    match &self.tokens[self.pos] {
                        Token::Identifier(id) => {
                            self.pos += 1;
                            Ok(PreprocessorDirective::Ifndef(id.clone()))
                        }
                        _ => Err(ParseError::ExpectedIdentifier),
                    }
                } else {
                    Err(ParseError::UnexpectedEOF)
                }
            }
            "endif" => {
                Ok(PreprocessorDirective::Endif)
            }
            _ => Err(ParseError::UnexpectedToken(format!("{:?}", self.tokens[self.pos]))),
        }
    }

    fn parse_replacement_list(&mut self) -> Vec<ReplacementItem> {
        let mut items = Vec::new();
        while self.pos < self.tokens.len() {
            match &self.tokens[self.pos] {
                Token::Identifier(id) => {
                    items.push(ReplacementItem::Identifier(id.clone()));
                    self.pos += 1;
                }
                Token::IntLit(n) => {
                    items.push(ReplacementItem::Constant(Constant::Integer(*n)));
                    self.pos += 1;
                }
                Token::FloatLit(f) => {
                    items.push(ReplacementItem::Constant(Constant::Float(*f)));
                    self.pos += 1;
                }
                Token::StringLit(s) => {
                    items.push(ReplacementItem::StringLiteral(s.clone()));
                    self.pos += 1;
                }
                _ => break,
            }
        }
        items
    }

    fn parse_external_declaration(&mut self) -> Option<ExternalDeclaration> {
        if self.pos >= self.tokens.len() {
            return None;
        }

        // Skip preprocessor directives, comments, and error tokens
        while self.pos < self.tokens.len() {
            match &self.tokens[self.pos] {
                Token::Preprocessor(_) | Token::Comment(_) | Token::BlockComment(_) | Token::Error(_) => {
                    self.pos += 1;
                    continue;
                }
                _ => break,
            }
        }
        
        if self.pos >= self.tokens.len() {
            return None;
        }

        
        match &self.tokens[self.pos] {
            Token::Identifier(id) if id == "printf" => {
                self.pos += 1;
                if let Some(printf) = self.parse_printf_statement() {
                    Some(ExternalDeclaration::Printf(printf))
                } else {
                    None
                }
            }
            Token::Struct => {
                self.pos += 1;
                if let Some(struct_decl) = self.parse_struct_declaration() {
                    Some(ExternalDeclaration::Struct(struct_decl))
                } else {
                    None
                }
            }
            Token::Typedef => {
                self.pos += 1;
                if let Some(typedef_decl) = self.parse_typedef_declaration() {
                    Some(ExternalDeclaration::Typedef(typedef_decl))
                } else {
                    None
                }
            }
            Token::Enum => {
                self.pos += 1;
                if let Some(enum_decl) = self.parse_enum_declaration() {
                    Some(ExternalDeclaration::Enum(enum_decl))
                } else {
                    None
                }
            }
            Token::Static => {
                self.pos += 1;
                if let Some(mut var_decl) = self.parse_variable_declaration() {
                    var_decl.storage_class = Some(StorageClass::Static);
                    Some(ExternalDeclaration::Variable(var_decl))
                } else {
                    None
                }
            }
            Token::Const => {
                self.pos += 1;
                if let Some(mut var_decl) = self.parse_variable_declaration() {
                    var_decl.type_qualifiers.push(TypeQualifier::Const);
                    Some(ExternalDeclaration::Variable(var_decl))
                } else {
                    None
                }
            }
            // Handle function definitions: int function_name(...) { ... }
            Token::Int | Token::Void | Token::Char | Token::Float | Token::Double | Token::Long | Token::Short => {
                if let Some(func) = self.parse_function_definition() {
                    Some(ExternalDeclaration::Function(func))
                } else {
                    None
                }
            }
            _ => None,
        }
    }

  fn parse_typedef_declaration(&mut self) -> Option<TypedefDeclaration> {
        // Simple typedef parsing: typedef int myint;
        if self.pos >= self.tokens.len() {
            return None;
        }
        
        let type_specifier = match &self.tokens[self.pos] {
            Token::Int => {
                self.pos += 1;
                TypeSpecifier::Int
            }
            Token::Float => {
                self.pos += 1;
                TypeSpecifier::Float
            }
            Token::Char => {
                self.pos += 1;
                TypeSpecifier::Char
            }
            Token::Double => {
                self.pos += 1;
                TypeSpecifier::Double
            }
            Token::Void => {
                self.pos += 1;
                TypeSpecifier::Void
            }
            _ => return None,
        };
        
        if self.pos >= self.tokens.len() {
            return None;
        }
        
        let name = match &self.tokens[self.pos] {
            Token::Identifier(id) => {
                self.pos += 1;
                id.clone()
            }
            _ => return None,
        };
        
        if self.pos < self.tokens.len() && self.tokens[self.pos] == Token::Semicolon {
            self.pos += 1;
        }
        
        Some(TypedefDeclaration {
            type_specifier,
            declarator: Declarator {
                name,
                pointer_depth: 0,
                array_sizes: Vec::new(),
                function_params: None,
            },
        })
    }
    
    fn parse_enum_declaration(&mut self) -> Option<EnumDeclaration> {
        // Simple enum parsing: enum Color { RED, GREEN, BLUE };
        if self.pos >= self.tokens.len() {
            return None;
        }
        
        let name = match &self.tokens[self.pos] {
            Token::Identifier(id) => {
                self.pos += 1;
                Some(id.clone())
            }
            _ => None,
        };
        
        if self.pos >= self.tokens.len() || self.tokens[self.pos] != Token::BraceL {
            return None;
        }
        self.pos += 1;
        
        let mut enumerators = Vec::new();
        
        while self.pos < self.tokens.len() && self.tokens[self.pos] != Token::BraceR {
            if let Token::Identifier(id) = &self.tokens[self.pos] {
                self.pos += 1;
                enumerators.push(Enumerator {
                    name: id.clone(),
                    value: None,
                });
                
                if self.pos < self.tokens.len() && self.tokens[self.pos] == Token::Comma {
                    self.pos += 1;
                }
            } else {
                break;
            }
        }
        
        if self.pos < self.tokens.len() && self.tokens[self.pos] == Token::BraceR {
            self.pos += 1;
        }
        
        if self.pos < self.tokens.len() && self.tokens[self.pos] == Token::Semicolon {
            self.pos += 1;
        }
        
        Some(EnumDeclaration {
            name,
            enumerators,
        })
    }
    
    fn parse_variable_declaration(&mut self) -> Option<VariableDeclaration> {
        // Simple variable parsing: static int global_var = 10; or const double global_const = PI;
        if self.pos >= self.tokens.len() {
            return None;
        }
        
        let type_specifier = match &self.tokens[self.pos] {
            Token::Int => {
                self.pos += 1;
                TypeSpecifier::Int
            }
            Token::Float => {
                self.pos += 1;
                TypeSpecifier::Float
            }
            Token::Char => {
                self.pos += 1;
                TypeSpecifier::Char
            }
            Token::Double => {
                self.pos += 1;
                TypeSpecifier::Double
            }
            Token::Void => {
                self.pos += 1;
                TypeSpecifier::Void
            }
            _ => return None,
        };
        
        if self.pos >= self.tokens.len() {
            return None;
        }
        
        let name = match &self.tokens[self.pos] {
            Token::Identifier(id) => {
                self.pos += 1;
                id.clone()
            }
            _ => return None,
        };
        
        let mut initializer = None;
        if self.pos < self.tokens.len() && self.tokens[self.pos] == Token::AssignOp {
            self.pos += 1;
            // Skip the initializer value for now
            while self.pos < self.tokens.len() && self.tokens[self.pos] != Token::Semicolon {
                self.pos += 1;
            }
        }
        
        if self.pos < self.tokens.len() && self.tokens[self.pos] == Token::Semicolon {
            self.pos += 1;
        }
        
        Some(VariableDeclaration {
            storage_class: None, // Will be set by caller
            type_qualifiers: Vec::new(),
            type_specifier,
            declarator: Declarator {
                name,
                pointer_depth: 0,
                array_sizes: Vec::new(),
                function_params: None,
            },
            initializer,
        })
    }

   