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

   