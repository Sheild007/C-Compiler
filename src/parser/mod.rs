// parser_new.rs: A clean, robust parser implementation for MiniC

pub mod ast;

use crate::lexer_regex::Token;
use crate::parser::ast::*;

pub struct Parser {
    tokens: Vec<Token>,
    pos: usize,
}

impl Parser {
    pub fn new(tokens: Vec<Token>) -> Self {
        Parser { tokens, pos: 0 }
    }

    // ============================================
    // Helper Methods
    // ============================================

    /// Skip whitespace-like tokens (comments, errors)
    fn skip_whitespace(&mut self) {
        while self.pos < self.tokens.len() {
            match &self.tokens[self.pos] {
                Token::Comment(_) | Token::BlockComment(_) | Token::Error(_) => {
                    self.pos += 1;
                }
                _ => break,
            }
        }
    }

    /// Peek at the current token without advancing
    fn peek(&self) -> Option<&Token> {
        if self.pos < self.tokens.len() {
            Some(&self.tokens[self.pos])
        } else {
            None
        }
    }

    /// Peek at token at offset from current position
    fn peek_at(&self, offset: usize) -> Option<&Token> {
        if self.pos + offset < self.tokens.len() {
            Some(&self.tokens[self.pos + offset])
        } else {
            None
        }
    }

    /// Consume current token if it matches
    fn consume(&mut self, expected: &Token) -> bool {
        if self.peek() == Some(expected) {
            self.pos += 1;
            true
        } else {
            false
        }
    }

    /// Consume token and return it
    fn next(&mut self) -> Option<Token> {
        if self.pos < self.tokens.len() {
            let token = self.tokens[self.pos].clone();
            self.pos += 1;
            Some(token)
        } else {
            None
        }
    }

    /// Check if we're at top level (no unmatched braces)
    fn is_at_top_level(&self) -> bool {
        let mut brace_count = 0;
        for i in 0..self.pos {
            match &self.tokens[i] {
                Token::BraceL => brace_count += 1,
                Token::BraceR => brace_count -= 1,
                _ => {}
            }
        }
        brace_count == 0
    }

    // ============================================
    // Main Entry Point
    // ============================================

    pub fn parse(&mut self) -> Result<TranslationUnit, ParseError> {
        let mut preprocessor_list = Vec::new();
        let mut external_declarations = Vec::new();

        while self.pos < self.tokens.len() {
            self.skip_whitespace();

            if self.pos >= self.tokens.len() {
                break;
            }

            match self.peek() {
                Some(Token::Preprocessor(_)) => {
                    if let Ok(directive) = self.parse_preprocessor_directive() {
                        preprocessor_list.push(directive);
                    }
                }
                Some(Token::Error(msg)) => {
                    return Err(ParseError::UnexpectedToken(format!("Lexer error: {}", msg)));
                }
                _ => {
                    if self.is_at_top_level() {
                        if let Some(decl) = self.parse_external_declaration() {
                            external_declarations.push(decl);
                        } else {
                            // Check for specific errors
                            if let Err(e) = self.check_for_specific_errors() {
                                return Err(e);
                            }
                            // Skip unrecognized token
                            self.pos += 1;
                        }
                    } else {
                        // Inside a function body - skip until we're back at top level
                        self.skip_to_top_level();
                    }
                }
            }
        }

        Ok(TranslationUnit {
            preprocessor_list,
            external_declarations,
        })
    }

    /// Skip tokens until we're back at top level
    fn skip_to_top_level(&mut self) {
        let mut brace_count = 0;
        for i in 0..self.pos {
            match &self.tokens[i] {
                Token::BraceL => brace_count += 1,
                Token::BraceR => brace_count -= 1,
                _ => {}
            }
        }
        while self.pos < self.tokens.len() && brace_count > 0 {
            match &self.tokens[self.pos] {
                Token::BraceL => brace_count += 1,
                Token::BraceR => brace_count -= 1,
                _ => {}
            }
            self.pos += 1;
        }
    }

    // ============================================
    // Preprocessor Directives
    // ============================================

    fn parse_preprocessor_directive(&mut self) -> Result<PreprocessorDirective, ParseError> {
        match self.next() {
            Some(Token::Preprocessor(directive)) => {
                let directive_type = directive.strip_prefix('#').unwrap_or(&directive).to_string();
                match directive_type.as_str() {
                    "include" => self.parse_include(),
                    "define" => self.parse_define(),
                    "ifdef" => self.parse_ifdef(),
                    "ifndef" => self.parse_ifndef(),
                    "endif" => Ok(PreprocessorDirective::Endif),
                    _ => Err(ParseError::UnexpectedToken(format!("Unknown directive: {}", directive_type))),
                }
            }
            _ => Err(ParseError::UnexpectedEOF),
        }
    }

    fn parse_include(&mut self) -> Result<PreprocessorDirective, ParseError> {
        if let Some(Token::StringLit(s)) = self.peek() {
            let s = s.clone();
            self.pos += 1;
            return Ok(PreprocessorDirective::Include(s));
        }
        if self.consume(&Token::LessOp) {
            let mut header = String::new();
            while self.pos < self.tokens.len() && self.tokens[self.pos] != Token::GreaterOp {
                match &self.tokens[self.pos] {
                    Token::Identifier(id) => header.push_str(id),
                    Token::Dot => header.push('.'),
                    _ => {}
                }
                self.pos += 1;
            }
            if self.consume(&Token::GreaterOp) {
                Ok(PreprocessorDirective::Include(header))
            } else {
                Err(ParseError::UnexpectedEOF)
            }
        } else {
            Err(ParseError::UnexpectedToken("Expected include path".to_string()))
        }
    }

    fn parse_define(&mut self) -> Result<PreprocessorDirective, ParseError> {
        match self.next() {
            Some(Token::Identifier(id)) => {
                let replacement_list = self.parse_replacement_list();
                Ok(PreprocessorDirective::Define(id, replacement_list))
            }
            _ => Err(ParseError::ExpectedIdentifier),
        }
    }

    fn parse_ifdef(&mut self) -> Result<PreprocessorDirective, ParseError> {
        match self.next() {
            Some(Token::Identifier(id)) => Ok(PreprocessorDirective::Ifdef(id)),
            _ => Err(ParseError::ExpectedIdentifier),
        }
    }

    fn parse_ifndef(&mut self) -> Result<PreprocessorDirective, ParseError> {
        match self.next() {
            Some(Token::Identifier(id)) => Ok(PreprocessorDirective::Ifndef(id)),
            _ => Err(ParseError::ExpectedIdentifier),
        }
    }

    fn parse_replacement_list(&mut self) -> Vec<ReplacementItem> {
        let mut items = Vec::new();
        while self.pos < self.tokens.len() {
            match self.peek() {
                Some(Token::Identifier(id)) => {
                    items.push(ReplacementItem::Identifier(id.clone()));
                    self.pos += 1;
                }
                Some(Token::IntLit(n)) => {
                    items.push(ReplacementItem::Constant(Constant::Integer(*n)));
                    self.pos += 1;
                }
                Some(Token::FloatLit(f)) => {
                    items.push(ReplacementItem::Constant(Constant::Float(*f)));
                    self.pos += 1;
                }
                Some(Token::StringLit(s)) => {
                    items.push(ReplacementItem::StringLiteral(s.clone()));
                    self.pos += 1;
                }
                _ => break,
            }
        }
        items
    }

    // ============================================
    // External Declarations
    // ============================================

    fn parse_external_declaration(&mut self) -> Option<ExternalDeclaration> {
        self.skip_whitespace();
        let saved_pos = self.pos;

        // Handle storage class specifiers
        let storage_class = if self.consume(&Token::Static) {
            Some(StorageClass::Static)
        } else {
            None
        };

        // Handle type qualifiers
        let mut type_qualifiers = Vec::new();
        if self.consume(&Token::Const) {
            type_qualifiers.push(TypeQualifier::Const);
        }

        // Check if this is a function or variable
        if self.is_type_specifier() {
            if self.is_function_declaration() {
                // Try function definition first
                self.pos = saved_pos;
                if let Some(func) = self.parse_function_definition() {
                    return Some(ExternalDeclaration::Function(func));
                }
                // Try function declaration
                self.pos = saved_pos;
                if let Some(func_decl) = self.parse_function_declaration() {
                    return Some(ExternalDeclaration::FunctionDeclaration(func_decl));
                }
            }
            // Try variable declaration
            self.pos = saved_pos;
            if let Some(mut var_decl) = self.parse_variable_declaration() {
                if let Some(sc) = storage_class {
                    var_decl.storage_class = Some(sc);
                }
                if !type_qualifiers.is_empty() {
                    var_decl.type_qualifiers = type_qualifiers;
                }
                return Some(ExternalDeclaration::Variable(var_decl));
            }
        }

        self.pos = saved_pos;
        None
    }

    /// Check if current token is a type specifier
    fn is_type_specifier(&self) -> bool {
        matches!(
            self.peek(),
            Some(Token::Int)
                | Some(Token::Float)
                | Some(Token::Char)
                | Some(Token::Double)
                | Some(Token::Void)
                | Some(Token::Long)
                | Some(Token::Short)
        )
    }

    /// Check if this looks like a function (has parentheses after identifier)
    fn is_function_declaration(&self) -> bool {
        let _saved_pos = self.pos;
        let mut check_pos = self.pos;

        // Skip type specifier
        if !self.is_type_specifier() {
            return false;
        }
        check_pos += 1;

        // Skip whitespace
        while check_pos < self.tokens.len() {
            match &self.tokens[check_pos] {
                Token::Comment(_) | Token::BlockComment(_) | Token::Error(_) => {
                    check_pos += 1;
                }
                _ => break,
            }
        }

        // Check for identifier
        if !matches!(&self.tokens.get(check_pos), Some(Token::Identifier(_))) {
            return false;
        }
        check_pos += 1;

        // Skip whitespace
        while check_pos < self.tokens.len() {
            match &self.tokens[check_pos] {
                Token::Comment(_) | Token::BlockComment(_) | Token::Error(_) => {
                    check_pos += 1;
                }
                _ => break,
            }
        }

        // Check for opening parenthesis
        matches!(&self.tokens.get(check_pos), Some(Token::ParenL))
    }

    // ============================================
    // Variable Declarations
    // ============================================

    fn parse_variable_declaration(&mut self) -> Option<VariableDeclaration> {
        let type_specifier = self.parse_type_specifier()?;
        self.skip_whitespace();

        let name = match self.next() {
            Some(Token::Identifier(id)) => id,
            _ => return None,
        };

        // Parse initializer if present
        let mut initializer = None;
        if self.consume(&Token::AssignOp) {
            if let Some(expr) = self.parse_expression() {
                initializer = Some(Initializer {
                    kind: InitializerKind::Assignment(expr),
                });
            }
        }

        // Consume semicolon
        if !self.consume(&Token::Semicolon) {
            return None;
        }

        Some(VariableDeclaration {
            storage_class: None,
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

    fn parse_type_specifier(&mut self) -> Option<TypeSpecifier> {
        match self.next() {
            Some(Token::Int) => Some(TypeSpecifier::Int),
            Some(Token::Float) => Some(TypeSpecifier::Float),
            Some(Token::Char) => Some(TypeSpecifier::Char),
            Some(Token::Double) => Some(TypeSpecifier::Double),
            Some(Token::Void) => Some(TypeSpecifier::Void),
            Some(Token::Long) => Some(TypeSpecifier::Long),
            Some(Token::Short) => Some(TypeSpecifier::Short),
            _ => None,
        }
    }

    fn parse_type_specifier_string(&mut self) -> Option<String> {
        match self.next() {
            Some(Token::Int) => Some("int".to_string()),
            Some(Token::Float) => Some("float".to_string()),
            Some(Token::Char) => Some("char".to_string()),
            Some(Token::Double) => Some("double".to_string()),
            Some(Token::Void) => Some("void".to_string()),
            Some(Token::Long) => Some("long".to_string()),
            Some(Token::Short) => Some("short".to_string()),
            _ => None,
        }
    }

    // ============================================
    // Function Declarations
    // ============================================

    fn parse_function_declaration(&mut self) -> Option<FunctionDeclaration> {
        let saved_pos = self.pos;
        let return_type = self.parse_type_specifier_string()?;
        self.skip_whitespace();

        let name = match self.next() {
            Some(Token::Identifier(id)) => id,
            _ => {
                self.pos = saved_pos;
                return None;
            }
        };

        if !self.consume(&Token::ParenL) {
            self.pos = saved_pos;
            return None;
        }

        let parameters = self.parse_parameter_list();

        if !self.consume(&Token::ParenR) {
            self.pos = saved_pos;
            return None;
        }

        // Must have semicolon for declaration
        if !self.consume(&Token::Semicolon) {
            self.pos = saved_pos;
            return None;
        }

        Some(FunctionDeclaration {
            return_type,
            name,
            parameters,
        })
    }

    // ============================================
    // Function Definitions
    // ============================================

    fn parse_function_definition(&mut self) -> Option<FunctionDefinition> {
        let saved_pos = self.pos;
        let return_type = self.parse_type_specifier_string()?;
        self.skip_whitespace();

        let name = match self.next() {
            Some(Token::Identifier(id)) => id,
            _ => {
                self.pos = saved_pos;
                return None;
            }
        };

        if !self.consume(&Token::ParenL) {
            self.pos = saved_pos;
            return None;
        }

        let parameters = self.parse_parameter_list();

        if !self.consume(&Token::ParenR) {
            self.pos = saved_pos;
            return None;
        }

        // Must have body for definition
        if !self.consume(&Token::BraceL) {
            self.pos = saved_pos;
            return None;
        }

        // Parse function body
        let body = self.parse_statement_list();

        // Find matching closing brace
        if !self.find_matching_brace() {
            self.pos = saved_pos;
            return None;
        }

        Some(FunctionDefinition {
            return_type,
            name,
            parameters,
            body,
        })
    }

    fn parse_parameter_list(&mut self) -> Vec<Parameter> {
        let mut parameters = Vec::new();

        while self.pos < self.tokens.len() && self.tokens[self.pos] != Token::ParenR {
            self.skip_whitespace();
            if self.pos >= self.tokens.len() || self.tokens[self.pos] == Token::ParenR {
                break;
            }

            if let Some(param) = self.parse_parameter() {
                parameters.push(param);
            } else {
                break;
            }

            if self.consume(&Token::Comma) {
                continue;
            } else {
                break;
            }
        }

        parameters
    }

    fn parse_parameter(&mut self) -> Option<Parameter> {
        let param_type = self.parse_type_specifier_string()?;
        self.skip_whitespace();

        let name = match self.next() {
            Some(Token::Identifier(id)) => id,
            _ => return None,
        };

        Some(Parameter { param_type, name })
    }

    /// Find matching closing brace and advance position
    fn find_matching_brace(&mut self) -> bool {
        if self.consume(&Token::BraceR) {
            return true;
        }

        // Search for matching brace by counting
        let mut brace_count = 1;
        let mut search_pos = self.pos;

        while search_pos < self.tokens.len() && brace_count > 0 {
            match &self.tokens[search_pos] {
                Token::BraceL => brace_count += 1,
                Token::BraceR => {
                    brace_count -= 1;
                    if brace_count == 0 {
                        self.pos = search_pos + 1;
                        return true;
                    }
                }
                _ => {}
            }
            search_pos += 1;
        }

        false
    }

    // ============================================
    // Statements
    // ============================================

    fn parse_statement_list(&mut self) -> Vec<Statement> {
        let mut statements = Vec::new();

        while self.pos < self.tokens.len() && self.tokens[self.pos] != Token::BraceR {
            self.skip_whitespace();
            if self.pos >= self.tokens.len() || self.tokens[self.pos] == Token::BraceR {
                break;
            }

            if let Some(stmt) = self.parse_statement() {
                statements.push(stmt);
            } else {
                // Skip unrecognized token
                self.pos += 1;
            }
        }

        statements
    }

    fn parse_statement(&mut self) -> Option<Statement> {
        self.skip_whitespace();

        match self.peek() {
            Some(Token::Return) => self.parse_return_statement(),
            Some(Token::If) => self.parse_if_statement(),
            Some(Token::While) => self.parse_while_statement(),
            Some(Token::For) => self.parse_for_statement(),
            Some(Token::Break) => self.parse_break_statement(),
            Some(Token::BraceL) => self.parse_block_statement(),
            Some(Token::Int)
            | Some(Token::Float)
            | Some(Token::Char)
            | Some(Token::Double)
            | Some(Token::Long)
            | Some(Token::Short) => self.parse_declaration_statement(),
            _ => self.parse_expression_statement(),
        }
    }

    fn parse_return_statement(&mut self) -> Option<Statement> {
        if !self.consume(&Token::Return) {
            return None;
        }

        self.skip_whitespace();
        let expr = if self.peek() != Some(&Token::Semicolon) {
            self.parse_expression()
        } else {
            None
        };

        self.consume(&Token::Semicolon);
        Some(Statement::Return(expr))
    }

    fn parse_if_statement(&mut self) -> Option<Statement> {
        if !self.consume(&Token::If) {
            return None;
        }

        if !self.consume(&Token::ParenL) {
            return None;
        }

        let condition = self.parse_expression()?;

        if !self.consume(&Token::ParenR) {
            return None;
        }

        let then_stmt = self.parse_statement()?;

        let else_stmt = if self.consume(&Token::Else) {
            self.parse_statement()
        } else {
            None
        };

        Some(Statement::If(
            condition,
            Box::new(then_stmt),
            else_stmt.map(Box::new),
        ))
    }

    fn parse_while_statement(&mut self) -> Option<Statement> {
        if !self.consume(&Token::While) {
            return None;
        }

        if !self.consume(&Token::ParenL) {
            return None;
        }

        let condition = self.parse_expression()?;

        if !self.consume(&Token::ParenR) {
            return None;
        }

        let body = self.parse_statement()?;

        Some(Statement::While(condition, Box::new(body)))
    }

    fn parse_for_statement(&mut self) -> Option<Statement> {
        if !self.consume(&Token::For) {
            return None;
        }

        if !self.consume(&Token::ParenL) {
            return None;
        }

        // Parse init (optional)
        let init = if self.peek() != Some(&Token::Semicolon) {
            self.parse_statement()
        } else {
            None
        };

        self.consume(&Token::Semicolon);

        // Parse condition (optional)
        let condition = if self.peek() != Some(&Token::Semicolon) {
            self.parse_expression()
        } else {
            None
        };

        self.consume(&Token::Semicolon);

        // Parse update (optional)
        let update = if self.peek() != Some(&Token::ParenR) {
            self.parse_expression()
        } else {
            None
        };

        if !self.consume(&Token::ParenR) {
            return None;
        }

        let body = self.parse_statement()?;

        Some(Statement::For(
            init.map(Box::new),
            condition,
            update,
            Box::new(body),
        ))
    }

    fn parse_break_statement(&mut self) -> Option<Statement> {
        if self.consume(&Token::Break) {
            self.consume(&Token::Semicolon);
            Some(Statement::Break)
        } else {
            None
        }
    }

    fn parse_block_statement(&mut self) -> Option<Statement> {
        if !self.consume(&Token::BraceL) {
            return None;
        }

        let stmts = self.parse_statement_list();

        if self.consume(&Token::BraceR) {
            Some(Statement::Block(stmts))
        } else {
            None
        }
    }

    fn parse_declaration_statement(&mut self) -> Option<Statement> {
        if let Some(var_decl) = self.parse_variable_declaration() {
            Some(Statement::Declaration(var_decl))
        } else {
            None
        }
    }

    fn parse_expression_statement(&mut self) -> Option<Statement> {
        if let Some(expr) = self.parse_expression() {
            self.consume(&Token::Semicolon);
            Some(Statement::Expression(expr))
        } else {
            None
        }
    }

    // ============================================
    // Expressions
    // ============================================

    fn parse_expression(&mut self) -> Option<Expression> {
        self.parse_assignment_expression()
    }

    fn parse_assignment_expression(&mut self) -> Option<Expression> {
        let mut left = self.parse_conditional_expression()?;

        while let Some(op) = self.peek() {
            let assignment_op = match op {
                Token::AssignOp => Some(AssignmentOperator::Assign),
                Token::PlusAssign => Some(AssignmentOperator::PlusAssign),
                Token::MinusAssign => Some(AssignmentOperator::MinusAssign),
                Token::MultAssign => Some(AssignmentOperator::MultAssign),
                Token::DivAssign => Some(AssignmentOperator::DivAssign),
                Token::ModAssign => Some(AssignmentOperator::ModAssign),
                _ => None,
            };

            if let Some(op) = assignment_op {
                self.pos += 1;
                if let Some(right) = self.parse_assignment_expression() {
                    left = Expression::Assignment(Box::new(left), op, Box::new(right));
                } else {
                    return None;
                }
            } else {
                break;
            }
        }

        Some(left)
    }

    fn parse_conditional_expression(&mut self) -> Option<Expression> {
        let condition = self.parse_logical_or_expression()?;

        if self.consume(&Token::Question) {
            let true_expr = self.parse_expression()?;
            if self.consume(&Token::Colon) {
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

        while self.consume(&Token::OrOp) {
            if let Some(right) = self.parse_logical_and_expression() {
                left = Expression::BinaryOp(Box::new(left), BinaryOperator::Or, Box::new(right));
            } else {
                return None;
            }
        }

        Some(left)
    }

    fn parse_logical_and_expression(&mut self) -> Option<Expression> {
        let mut left = self.parse_bitwise_or_expression()?;

        while self.consume(&Token::AndOp) {
            if let Some(right) = self.parse_bitwise_or_expression() {
                left = Expression::BinaryOp(Box::new(left), BinaryOperator::And, Box::new(right));
            } else {
                return None;
            }
        }

        Some(left)
    }

    fn parse_bitwise_or_expression(&mut self) -> Option<Expression> {
        let mut left = self.parse_bitwise_xor_expression()?;

        while self.consume(&Token::BitOrOp) {
            if let Some(right) = self.parse_bitwise_xor_expression() {
                left = Expression::BinaryOp(Box::new(left), BinaryOperator::BitOr, Box::new(right));
            } else {
                return None;
            }
        }

        Some(left)
    }

    fn parse_bitwise_xor_expression(&mut self) -> Option<Expression> {
        let mut left = self.parse_bitwise_and_expression()?;

        while self.consume(&Token::Xor) {
            if let Some(right) = self.parse_bitwise_and_expression() {
                left = Expression::BinaryOp(Box::new(left), BinaryOperator::Xor, Box::new(right));
            } else {
                return None;
            }
        }

        Some(left)
    }

    fn parse_bitwise_and_expression(&mut self) -> Option<Expression> {
        let mut left = self.parse_equality_expression()?;

        while self.consume(&Token::BitAndOp) {
            if let Some(right) = self.parse_equality_expression() {
                left = Expression::BinaryOp(Box::new(left), BinaryOperator::BitAnd, Box::new(right));
            } else {
                return None;
            }
        }

        Some(left)
    }

    fn parse_equality_expression(&mut self) -> Option<Expression> {
        let mut left = self.parse_relational_expression()?;

        loop {
            let op = if self.consume(&Token::EqualsOp) {
                Some(BinaryOperator::Equals)
            } else if self.consume(&Token::NotEqualsOp) {
                Some(BinaryOperator::NotEquals)
            } else {
                None
            };

            if let Some(op) = op {
                if let Some(right) = self.parse_relational_expression() {
                    left = Expression::BinaryOp(Box::new(left), op, Box::new(right));
                } else {
                    return None;
                }
            } else {
                break;
            }
        }

        Some(left)
    }

    fn parse_relational_expression(&mut self) -> Option<Expression> {
        let mut left = self.parse_shift_expression()?;

        loop {
            let op = if self.consume(&Token::LessOp) {
                Some(BinaryOperator::Less)
            } else if self.consume(&Token::GreaterOp) {
                Some(BinaryOperator::Greater)
            } else if self.consume(&Token::LessEqOp) {
                Some(BinaryOperator::LessEq)
            } else if self.consume(&Token::GreaterEqOp) {
                Some(BinaryOperator::GreaterEq)
            } else {
                None
            };

            if let Some(op) = op {
                if let Some(right) = self.parse_shift_expression() {
                    left = Expression::BinaryOp(Box::new(left), op, Box::new(right));
                } else {
                    return None;
                }
            } else {
                break;
            }
        }

        Some(left)
    }

    fn parse_shift_expression(&mut self) -> Option<Expression> {
        let mut left = self.parse_additive_expression()?;

        loop {
            let op = if self.consume(&Token::LShift) {
                Some(BinaryOperator::LShift)
            } else if self.consume(&Token::RShift) {
                Some(BinaryOperator::RShift)
            } else {
                None
            };

            if let Some(op) = op {
                if let Some(right) = self.parse_additive_expression() {
                    left = Expression::BinaryOp(Box::new(left), op, Box::new(right));
                } else {
                    return None;
                }
            } else {
                break;
            }
        }

        Some(left)
    }

    fn parse_additive_expression(&mut self) -> Option<Expression> {
        let mut left = self.parse_multiplicative_expression()?;

        loop {
            let op = if self.consume(&Token::Plus) {
                Some(BinaryOperator::Plus)
            } else if self.consume(&Token::Minus) {
                Some(BinaryOperator::Minus)
            } else {
                None
            };

            if let Some(op) = op {
                if let Some(right) = self.parse_multiplicative_expression() {
                    left = Expression::BinaryOp(Box::new(left), op, Box::new(right));
                } else {
                    return None;
                }
            } else {
                break;
            }
        }

        Some(left)
    }

    fn parse_multiplicative_expression(&mut self) -> Option<Expression> {
        let mut left = self.parse_unary_expression()?;

        loop {
            let op = if self.consume(&Token::Mult) {
                Some(BinaryOperator::Mult)
            } else if self.consume(&Token::Div) {
                Some(BinaryOperator::Div)
            } else if self.consume(&Token::Mod) {
                Some(BinaryOperator::Mod)
            } else {
                None
            };

            if let Some(op) = op {
                if let Some(right) = self.parse_unary_expression() {
                    left = Expression::BinaryOp(Box::new(left), op, Box::new(right));
                } else {
                    return None;
                }
            } else {
                break;
            }
        }

        Some(left)
    }

    fn parse_unary_expression(&mut self) -> Option<Expression> {
        if let Some(op) = self.peek() {
            match op {
                Token::Plus => {
                    self.pos += 1;
                    if let Some(expr) = self.parse_unary_expression() {
                        return Some(Expression::UnaryOp(UnaryOperator::Plus, Box::new(expr)));
                    }
                }
                Token::Minus => {
                    self.pos += 1;
                    if let Some(expr) = self.parse_unary_expression() {
                        return Some(Expression::UnaryOp(UnaryOperator::Minus, Box::new(expr)));
                    }
                }
                Token::Not => {
                    self.pos += 1;
                    if let Some(expr) = self.parse_unary_expression() {
                        return Some(Expression::UnaryOp(UnaryOperator::Not, Box::new(expr)));
                    }
                }
                Token::BitAndOp => {
                    self.pos += 1;
                    if let Some(expr) = self.parse_unary_expression() {
                        return Some(Expression::UnaryOp(UnaryOperator::AddressOf, Box::new(expr)));
                    }
                }
                Token::Mult => {
                    self.pos += 1;
                    if let Some(expr) = self.parse_unary_expression() {
                        return Some(Expression::UnaryOp(UnaryOperator::Dereference, Box::new(expr)));
                    }
                }
                _ => {}
            }
        }

        self.parse_postfix_expression()
    }

    fn parse_postfix_expression(&mut self) -> Option<Expression> {
        let mut expr = self.parse_primary_expression()?;

        loop {
            match self.peek() {
                Some(Token::ParenL) => {
                    self.pos += 1;
                    let mut args = Vec::new();

                    if self.peek() != Some(&Token::ParenR) {
                        if let Some(arg) = self.parse_expression() {
                            args.push(arg);
                        }

                        while self.consume(&Token::Comma) {
                            if let Some(arg) = self.parse_expression() {
                                args.push(arg);
                            }
                        }
                    }

                    if self.consume(&Token::ParenR) {
                        if let Expression::Identifier(name) = expr {
                            expr = Expression::FunctionCall(name, args);
                        }
                    } else {
                        break;
                    }
                }
                Some(Token::BracketL) => {
                    self.pos += 1;
                    if let Some(index) = self.parse_expression() {
                        if self.consume(&Token::BracketR) {
                            expr = Expression::ArrayAccess(Box::new(expr), Box::new(index));
                        } else {
                            break;
                        }
                    } else {
                        break;
                    }
                }
                Some(Token::Dot) => {
                    self.pos += 1;
                    if let Some(Token::Identifier(member)) = self.next() {
                        expr = Expression::MemberAccess(Box::new(expr), member);
                    } else {
                        break;
                    }
                }
                Some(Token::Arrow) => {
                    self.pos += 1;
                    if let Some(Token::Identifier(member)) = self.next() {
                        expr = Expression::PointerAccess(Box::new(expr), member);
                    } else {
                        break;
                    }
                }
                Some(Token::PlusPlus) => {
                    self.pos += 1;
                    expr = Expression::PostfixOp(Box::new(expr), PostfixOperator::PlusPlus);
                }
                Some(Token::MinusMinus) => {
                    self.pos += 1;
                    expr = Expression::PostfixOp(Box::new(expr), PostfixOperator::MinusMinus);
                }
                _ => break,
            }
        }

        Some(expr)
    }

    fn parse_primary_expression(&mut self) -> Option<Expression> {
        match self.next() {
            Some(Token::Identifier(id)) => Some(Expression::Identifier(id)),
            Some(Token::IntLit(n)) => Some(Expression::Constant(Constant::Integer(n))),
            Some(Token::FloatLit(f)) => Some(Expression::Constant(Constant::Float(f))),
            Some(Token::StringLit(s)) => Some(Expression::StringLiteral(s)),
            Some(Token::ParenL) => {
                let expr = self.parse_expression()?;
                if self.consume(&Token::ParenR) {
                    Some(expr)
                } else {
                    None
                }
            }
            _ => None,
        }
    }

    // ============================================
    // Error Detection
    // ============================================

    fn check_for_specific_errors(&mut self) -> Result<(), ParseError> {
        if !self.is_at_top_level() {
            return Ok(());
        }

        // Check for missing identifier after type: int = 5;
        if let (Some(Token::Int | Token::Float | Token::Char | Token::Double | Token::Long | Token::Short | Token::Void),
                Some(Token::AssignOp),
                Some(Token::IntLit(_) | Token::FloatLit(_) | Token::StringLit(_) | Token::BoolLit(_))) =
            (self.peek(), self.peek_at(1), self.peek_at(2))
        {
            return Err(ParseError::ExpectedIdentifier);
        }

        // Check for missing type specifier: x = 5;
        if let (Some(Token::Identifier(_)), Some(Token::AssignOp)) = (self.peek(), self.peek_at(1)) {
            return Err(ParseError::ExpectedTypeToken);
        }

        // Check for missing value after assignment: int x = ;
        if let (Some(Token::Int | Token::Float | Token::Char | Token::Double),
                Some(Token::Identifier(_)),
                Some(Token::AssignOp),
                Some(Token::Semicolon)) =
            (self.peek(), self.peek_at(1), self.peek_at(2), self.peek_at(3))
        {
            if matches!(self.peek(), Some(Token::Int)) {
                return Err(ParseError::ExpectedIntLit);
            } else if matches!(self.peek(), Some(Token::Float)) {
                return Err(ParseError::ExpectedFloatLit);
            } else if matches!(self.peek(), Some(Token::Char)) {
                return Err(ParseError::ExpectedStringLit);
            }
        }

        // Check for missing operand after operator: int x = 5 + ;
        if let (Some(Token::Int | Token::Float | Token::Char | Token::Double),
                Some(Token::Identifier(_)),
                Some(Token::AssignOp),
                Some(Token::IntLit(_) | Token::FloatLit(_)),
                Some(Token::Plus | Token::Minus | Token::Mult | Token::Div),
                Some(Token::Semicolon)) =
            (self.peek(), self.peek_at(1), self.peek_at(2), self.peek_at(3), self.peek_at(4), self.peek_at(5))
        {
            return Err(ParseError::FailedToFindToken("Missing operand after operator".to_string()));
        }

        Ok(())
    }
}

