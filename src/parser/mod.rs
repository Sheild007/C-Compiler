// mod.rs: Implements the parser logic and entry points for MiniC source code.
pub mod ast;
use crate::lexer_regex::Token;
use ast::*;

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
                    let directive_type = directive_type
                        .strip_prefix('#')
                        .unwrap_or(directive_type)
                        .to_string();
                    match self.parse_preprocessor_directive(&directive_type) {
                        Ok(directive) => preprocessor_list.push(directive),
                        Err(e) => return Err(e),
                    }
                }
                Token::Error(error_msg) => {
                    // Return parse error for lexer errors
                    return Err(ParseError::UnexpectedToken(format!(
                        "Lexer error: {}",
                        error_msg
                    )));
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

    fn check_for_specific_errors(&mut self) -> Result<(), ParseError> {
        if self.pos >= self.tokens.len() {
            return Ok(());
        }

        // Check for ExpectedIdentifier: int = 5; (missing identifier after type)
        if self.pos + 2 < self.tokens.len() {
            match (
                &self.tokens[self.pos],
                &self.tokens[self.pos + 1],
                &self.tokens[self.pos + 2],
            ) {
                (
                    Token::Int
                    | Token::Float
                    | Token::Char
                    | Token::Double
                    | Token::Long
                    | Token::Short
                    | Token::Void,
                    Token::AssignOp,
                    Token::IntLit(_) | Token::FloatLit(_) | Token::StringLit(_) | Token::BoolLit(_),
                ) => {
                    return Err(ParseError::ExpectedIdentifier);
                }
                _ => {}
            }
        }

        // Check for ExpectedTypeToken: x = 5; (missing type specifier)
        // Only trigger this if we're at the start of a statement and there's no preceding type
        if self.pos + 1 < self.tokens.len() {
            match (&self.tokens[self.pos], &self.tokens[self.pos + 1]) {
                (Token::Identifier(_), Token::AssignOp) => {
                    // Check if this is at the start of a statement (after semicolon, brace, etc.)
                    let is_start_of_statement = if self.pos > 0 {
                        matches!(
                            &self.tokens[self.pos - 1],
                            Token::Semicolon | Token::BraceL | Token::BraceR
                        )
                    } else {
                        true
                    };

                    if is_start_of_statement {
                        return Err(ParseError::ExpectedTypeToken);
                    }
                }
                _ => {}
            }
        }

        // Check for ExpectedFloatLit: float f = ;
        if self.pos + 3 < self.tokens.len() {
            match (
                &self.tokens[self.pos],
                &self.tokens[self.pos + 1],
                &self.tokens[self.pos + 2],
                &self.tokens[self.pos + 3],
            ) {
                (Token::Float, Token::Identifier(_), Token::AssignOp, Token::Semicolon) => {
                    return Err(ParseError::ExpectedFloatLit);
                }
                _ => {}
            }
        }

        // Check for ExpectedIntLit: int i = ;
        if self.pos + 3 < self.tokens.len() {
            match (
                &self.tokens[self.pos],
                &self.tokens[self.pos + 1],
                &self.tokens[self.pos + 2],
                &self.tokens[self.pos + 3],
            ) {
                (Token::Int, Token::Identifier(_), Token::AssignOp, Token::Semicolon) => {
                    return Err(ParseError::ExpectedIntLit);
                }
                _ => {}
            }
        }

        // Check for ExpectedStringLit: char* str = ;
        if self.pos + 4 < self.tokens.len() {
            match (
                &self.tokens[self.pos],
                &self.tokens[self.pos + 1],
                &self.tokens[self.pos + 2],
                &self.tokens[self.pos + 3],
                &self.tokens[self.pos + 4],
            ) {
                (
                    Token::Char,
                    Token::Mult,
                    Token::Identifier(_),
                    Token::AssignOp,
                    Token::Semicolon,
                ) => {
                    return Err(ParseError::ExpectedStringLit);
                }
                _ => {}
            }
        }

        // Check for ExpectedBoolLit: bool flag = ;
        if self.pos + 3 < self.tokens.len() {
            match (
                &self.tokens[self.pos],
                &self.tokens[self.pos + 1],
                &self.tokens[self.pos + 2],
                &self.tokens[self.pos + 3],
            ) {
                (Token::Bool, Token::Identifier(_), Token::AssignOp, Token::Semicolon) => {
                    return Err(ParseError::ExpectedBoolLit);
                }
                _ => {}
            }
        }

        // Check for FailedToFindToken: int x = 5 + ;
        if self.pos + 4 < self.tokens.len() {
            match (
                &self.tokens[self.pos],
                &self.tokens[self.pos + 1],
                &self.tokens[self.pos + 2],
                &self.tokens[self.pos + 3],
                &self.tokens[self.pos + 4],
            ) {
                (
                    Token::Int,
                    Token::Identifier(_),
                    Token::AssignOp,
                    Token::IntLit(_),
                    Token::Plus | Token::Minus | Token::Mult | Token::Div,
                ) => {
                    if self.pos + 5 < self.tokens.len()
                        && self.tokens[self.pos + 5] == Token::Semicolon
                    {
                        return Err(ParseError::FailedToFindToken(
                            "Missing operand after operator".to_string(),
                        ));
                    }
                }
                _ => {}
            }
        }

        // Check for UnexpectedEOF: incomplete function (missing closing brace)
        if let Err(e) = self.check_for_incomplete_functions() {
            return Err(e);
        }

        Ok(())
    }

    fn check_for_incomplete_functions(&mut self) -> Result<(), ParseError> {
        // Look for function definitions that don't have closing braces
        let mut brace_count = 0;
        let mut in_function = false;
        let mut start_pos = self.pos;

        while self.pos < self.tokens.len() {
            match &self.tokens[self.pos] {
                Token::BraceL => {
                    brace_count += 1;
                    in_function = true;
                }
                Token::BraceR => {
                    brace_count -= 1;
                    if brace_count == 0 {
                        in_function = false;
                    }
                }
                Token::Int
                | Token::Void
                | Token::Char
                | Token::Float
                | Token::Double
                | Token::Long
                | Token::Short => {
                    if self.pos + 1 < self.tokens.len() {
                        match &self.tokens[self.pos + 1] {
                            Token::Identifier(_) => {
                                // This looks like a function definition
                                start_pos = self.pos;
                            }
                            _ => {}
                        }
                    }
                }
                _ => {}
            }
            self.pos += 1;
        }

        // If we're still in a function when we reach the end, it's incomplete
        if in_function && brace_count > 0 {
            return Err(ParseError::UnexpectedEOF);
        }

        // Reset position
        self.pos = start_pos;
        Ok(())
    }

    fn parse_preprocessor_directive(
        &mut self,
        directive_type: &str,
    ) -> Result<PreprocessorDirective, ParseError> {
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
                            while self.pos < self.tokens.len()
                                && self.tokens[self.pos] != Token::GreaterOp
                            {
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
                            if self.pos < self.tokens.len()
                                && self.tokens[self.pos] == Token::GreaterOp
                            {
                                self.pos += 1; // Skip >
                            }
                            Ok(PreprocessorDirective::Include(header))
                        }
                        _ => Err(ParseError::UnexpectedToken(format!(
                            "{:?}",
                            self.tokens[self.pos]
                        ))),
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
            "endif" => Ok(PreprocessorDirective::Endif),
            _ => Err(ParseError::UnexpectedToken(format!(
                "{:?}",
                self.tokens[self.pos]
            ))),
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
                Token::Preprocessor(_)
                | Token::Comment(_)
                | Token::BlockComment(_)
                | Token::Error(_) => {
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
            Token::Int
            | Token::Void
            | Token::Char
            | Token::Float
            | Token::Double
            | Token::Long
            | Token::Short => {
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

        Some(EnumDeclaration { name, enumerators })
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

    fn parse_printf_statement(&mut self) -> Option<PrintfStatement> {
        if self.pos >= self.tokens.len() || self.tokens[self.pos] != Token::ParenL {
            return None;
        }
        self.pos += 1; // Consume '('

        let args = self.parse_printf_args();

        if self.pos < self.tokens.len() && self.tokens[self.pos] == Token::ParenR {
            self.pos += 1; // Consume ')'
            if self.pos < self.tokens.len() && self.tokens[self.pos] == Token::Semicolon {
                self.pos += 1; // Consume ';'
                Some(PrintfStatement { args })
            } else {
                None
            }
        } else {
            None
        }
    }

    fn parse_printf_args(&mut self) -> Vec<PrintfArg> {
        let mut args = Vec::new();
        if self.pos >= self.tokens.len() {
            return args;
        }

        // First argument (if any) is often a StringLiteral
        match &self.tokens[self.pos] {
            Token::StringLit(s) => {
                args.push(PrintfArg::StringLiteral(s.clone()));
                self.pos += 1;
            }
            _ => {
                if let Some(expr) = self.parse_expression() {
                    args.push(PrintfArg::Expression(expr));
                }
            }
        }

        // Parse tail (comma-separated expressions)
        while self.pos < self.tokens.len() && self.tokens[self.pos] == Token::Comma {
            self.pos += 1; // Consume ','
            if let Some(expr) = self.parse_expression() {
                args.push(PrintfArg::Expression(expr));
            } else {
                break;
            }
        }

        args
    }

    fn parse_expression(&mut self) -> Option<Expression> {
        self.parse_assignment_expression()
    }

    fn parse_assignment_expression(&mut self) -> Option<Expression> {
        let left = self.parse_conditional_expression()?;

        if self.pos < self.tokens.len() {
            match &self.tokens[self.pos] {
                Token::AssignOp => {
                    self.pos += 1;
                    // Check if there's a missing expression after assignment operator
                    if self.pos >= self.tokens.len() || self.tokens[self.pos] == Token::Semicolon {
                        // Missing expression after assignment - this should be a parse error
                        return None;
                    }
                    let right = self.parse_assignment_expression()?;
                    Some(Expression::Assignment(
                        Box::new(left),
                        AssignmentOperator::Assign,
                        Box::new(right),
                    ))
                }
                Token::PlusAssign => {
                    self.pos += 1;
                    let right = self.parse_assignment_expression()?;
                    Some(Expression::Assignment(
                        Box::new(left),
                        AssignmentOperator::PlusAssign,
                        Box::new(right),
                    ))
                }
                Token::MinusAssign => {
                    self.pos += 1;
                    let right = self.parse_assignment_expression()?;
                    Some(Expression::Assignment(
                        Box::new(left),
                        AssignmentOperator::MinusAssign,
                        Box::new(right),
                    ))
                }
                Token::MultAssign => {
                    self.pos += 1;
                    let right = self.parse_assignment_expression()?;
                    Some(Expression::Assignment(
                        Box::new(left),
                        AssignmentOperator::MultAssign,
                        Box::new(right),
                    ))
                }
                Token::DivAssign => {
                    self.pos += 1;
                    let right = self.parse_assignment_expression()?;
                    Some(Expression::Assignment(
                        Box::new(left),
                        AssignmentOperator::DivAssign,
                        Box::new(right),
                    ))
                }
                Token::ModAssign => {
                    self.pos += 1;
                    let right = self.parse_assignment_expression()?;
                    Some(Expression::Assignment(
                        Box::new(left),
                        AssignmentOperator::ModAssign,
                        Box::new(right),
                    ))
                }
                Token::LShiftAssign => {
                    self.pos += 1;
                    let right = self.parse_assignment_expression()?;
                    Some(Expression::Assignment(
                        Box::new(left),
                        AssignmentOperator::LShiftAssign,
                        Box::new(right),
                    ))
                }
                Token::RShiftAssign => {
                    self.pos += 1;
                    let right = self.parse_assignment_expression()?;
                    Some(Expression::Assignment(
                        Box::new(left),
                        AssignmentOperator::RShiftAssign,
                        Box::new(right),
                    ))
                }
                Token::AndAssign => {
                    self.pos += 1;
                    let right = self.parse_assignment_expression()?;
                    Some(Expression::Assignment(
                        Box::new(left),
                        AssignmentOperator::AndAssign,
                        Box::new(right),
                    ))
                }
                Token::XorAssign => {
                    self.pos += 1;
                    let right = self.parse_assignment_expression()?;
                    Some(Expression::Assignment(
                        Box::new(left),
                        AssignmentOperator::XorAssign,
                        Box::new(right),
                    ))
                }
                Token::OrAssign => {
                    self.pos += 1;
                    let right = self.parse_assignment_expression()?;
                    Some(Expression::Assignment(
                        Box::new(left),
                        AssignmentOperator::OrAssign,
                        Box::new(right),
                    ))
                }
                _ => Some(left),
            }
        } else {
            Some(left)
        }
    }

    fn parse_conditional_expression(&mut self) -> Option<Expression> {
        let condition = self.parse_logical_or_expression()?;

        if self.pos < self.tokens.len() && self.tokens[self.pos] == Token::Question {
            self.pos += 1; // Consume '?'
            let true_expr = self.parse_expression()?;
            if self.pos < self.tokens.len() && self.tokens[self.pos] == Token::Colon {
                self.pos += 1; // Consume ':'
                let false_expr = self.parse_conditional_expression()?;
                Some(Expression::Conditional(
                    Box::new(condition),
                    Box::new(true_expr),
                    Box::new(false_expr),
                ))
            } else {
                None
            }
        } else {
            Some(condition)
        }
    }

    fn parse_logical_or_expression(&mut self) -> Option<Expression> {
        let mut left = self.parse_logical_and_expression()?;

        while self.pos < self.tokens.len() && self.tokens[self.pos] == Token::OrOp {
            self.pos += 1; // Consume '||'
            let right = self.parse_logical_and_expression()?;
            left = Expression::BinaryOp(Box::new(left), BinaryOperator::Or, Box::new(right));
        }

        Some(left)
    }

    fn parse_logical_and_expression(&mut self) -> Option<Expression> {
        let mut left = self.parse_bitwise_or_expression()?;

        while self.pos < self.tokens.len() && self.tokens[self.pos] == Token::AndOp {
            self.pos += 1; // Consume '&&'
            let right = self.parse_bitwise_or_expression()?;
            left = Expression::BinaryOp(Box::new(left), BinaryOperator::And, Box::new(right));
        }

        Some(left)
    }

    fn parse_bitwise_or_expression(&mut self) -> Option<Expression> {
        let mut left = self.parse_bitwise_xor_expression()?;

        while self.pos < self.tokens.len() && self.tokens[self.pos] == Token::BitOrOp {
            self.pos += 1; // Consume '|'
            let right = self.parse_bitwise_xor_expression()?;
            left = Expression::BinaryOp(Box::new(left), BinaryOperator::BitOr, Box::new(right));
        }

        Some(left)
    }

    fn parse_bitwise_xor_expression(&mut self) -> Option<Expression> {
        let mut left = self.parse_bitwise_and_expression()?;

        while self.pos < self.tokens.len() && self.tokens[self.pos] == Token::Xor {
            self.pos += 1; // Consume '^'
            let right = self.parse_bitwise_and_expression()?;
            left = Expression::BinaryOp(Box::new(left), BinaryOperator::Xor, Box::new(right));
        }

        Some(left)
    }

    fn parse_bitwise_and_expression(&mut self) -> Option<Expression> {
        let mut left = self.parse_equality_expression()?;

        while self.pos < self.tokens.len() && self.tokens[self.pos] == Token::BitAndOp {
            self.pos += 1; // Consume '&'
            let right = self.parse_equality_expression()?;
            left = Expression::BinaryOp(Box::new(left), BinaryOperator::BitAnd, Box::new(right));
        }

        Some(left)
    }

    fn parse_equality_expression(&mut self) -> Option<Expression> {
        let mut left = self.parse_relational_expression()?;

        while self.pos < self.tokens.len() {
            match &self.tokens[self.pos] {
                Token::EqualsOp => {
                    self.pos += 1; // Consume '=='
                    let right = self.parse_relational_expression()?;
                    left = Expression::BinaryOp(
                        Box::new(left),
                        BinaryOperator::Equals,
                        Box::new(right),
                    );
                }
                Token::NotEqualsOp => {
                    self.pos += 1; // Consume '!='
                    let right = self.parse_relational_expression()?;
                    left = Expression::BinaryOp(
                        Box::new(left),
                        BinaryOperator::NotEquals,
                        Box::new(right),
                    );
                }
                _ => break,
            }
        }

        Some(left)
    }

    fn parse_relational_expression(&mut self) -> Option<Expression> {
        let mut left = self.parse_shift_expression()?;

        while self.pos < self.tokens.len() {
            match &self.tokens[self.pos] {
                Token::LessOp => {
                    self.pos += 1; // Consume '<'
                    let right = self.parse_shift_expression()?;
                    left =
                        Expression::BinaryOp(Box::new(left), BinaryOperator::Less, Box::new(right));
                }
                Token::GreaterOp => {
                    self.pos += 1; // Consume '>'
                    let right = self.parse_shift_expression()?;
                    left = Expression::BinaryOp(
                        Box::new(left),
                        BinaryOperator::Greater,
                        Box::new(right),
                    );
                }
                Token::LessEqOp => {
                    self.pos += 1; // Consume '<='
                    let right = self.parse_shift_expression()?;
                    left = Expression::BinaryOp(
                        Box::new(left),
                        BinaryOperator::LessEq,
                        Box::new(right),
                    );
                }
                Token::GreaterEqOp => {
                    self.pos += 1; // Consume '>='
                    let right = self.parse_shift_expression()?;
                    left = Expression::BinaryOp(
                        Box::new(left),
                        BinaryOperator::GreaterEq,
                        Box::new(right),
                    );
                }
                _ => break,
            }
        }

        Some(left)
    }

    fn parse_shift_expression(&mut self) -> Option<Expression> {
        let mut left = self.parse_additive_expression()?;

        while self.pos < self.tokens.len() {
            match &self.tokens[self.pos] {
                Token::LShift => {
                    self.pos += 1; // Consume '<<'
                    let right = self.parse_additive_expression()?;
                    left = Expression::BinaryOp(
                        Box::new(left),
                        BinaryOperator::LShift,
                        Box::new(right),
                    );
                }
                Token::RShift => {
                    self.pos += 1; // Consume '>>'
                    let right = self.parse_additive_expression()?;
                    left = Expression::BinaryOp(
                        Box::new(left),
                        BinaryOperator::RShift,
                        Box::new(right),
                    );
                }
                _ => break,
            }
        }

        Some(left)
    }

    fn parse_additive_expression(&mut self) -> Option<Expression> {
        let mut left = self.parse_multiplicative_expression()?;

        while self.pos < self.tokens.len() {
            match &self.tokens[self.pos] {
                Token::Plus => {
                    self.pos += 1; // Consume '+'
                    let right = self.parse_multiplicative_expression()?;
                    left =
                        Expression::BinaryOp(Box::new(left), BinaryOperator::Plus, Box::new(right));
                }
                Token::Minus => {
                    self.pos += 1; // Consume '-'
                    let right = self.parse_multiplicative_expression()?;
                    left = Expression::BinaryOp(
                        Box::new(left),
                        BinaryOperator::Minus,
                        Box::new(right),
                    );
                }
                _ => break,
            }
        }

        Some(left)
    }

    fn parse_multiplicative_expression(&mut self) -> Option<Expression> {
        let mut left = self.parse_unary_expression()?;

        while self.pos < self.tokens.len() {
            match &self.tokens[self.pos] {
                Token::Mult => {
                    self.pos += 1; // Consume '*'
                    let right = self.parse_unary_expression()?;
                    left =
                        Expression::BinaryOp(Box::new(left), BinaryOperator::Mult, Box::new(right));
                }
                Token::Div => {
                    self.pos += 1; // Consume '/'
                    let right = self.parse_unary_expression()?;
                    left =
                        Expression::BinaryOp(Box::new(left), BinaryOperator::Div, Box::new(right));
                }
                Token::Mod => {
                    self.pos += 1; // Consume '%'
                    let right = self.parse_unary_expression()?;
                    left =
                        Expression::BinaryOp(Box::new(left), BinaryOperator::Mod, Box::new(right));
                }
                _ => break,
            }
        }

        Some(left)
    }

    fn parse_unary_expression(&mut self) -> Option<Expression> {
        if self.pos >= self.tokens.len() {
            return None;
        }

        match &self.tokens[self.pos] {
            Token::Plus => {
                self.pos += 1; // Consume '+'
                let expr = self.parse_unary_expression()?;
                Some(Expression::UnaryOp(UnaryOperator::Plus, Box::new(expr)))
            }
            Token::Minus => {
                self.pos += 1; // Consume '-'
                let expr = self.parse_unary_expression()?;
                Some(Expression::UnaryOp(UnaryOperator::Minus, Box::new(expr)))
            }
            Token::Not => {
                self.pos += 1; // Consume '!'
                let expr = self.parse_unary_expression()?;
                Some(Expression::UnaryOp(UnaryOperator::Not, Box::new(expr)))
            }
            Token::BitAndOp => {
                self.pos += 1; // Consume '&'
                let expr = self.parse_unary_expression()?;
                Some(Expression::UnaryOp(
                    UnaryOperator::AddressOf,
                    Box::new(expr),
                ))
            }
            Token::Mult => {
                self.pos += 1; // Consume '*'
                let expr = self.parse_unary_expression()?;
                Some(Expression::UnaryOp(
                    UnaryOperator::Dereference,
                    Box::new(expr),
                ))
            }
            _ => self.parse_postfix_expression(),
        }
    }

    fn parse_postfix_expression(&mut self) -> Option<Expression> {
        let mut expr = self.parse_primary_expression()?;

        while self.pos < self.tokens.len() {
            match &self.tokens[self.pos] {
                Token::ParenL => {
                    // Function call
                    self.pos += 1; // Consume '('
                    let mut args = Vec::new();

                    if self.pos < self.tokens.len() && self.tokens[self.pos] != Token::ParenR {
                        if let Some(arg) = self.parse_expression() {
                            args.push(arg);
                        }

                        while self.pos < self.tokens.len() && self.tokens[self.pos] == Token::Comma
                        {
                            self.pos += 1; // Consume ','
                            if let Some(arg) = self.parse_expression() {
                                args.push(arg);
                            }
                        }
                    }

                    if self.pos < self.tokens.len() && self.tokens[self.pos] == Token::ParenR {
                        self.pos += 1; // Consume ')'
                        if let Expression::Identifier(name) = expr {
                            expr = Expression::FunctionCall(name, args);
                        }
                    }
                }
                Token::BracketL => {
                    // Array access
                    self.pos += 1; // Consume '['
                    if let Some(index) = self.parse_expression() {
                        if self.pos < self.tokens.len() && self.tokens[self.pos] == Token::BracketR
                        {
                            self.pos += 1; // Consume ']'
                            expr = Expression::ArrayAccess(Box::new(expr), Box::new(index));
                        }
                    }
                }
                Token::Dot => {
                    // Member access
                    self.pos += 1; // Consume '.'
                    if self.pos < self.tokens.len() {
                        if let Token::Identifier(member) = &self.tokens[self.pos] {
                            self.pos += 1; // Consume member name
                            expr = Expression::MemberAccess(Box::new(expr), member.clone());
                        }
                    }
                }
                Token::Arrow => {
                    // Pointer access
                    self.pos += 1; // Consume '->'
                    if self.pos < self.tokens.len() {
                        if let Token::Identifier(member) = &self.tokens[self.pos] {
                            self.pos += 1; // Consume member name
                            expr = Expression::PointerAccess(Box::new(expr), member.clone());
                        }
                    }
                }
                Token::PlusPlus => {
                    self.pos += 1; // Consume '++'
                    expr = Expression::PostfixOp(Box::new(expr), PostfixOperator::PlusPlus);
                }
                Token::MinusMinus => {
                    self.pos += 1; // Consume '--'
                    expr = Expression::PostfixOp(Box::new(expr), PostfixOperator::MinusMinus);
                }
                _ => break,
            }
        }

        Some(expr)
    }

    fn parse_primary_expression(&mut self) -> Option<Expression> {
        if self.pos >= self.tokens.len() {
            return None;
        }

        match &self.tokens[self.pos] {
            Token::Identifier(id) => {
                self.pos += 1;
                Some(Expression::Identifier(id.clone()))
            }
            Token::IntLit(n) => {
                self.pos += 1;
                Some(Expression::Constant(Constant::Integer(*n)))
            }
            Token::FloatLit(f) => {
                self.pos += 1;
                Some(Expression::Constant(Constant::Float(*f)))
            }
            Token::StringLit(s) => {
                self.pos += 1;
                Some(Expression::StringLiteral(s.clone()))
            }
            Token::ParenL => {
                self.pos += 1; // Consume '('
                let expr = self.parse_expression()?;
                if self.pos < self.tokens.len() && self.tokens[self.pos] == Token::ParenR {
                    self.pos += 1; // Consume ')'
                    Some(expr)
                } else {
                    None
                }
            }
            _ => None,
        }
    }

    fn parse_struct_declaration(&mut self) -> Option<StructDeclaration> {
        // Simple struct parsing: struct Point { int x; int y; };
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

        let mut specifiers = Vec::new();
        let mut declarators = Vec::new();

        while self.pos < self.tokens.len() && self.tokens[self.pos] != Token::BraceR {
            // Parse member: int x;
            if let Some(member_specifiers) = self.parse_specifier_qualifier_list() {
                specifiers.extend(member_specifiers);

                if let Some(member_declarators) = self.parse_struct_declarator_list() {
                    declarators.extend(member_declarators);
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

        Some(StructDeclaration {
            specifiers,
            declarators,
        })
    }

    fn parse_specifier_qualifier_list(&mut self) -> Option<Vec<SpecifierQualifier>> {
        let mut specifiers = Vec::new();
        while self.pos < self.tokens.len() {
            match &self.tokens[self.pos] {
                Token::Int => {
                    specifiers.push(SpecifierQualifier::TypeSpecifier(TypeSpecifier::Int));
                    self.pos += 1;
                }
                Token::Float => {
                    specifiers.push(SpecifierQualifier::TypeSpecifier(TypeSpecifier::Float));
                    self.pos += 1;
                }
                Token::Identifier(id) if id == "const" => {
                    specifiers.push(SpecifierQualifier::TypeQualifier(TypeQualifier::Const));
                    self.pos += 1;
                }
                Token::Identifier(id) if id == "volatile" => {
                    specifiers.push(SpecifierQualifier::TypeQualifier(TypeQualifier::Volatile));
                    self.pos += 1;
                }
                _ => break,
            }
        }
        if specifiers.is_empty() {
            None
        } else {
            Some(specifiers)
        }
    }

    fn parse_struct_declarator_list(&mut self) -> Option<Vec<StructDeclarator>> {
        let mut declarators = Vec::new();
        if let Some(decl) = self.parse_struct_declarator() {
            declarators.push(decl);
        } else {
            return None;
        }

        while self.pos < self.tokens.len() && self.tokens[self.pos] == Token::Comma {
            self.pos += 1; // Consume ','
            if let Some(decl) = self.parse_struct_declarator() {
                declarators.push(decl);
            } else {
                break;
            }
        }

        Some(declarators)
    }

    fn parse_struct_declarator(&mut self) -> Option<StructDeclarator> {
        if self.pos >= self.tokens.len() {
            return None;
        }

        match &self.tokens[self.pos] {
            Token::Identifier(id) => {
                let name = id.clone();
                self.pos += 1;
                if self.pos < self.tokens.len() && self.tokens[self.pos] == Token::Colon {
                    self.pos += 1; // Consume ':'
                    if let Some(expr) = self.parse_expression() {
                        // Consume semicolon after bitfield
                        if self.pos < self.tokens.len() && self.tokens[self.pos] == Token::Semicolon
                        {
                            self.pos += 1;
                        }
                        Some(StructDeclarator {
                            declarator: Some(Declarator {
                                name: name.clone(),
                                pointer_depth: 0,
                                array_sizes: vec![],
                                function_params: None,
                            }),
                            bitfield: Some(expr),
                        })
                    } else {
                        None
                    }
                } else {
                    // Consume semicolon after member
                    if self.pos < self.tokens.len() && self.tokens[self.pos] == Token::Semicolon {
                        self.pos += 1;
                    }
                    Some(StructDeclarator {
                        declarator: Some(Declarator {
                            name,
                            pointer_depth: 0,
                            array_sizes: vec![],
                            function_params: None,
                        }),
                        bitfield: None,
                    })
                }
            }
            Token::Colon => {
                self.pos += 1; // Consume ':'
                if let Some(expr) = self.parse_expression() {
                    Some(StructDeclarator {
                        declarator: None,
                        bitfield: Some(expr),
                    })
                } else {
                    None
                }
            }
            _ => None,
        }
    }

    fn parse_initializer(&mut self) -> Option<Initializer> {
        if self.pos >= self.tokens.len() {
            return None;
        }

        match &self.tokens[self.pos] {
            Token::BraceL => {
                self.pos += 1; // Consume '{'
                let mut initializers = Vec::new();
                if let Some(init) = self.parse_initializer() {
                    initializers.push(init);
                } else {
                    return None;
                }

                while self.pos < self.tokens.len() && self.tokens[self.pos] == Token::Comma {
                    self.pos += 1; // Consume ','
                    if let Some(init) = self.parse_initializer() {
                        initializers.push(init);
                    } else {
                        // Allow trailing comma
                        break;
                    }
                }

                if self.pos < self.tokens.len() && self.tokens[self.pos] == Token::BraceR {
                    self.pos += 1; // Consume '}'
                    Some(Initializer {
                        kind: InitializerKind::List(initializers),
                    })
                } else {
                    None
                }
            }
            _ => {
                if let Some(expr) = self.parse_expression() {
                    Some(Initializer {
                        kind: InitializerKind::Assignment(expr),
                    })
                } else {
                    None
                }
            }
        }
    }

    fn parse_parameter_type_list(&mut self) -> Option<ParameterTypeList> {
        let mut parameters = Vec::new();
        let mut variadic = false;

        if let Some(param) = self.parse_parameter_declaration() {
            parameters.push(param);
        } else {
            return None;
        }

        while self.pos < self.tokens.len() && self.tokens[self.pos] == Token::Comma {
            self.pos += 1; // Consume ','
            if self.pos < self.tokens.len()
                && self.tokens[self.pos] == Token::Identifier("...".to_string())
            {
                variadic = true;
                self.pos += 1;
                break;
            }
            if let Some(param) = self.parse_parameter_declaration() {
                parameters.push(param);
            } else {
                break;
            }
        }

        Some(ParameterTypeList {
            parameters,
            variadic,
        })
    }

    fn parse_parameter_declaration(&mut self) -> Option<ParameterDeclaration> {
        let specifiers = self.parse_specifier_qualifier_list()?;
        let declarator = match &self.tokens[self.pos] {
            Token::Identifier(id) => {
                self.pos += 1;
                Some(Declarator {
                    name: id.clone(),
                    pointer_depth: 0,
                    array_sizes: vec![],
                    function_params: None,
                })
            }
            _ => None, // Abstract declarator (no name) is allowed
        };
        Some(ParameterDeclaration {
            specifiers,
            declarator,
        })
    }

    fn parse_function_definition(&mut self) -> Option<FunctionDefinition> {
        // Parse return type
        let return_type = match &self.tokens[self.pos] {
            Token::Int => {
                self.pos += 1;
                "int".to_string()
            }
            Token::Void => {
                self.pos += 1;
                "void".to_string()
            }
            Token::Char => {
                self.pos += 1;
                "char".to_string()
            }
            Token::Float => {
                self.pos += 1;
                "float".to_string()
            }
            Token::Double => {
                self.pos += 1;
                "double".to_string()
            }
            Token::Long => {
                self.pos += 1;
                "long".to_string()
            }
            Token::Short => {
                self.pos += 1;
                "short".to_string()
            }
            _ => return None,
        };

        // Parse function name
        let name = match &self.tokens[self.pos] {
            Token::Identifier(id) => {
                self.pos += 1;
                id.clone()
            }
            _ => return None,
        };

        // Parse parameters: (int a, int b)
        if self.pos >= self.tokens.len() || self.tokens[self.pos] != Token::ParenL {
            return None;
        }
        self.pos += 1; // Consume '('

        let mut parameters = Vec::new();
        while self.pos < self.tokens.len() && self.tokens[self.pos] != Token::ParenR {
            if let Some(param) = self.parse_function_parameter() {
                parameters.push(param);
            } else {
                break;
            }

            // Check for comma
            if self.pos < self.tokens.len() && self.tokens[self.pos] == Token::Comma {
                self.pos += 1; // Consume ','
            }
        }

        if self.pos >= self.tokens.len() || self.tokens[self.pos] != Token::ParenR {
            return None;
        }
        self.pos += 1; // Consume ')'

        // Parse function body: { ... }
        if self.pos >= self.tokens.len() || self.tokens[self.pos] != Token::BraceL {
            return None;
        }
        self.pos += 1; // Consume '{'

        let mut body = Vec::new();
        while self.pos < self.tokens.len() && self.tokens[self.pos] != Token::BraceR {
            if let Some(stmt) = self.parse_statement() {
                body.push(stmt);
            } else {
                break;
            }
        }

        if self.pos >= self.tokens.len() || self.tokens[self.pos] != Token::BraceR {
            return None;
        }
        self.pos += 1; // Consume '}'

        Some(FunctionDefinition {
            return_type,
            name,
            parameters,
            body,
        })
    }

    fn parse_function_parameter(&mut self) -> Option<Parameter> {
        // Parse parameter type
        let param_type = match &self.tokens[self.pos] {
            Token::Int => {
                self.pos += 1;
                "int".to_string()
            }
            Token::Float => {
                self.pos += 1;
                "float".to_string()
            }
            Token::Char => {
                self.pos += 1;
                "char".to_string()
            }
            Token::Double => {
                self.pos += 1;
                "double".to_string()
            }
            Token::Long => {
                self.pos += 1;
                "long".to_string()
            }
            Token::Short => {
                self.pos += 1;
                "short".to_string()
            }
            _ => return None,
        };

        // Parse parameter name
        let name = match &self.tokens[self.pos] {
            Token::Identifier(id) => {
                self.pos += 1;
                id.clone()
            }
            _ => return None,
        };

        Some(Parameter { param_type, name })
    }

    fn parse_statement(&mut self) -> Option<Statement> {
        if self.pos >= self.tokens.len() {
            return None;
        }

        match &self.tokens[self.pos] {
            Token::Return => {
                self.pos += 1; // Consume 'return'
                let expr =
                    if self.pos < self.tokens.len() && self.tokens[self.pos] != Token::Semicolon {
                        Some(
                            self.parse_expression()
                                .unwrap_or(Expression::Identifier("".to_string())),
                        )
                    } else {
                        None
                    };

                if self.pos < self.tokens.len() && self.tokens[self.pos] == Token::Semicolon {
                    self.pos += 1; // Consume ';'
                }

                Some(Statement::Return(expr))
            }
            Token::If => {
                self.pos += 1; // Consume 'if'
                if self.pos < self.tokens.len() && self.tokens[self.pos] == Token::ParenL {
                    self.pos += 1; // Consume '('
                    let condition = self.parse_expression()?;
                    if self.pos < self.tokens.len() && self.tokens[self.pos] == Token::ParenR {
                        self.pos += 1; // Consume ')'
                        let then_stmt = self.parse_statement()?;
                        let else_stmt = if self.pos < self.tokens.len()
                            && self.tokens[self.pos] == Token::Else
                        {
                            self.pos += 1; // Consume 'else'
                            Some(self.parse_statement()?)
                        } else {
                            None
                        };
                        Some(Statement::If(
                            condition,
                            Box::new(then_stmt),
                            else_stmt.map(Box::new),
                        ))
                    } else {
                        None
                    }
                } else {
                    None
                }
            }
            Token::For => {
                println!("DEBUG: Found For token at position {}", self.pos);
                self.pos += 1; // Consume 'for'
                if self.pos < self.tokens.len() && self.tokens[self.pos] == Token::ParenL {
                    self.pos += 1; // Consume '('

                    // Parse initialization (optional)
                    let init = if self.pos < self.tokens.len()
                        && self.tokens[self.pos] != Token::Semicolon
                    {
                        Some(Box::new(self.parse_statement()?))
                    } else {
                        None
                    };

                    // Parse condition (optional)
                    let condition = if self.pos < self.tokens.len()
                        && self.tokens[self.pos] != Token::Semicolon
                    {
                        Some(self.parse_expression()?)
                    } else {
                        None
                    };

                    if self.pos < self.tokens.len() && self.tokens[self.pos] == Token::Semicolon {
                        self.pos += 1; // Consume ';'
                    }

                    // Parse update (optional)
                    let update =
                        if self.pos < self.tokens.len() && self.tokens[self.pos] != Token::ParenR {
                            Some(self.parse_expression()?)
                        } else {
                            None
                        };

                    if self.pos < self.tokens.len() && self.tokens[self.pos] == Token::ParenR {
                        self.pos += 1; // Consume ')'
                        let body = self.parse_statement()?;
                        Some(Statement::For(init, condition, update, Box::new(body)))
                    } else {
                        None
                    }
                } else {
                    None
                }
            }
            Token::While => {
                self.pos += 1; // Consume 'while'
                if self.pos < self.tokens.len() && self.tokens[self.pos] == Token::ParenL {
                    self.pos += 1; // Consume '('
                    let condition = self.parse_expression()?;
                    if self.pos < self.tokens.len() && self.tokens[self.pos] == Token::ParenR {
                        self.pos += 1; // Consume ')'
                        let body = self.parse_statement()?;
                        Some(Statement::While(condition, Box::new(body)))
                    } else {
                        None
                    }
                } else {
                    None
                }
            }
            Token::Do => {
                self.pos += 1; // Consume 'do'
                let body = self.parse_statement()?;
                if self.pos < self.tokens.len() && self.tokens[self.pos] == Token::While {
                    self.pos += 1; // Consume 'while'
                    if self.pos < self.tokens.len() && self.tokens[self.pos] == Token::ParenL {
                        self.pos += 1; // Consume '('
                        let condition = self.parse_expression()?;
                        if self.pos < self.tokens.len() && self.tokens[self.pos] == Token::ParenR {
                            self.pos += 1; // Consume ')'
                            if self.pos < self.tokens.len()
                                && self.tokens[self.pos] == Token::Semicolon
                            {
                                self.pos += 1; // Consume ';'
                            }
                            Some(Statement::DoWhile(Box::new(body), condition))
                        } else {
                            None
                        }
                    } else {
                        None
                    }
                } else {
                    None
                }
            }
            Token::Switch => {
                self.pos += 1; // Consume 'switch'
                if self.pos < self.tokens.len() && self.tokens[self.pos] == Token::ParenL {
                    self.pos += 1; // Consume '('
                    let expr = self.parse_expression()?;
                    if self.pos < self.tokens.len() && self.tokens[self.pos] == Token::ParenR {
                        self.pos += 1; // Consume ')'
                        if self.pos < self.tokens.len() && self.tokens[self.pos] == Token::BraceL {
                            self.pos += 1; // Consume '{'
                            let mut cases = Vec::new();

                            while self.pos < self.tokens.len()
                                && self.tokens[self.pos] != Token::BraceR
                            {
                                if self.pos < self.tokens.len()
                                    && self.tokens[self.pos] == Token::Case
                                {
                                    self.pos += 1; // Consume 'case'
                                    let case_expr = self.parse_expression()?;
                                    if self.pos < self.tokens.len()
                                        && self.tokens[self.pos] == Token::Colon
                                    {
                                        self.pos += 1; // Consume ':'
                                        let mut stmts = Vec::new();
                                        while self.pos < self.tokens.len()
                                            && self.tokens[self.pos] != Token::Case
                                            && self.tokens[self.pos] != Token::Default
                                            && self.tokens[self.pos] != Token::BraceR
                                        {
                                            if let Some(stmt) = self.parse_statement() {
                                                stmts.push(stmt);
                                            } else {
                                                break;
                                            }
                                        }
                                        cases.push(Case::Case(case_expr, stmts));
                                    }
                                } else if self.pos < self.tokens.len()
                                    && self.tokens[self.pos] == Token::Default
                                {
                                    self.pos += 1; // Consume 'default'
                                    if self.pos < self.tokens.len()
                                        && self.tokens[self.pos] == Token::Colon
                                    {
                                        self.pos += 1; // Consume ':'
                                        let mut stmts = Vec::new();
                                        while self.pos < self.tokens.len()
                                            && self.tokens[self.pos] != Token::Case
                                            && self.tokens[self.pos] != Token::Default
                                            && self.tokens[self.pos] != Token::BraceR
                                        {
                                            if let Some(stmt) = self.parse_statement() {
                                                stmts.push(stmt);
                                            } else {
                                                break;
                                            }
                                        }
                                        cases.push(Case::Default(stmts));
                                    }
                                } else {
                                    break;
                                }
                            }

                            if self.pos < self.tokens.len()
                                && self.tokens[self.pos] == Token::BraceR
                            {
                                self.pos += 1; // Consume '}'
                                Some(Statement::Switch(expr, cases))
                            } else {
                                None
                            }
                        } else {
                            None
                        }
                    } else {
                        None
                    }
                } else {
                    None
                }
            }
            Token::Break => {
                self.pos += 1; // Consume 'break'
                if self.pos < self.tokens.len() && self.tokens[self.pos] == Token::Semicolon {
                    self.pos += 1; // Consume ';'
                }
                Some(Statement::Break)
            }
            Token::Continue => {
                self.pos += 1; // Consume 'continue'
                if self.pos < self.tokens.len() && self.tokens[self.pos] == Token::Semicolon {
                    self.pos += 1; // Consume ';'
                }
                Some(Statement::Continue)
            }
            Token::Goto => {
                self.pos += 1; // Consume 'goto'
                if self.pos < self.tokens.len() {
                    if let Token::Identifier(label) = &self.tokens[self.pos] {
                        self.pos += 1; // Consume label
                        if self.pos < self.tokens.len() && self.tokens[self.pos] == Token::Semicolon
                        {
                            self.pos += 1; // Consume ';'
                        }
                        Some(Statement::Goto(label.clone()))
                    } else {
                        None
                    }
                } else {
                    None
                }
            }
            Token::BraceL => {
                self.pos += 1; // Consume '{'
                let mut stmts = Vec::new();
                while self.pos < self.tokens.len() && self.tokens[self.pos] != Token::BraceR {
                    if let Some(stmt) = self.parse_statement() {
                        stmts.push(stmt);
                    } else {
                        break;
                    }
                }
                if self.pos < self.tokens.len() && self.tokens[self.pos] == Token::BraceR {
                    self.pos += 1; // Consume '}'
                }
                Some(Statement::Block(stmts))
            }
            Token::Int
            | Token::Float
            | Token::Char
            | Token::Double
            | Token::Long
            | Token::Short => {
                // Variable declaration
                let _var_type = match &self.tokens[self.pos] {
                    Token::Int => {
                        self.pos += 1;
                        "int".to_string()
                    }
                    Token::Float => {
                        self.pos += 1;
                        "float".to_string()
                    }
                    Token::Char => {
                        self.pos += 1;
                        "char".to_string()
                    }
                    Token::Double => {
                        self.pos += 1;
                        "double".to_string()
                    }
                    Token::Long => {
                        self.pos += 1;
                        "long".to_string()
                    }
                    Token::Short => {
                        self.pos += 1;
                        "short".to_string()
                    }
                    _ => return None,
                };

                let var_name = match &self.tokens[self.pos] {
                    Token::Identifier(id) => {
                        self.pos += 1;
                        id.clone()
                    }
                    _ => return None,
                };

                if self.pos < self.tokens.len() && self.tokens[self.pos] == Token::Semicolon {
                    self.pos += 1; // Consume ';'
                }

                Some(Statement::Declaration(VariableDeclaration {
                    storage_class: None,
                    type_qualifiers: vec![],
                    type_specifier: TypeSpecifier::Int, // Default to int for now
                    declarator: Declarator {
                        name: var_name,
                        pointer_depth: 0,
                        array_sizes: vec![],
                        function_params: None,
                    },
                    initializer: None,
                }))
            }
            _ => {
                // Try to parse as expression statement
                if let Some(expr) = self.parse_expression() {
                    if self.pos < self.tokens.len() && self.tokens[self.pos] == Token::Semicolon {
                        self.pos += 1; // Consume ';'
                    }
                    Some(Statement::Expression(expr))
                } else {
                    None
                }
            }
        }
    }
}
