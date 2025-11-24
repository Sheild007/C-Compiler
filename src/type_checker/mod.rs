// type_checker/mod.rs: Type checking implementation for MiniC compiler

use crate::parser::ast::*;
use crate::scope::{ScopeAnalyzer, SymbolKind, ScopeNode};
use std::rc::Rc;

#[derive(Debug, Clone)]
pub enum TypeChkError {
    ErroneousVarDecl,
    FnCallParamCount,
    FnCallParamType,
    ErroneousReturnType,
    ExpressionTypeMismatch,
    ExpectedBooleanExpression,
    ErroneousBreak,
    NonBooleanCondStmt,
    EmptyExpression,
    AttemptedBoolOpOnNonBools,
    AttemptedBitOpOnNonNumeric,
    AttemptedShiftOnNonInt,
    AttemptedAddOpOnNonNumeric,
    AttemptedExponentiationOfNonNumeric,
    ReturnStmtNotFound,
}

#[derive(Debug, Clone, PartialEq)]
pub enum Type {
    Int,
    Float,
    Double,
    Char,
    Short,
    Long,
    Void,
    Bool, // For boolean expressions
    String, // For string literals (char arrays/pointers)
    Unknown, // For error cases
}

pub struct TypeChecker {
    scope_analyzer: ScopeAnalyzer,
    errors: Vec<TypeError>,
    current_return_type: Option<Type>,
    in_loop: bool, // Track if we're inside a loop (for break statements)
    current_scope: Option<Rc<ScopeNode>>, // Track current scope during type checking
    source_lines: Vec<String>,
}

#[derive(Debug, Clone)]
pub struct TypeError {
    pub error: TypeChkError,
    pub line: Option<usize>,
    pub context: String,
}

impl TypeChecker {
    pub fn new(scope_analyzer: ScopeAnalyzer, source_lines: Vec<String>) -> Self {
        let global_scope = scope_analyzer.get_global_scope().clone();
        TypeChecker {
            scope_analyzer,
            errors: Vec::new(),
            current_return_type: None,
            in_loop: false,
            current_scope: Some(global_scope),
            source_lines,
        }
    }

    pub fn check_translation_unit(&mut self, unit: &TranslationUnit) -> Result<(), Vec<TypeError>> {
        for external_decl in &unit.external_declarations {
            self.check_external_declaration(external_decl);
        }

        if self.errors.is_empty() {
            Ok(())
        } else {
            Err(self.errors.clone())
        }
    }

    fn check_external_declaration(&mut self, decl: &ExternalDeclaration) {
        match decl {
            ExternalDeclaration::Variable(var_decl) => {
                self.check_variable_declaration(var_decl);
            }
            ExternalDeclaration::Function(func_def) => {
                self.check_function_definition(func_def);
            }
            ExternalDeclaration::FunctionDeclaration(_func_decl) => {
                // Function declarations don't need type checking, only definitions
            }
        }
    }

    fn check_variable_declaration(&mut self, var_decl: &VariableDeclaration) {
        let var_type = self.type_specifier_to_type(&var_decl.type_specifier);
        
        // Check if variable type is valid
        if var_type == Type::Unknown {
            self.record_error(TypeChkError::ErroneousVarDecl, &var_decl.declarator.name);
        }

        // Check initializer if present
        if let Some(initializer) = &var_decl.initializer {
            if let Some(init_type) = self.check_initializer(initializer) {
                // Check if initializer type is compatible with variable type
                if init_type != Type::Unknown && !self.are_types_compatible(&var_type, &init_type) {
                    self.record_error(TypeChkError::ExpressionTypeMismatch, &var_decl.declarator.name);
                }
            }
            // If check_initializer returns None, error was already reported in check_expression
        }
    }

    fn check_initializer(&mut self, initializer: &Initializer) -> Option<Type> {
        match &initializer.kind {
            InitializerKind::Assignment(expr) => {
                self.check_expression(expr)
            }
            InitializerKind::List(initializers) => {
                // For list initializers, check all elements
                for init in initializers {
                    self.check_initializer(init);
                }
                // Return type of first element or None
                if let Some(first) = initializers.first() {
                    self.check_initializer(first)
                } else {
                    None
                }
            }
            InitializerKind::Designated(_designator, init) => {
                self.check_initializer(init)
            }
        }
    }

    fn check_function_definition(&mut self, func_def: &FunctionDefinition) {
        // Set current return type for return statement checking
        let return_type_str = &func_def.return_type;
        self.current_return_type = Some(self.string_to_type(return_type_str));

        // Find function scope from all_scopes (function scope has level 1 and contains all parameters)
        // The function scope can contain parameters AND variables declared in the function body
        let function_scope = self.scope_analyzer.get_all_scopes()
            .iter()
            .find(|scope| {
                scope.scope_level == 1 && {
                    let symbols = scope.symbols.borrow();
                    // Check if this scope contains all the function's parameters
                    // (it can also contain other symbols like variables)
                    func_def.parameters.iter().all(|param| symbols.contains_key(&param.name))
                }
            })
            .cloned();

        // Save current scope and set to function scope
        let saved_scope = self.current_scope.clone();
        if let Some(func_scope) = function_scope {
            self.current_scope = Some(func_scope);
        }

        // Check function body
        let saved_in_loop = self.in_loop;
        self.in_loop = false;

        let mut has_return = false;
        for stmt in &func_def.body {
            if self.check_statement(stmt) {
                has_return = true;
            }
        }

        // Check if non-void function has return statement
        if let Some(ref ret_type) = self.current_return_type {
            if *ret_type != Type::Void && !has_return {
                self.record_error(TypeChkError::ReturnStmtNotFound, &func_def.name);
            }
        }

        self.in_loop = saved_in_loop;
        self.current_return_type = None;
        self.current_scope = saved_scope;
    }

    fn check_statement(&mut self, stmt: &Statement) -> bool {
        // Returns true if statement is a return statement
        match stmt {
            Statement::Declaration(var_decl) => {
                self.check_variable_declaration(var_decl);
                false
            }
            Statement::Assignment(var_name, expr) => {
                // Get variable type from symbol table
                if let Some(var_type) = self.get_variable_type(var_name) {
                    if let Some(expr_type) = self.check_expression(expr) {
                        if expr_type != Type::Unknown && !self.are_types_compatible(&var_type, &expr_type) {
                            self.record_error(TypeChkError::ExpressionTypeMismatch, var_name);
                        }
                    }
                    // If check_expression returns None, error was already reported
                }
                false
            }
            Statement::Return(expr_opt) => {
                if let Some(ret_type) = &self.current_return_type {
                    let ret_type_clone = ret_type.clone();
                    if ret_type_clone == Type::Void {
                        // Void function should not return a value
                        if expr_opt.is_some() {
                            self.record_error(TypeChkError::ErroneousReturnType, "return");
                        }
                    } else {
                        // Non-void function must return a value
                        if let Some(expr) = expr_opt {
                            if let Some(expr_type) = self.check_expression(expr) {
                                if expr_type != Type::Unknown && !self.are_types_compatible(&ret_type_clone, &expr_type) {
                                    self.record_error(TypeChkError::ErroneousReturnType, "return");
                                }
                            }
                            // If check_expression returns None, error was already reported
                        } else {
                            self.record_error(TypeChkError::ErroneousReturnType, "return");
                        }
                    }
                }
                true
            }
            Statement::Expression(expr) => {
                self.check_expression(expr);
                false
            }
            Statement::Block(statements) => {
                // Enter block scope - find child scope of current scope
                let saved_scope = self.current_scope.clone();
                if let Some(current) = &self.current_scope {
                    // Find a child scope (one level deeper)
                    let child_scope = self.scope_analyzer.get_all_scopes()
                        .iter()
                        .find(|scope| {
                            scope.scope_level == current.scope_level + 1 &&
                            scope.parent.as_ref().map(|p| Rc::ptr_eq(p, current)).unwrap_or(false)
                        })
                        .cloned();
                    if let Some(child) = child_scope {
                        self.current_scope = Some(child);
                    }
                }

                let mut has_return = false;
                for stmt in statements {
                    if self.check_statement(stmt) {
                        has_return = true;
                    }
                }

                // Restore previous scope
                self.current_scope = saved_scope;
                has_return
            }
            Statement::If(condition, then_stmt, else_stmt) => {
                // Condition must be boolean
                if let Some(cond_type) = self.check_expression(condition) {
                    if cond_type != Type::Bool {
                        self.record_error(TypeChkError::NonBooleanCondStmt, "if");
                    }
                }
                // If check_expression returns None, error was already reported

                let then_returns = self.check_statement(then_stmt);
                let else_returns = if let Some(else_stmt) = else_stmt {
                    self.check_statement(else_stmt)
                } else {
                    false
                };
                then_returns && else_returns
            }
            Statement::While(condition, body) => {
                // Condition must be boolean
                if let Some(cond_type) = self.check_expression(condition) {
                    if cond_type != Type::Bool {
                        self.record_error(TypeChkError::NonBooleanCondStmt, "while");
                    }
                }
                // If check_expression returns None, error was already reported

                let saved_in_loop = self.in_loop;
                self.in_loop = true;
                self.check_statement(body);
                self.in_loop = saved_in_loop;
                false
            }
            Statement::For(init, condition, update, body) => {
                // Enter for loop scope
                let saved_scope = self.current_scope.clone();
                if let Some(current) = &self.current_scope {
                    let for_scope = self.scope_analyzer.get_all_scopes()
                        .iter()
                        .find(|scope| {
                            scope.scope_level == current.scope_level + 1 &&
                            scope.parent.as_ref().map(|p| Rc::ptr_eq(p, current)).unwrap_or(false)
                        })
                        .cloned();
                    if let Some(scope) = for_scope {
                        self.current_scope = Some(scope);
                    }
                }

                // Check initialization
                if let Some(init_stmt) = init {
                    self.check_statement(init_stmt);
                }

                // Condition must be boolean (if present)
                if let Some(cond) = condition {
                    if let Some(cond_type) = self.check_expression(cond) {
                        if cond_type != Type::Bool {
                            self.record_error(TypeChkError::NonBooleanCondStmt, "for");
                        }
                    }
                    // If check_expression returns None, error was already reported
                }

                // Check update
                if let Some(update_expr) = update {
                    self.check_expression(update_expr);
                }

                // Check body
                let saved_in_loop = self.in_loop;
                self.in_loop = true;
                self.check_statement(body);
                self.in_loop = saved_in_loop;

                // Restore previous scope
                self.current_scope = saved_scope;
                false
            }
            Statement::Break => {
                if !self.in_loop {
                    self.record_error(TypeChkError::ErroneousBreak, "break");
                }
                false
            }
        }
    }

    fn check_expression(&mut self, expr: &Expression) -> Option<Type> {
        match expr {
            Expression::Identifier(name) => {
                // If variable not found, return Unknown (scope analyzer should have caught this)
                self.get_variable_type(name).or(Some(Type::Unknown))
            }
            Expression::Constant(constant) => {
                Some(self.constant_to_type(constant))
            }
            Expression::StringLiteral(_) => {
                // String literals are char arrays/pointers, not simple char values
                Some(Type::String)
            }
            Expression::BinaryOp(left, op, right) => {
                self.check_binary_operation(left, op, right)
            }
            Expression::UnaryOp(op, expr) => {
                self.check_unary_operation(op, expr)
            }
            Expression::Assignment(left, op, right) => {
                self.check_assignment_operation(left, op, right)
            }
            Expression::Conditional(condition, true_expr, false_expr) => {
                self.check_conditional_expression(condition, true_expr, false_expr)
            }
            Expression::FunctionCall(name, args) => {
                self.check_function_call(name, args)
            }
            Expression::ArrayAccess(array, index) => {
                self.check_array_access(array, index)
            }
            Expression::MemberAccess(obj, _member) => {
                self.check_expression(obj)
            }
            Expression::PointerAccess(ptr, _member) => {
                self.check_expression(ptr)
            }
            Expression::PostfixOp(expr, _op) => {
                self.check_expression(expr)
            }
            Expression::Cast(target_type, expr) => {
                if let Some(_expr_type) = self.check_expression(expr) {
                    Some(self.type_specifier_to_type(target_type))
                } else {
                    None
                }
            }
        }
    }

    fn check_binary_operation(&mut self, left: &Expression, op: &BinaryOperator, right: &Expression) -> Option<Type> {
        let left_type = match self.check_expression(left) {
            Some(t) => t,
            None => return None, // Error already reported
        };
        let right_type = match self.check_expression(right) {
            Some(t) => t,
            None => return None, // Error already reported
        };

        match op {
            // Arithmetic operators (require numeric types)
            BinaryOperator::Plus | BinaryOperator::Minus | BinaryOperator::Mult | BinaryOperator::Div => {
                if !self.is_numeric_type(&left_type) || !self.is_numeric_type(&right_type) {
                    self.record_error(TypeChkError::AttemptedAddOpOnNonNumeric, "+");
                    return Some(Type::Unknown); // Return Unknown type but continue checking
                }
                // Result type is the "wider" type
                Some(self.wider_type(&left_type, &right_type))
            }
            BinaryOperator::Mod => {
                // Modulo requires integer types
                if !self.is_integer_type(&left_type) || !self.is_integer_type(&right_type) {
                    self.record_error(TypeChkError::AttemptedAddOpOnNonNumeric, "%");
                    return Some(Type::Unknown);
                }
                Some(left_type)
            }
            // Comparison operators (return boolean)
            BinaryOperator::Less | BinaryOperator::LessEq | BinaryOperator::Greater | BinaryOperator::GreaterEq => {
                if !self.is_numeric_type(&left_type) || !self.is_numeric_type(&right_type) {
                    self.record_error(TypeChkError::ExpressionTypeMismatch, "comparison");
                    return Some(Type::Unknown);
                }
                Some(Type::Bool)
            }
            BinaryOperator::Equals | BinaryOperator::NotEquals => {
                // Equality can work on any compatible types
                if !self.are_types_compatible(&left_type, &right_type) {
                    self.record_error(TypeChkError::ExpressionTypeMismatch, "==");
                    return Some(Type::Unknown);
                }
                Some(Type::Bool)
            }
            // Logical operators (require boolean operands)
            BinaryOperator::And | BinaryOperator::Or => {
                if left_type != Type::Bool || right_type != Type::Bool {
                    self.record_error(TypeChkError::AttemptedBoolOpOnNonBools, "&&");
                    return Some(Type::Unknown);
                }
                Some(Type::Bool)
            }
            // Bitwise operators (require integer types)
            BinaryOperator::BitAnd | BinaryOperator::BitOr | BinaryOperator::Xor => {
                if !self.is_integer_type(&left_type) || !self.is_integer_type(&right_type) {
                    self.record_error(TypeChkError::AttemptedBitOpOnNonNumeric, "&");
                    return Some(Type::Unknown);
                }
                Some(left_type)
            }
            // Shift operators (require integer types)
            BinaryOperator::LShift | BinaryOperator::RShift => {
                if !self.is_integer_type(&left_type) || !self.is_integer_type(&right_type) {
                    self.record_error(TypeChkError::AttemptedShiftOnNonInt, "<<");
                    return Some(Type::Unknown);
                }
                Some(left_type)
            }
        }
    }

    fn check_unary_operation(&mut self, op: &UnaryOperator, expr: &Expression) -> Option<Type> {
        let expr_type = match self.check_expression(expr) {
            Some(t) => t,
            None => return None, // Error already reported
        };

        match op {
            UnaryOperator::Plus | UnaryOperator::Minus => {
                if !self.is_numeric_type(&expr_type) {
                    self.record_error(TypeChkError::AttemptedAddOpOnNonNumeric, "unary +/-");
                    return Some(Type::Unknown);
                }
                Some(expr_type)
            }
            UnaryOperator::Not => {
                if expr_type != Type::Bool {
                    self.record_error(TypeChkError::AttemptedBoolOpOnNonBools, "!");
                    return Some(Type::Unknown);
                }
                Some(Type::Bool)
            }
            UnaryOperator::BitNot => {
                if !self.is_integer_type(&expr_type) {
                    self.record_error(TypeChkError::AttemptedBitOpOnNonNumeric, "~");
                    return Some(Type::Unknown);
                }
                Some(expr_type)
            }
            UnaryOperator::AddressOf | UnaryOperator::Dereference => {
                // Pointer operations - simplified, return the type
                Some(expr_type)
            }
            UnaryOperator::PreIncrement | UnaryOperator::PreDecrement => {
                if !self.is_numeric_type(&expr_type) {
                    self.record_error(TypeChkError::AttemptedAddOpOnNonNumeric, "++/--");
                    return Some(Type::Unknown);
                }
                Some(expr_type)
            }
        }
    }

    fn check_assignment_operation(&mut self, left: &Expression, op: &AssignmentOperator, right: &Expression) -> Option<Type> {
        let left_type = match self.check_expression(left) {
            Some(t) => t,
            None => return None, // Error already reported
        };
        let right_type = match self.check_expression(right) {
            Some(t) => t,
            None => return None, // Error already reported
        };

        match op {
            AssignmentOperator::Assign => {
                if !self.are_types_compatible(&left_type, &right_type) {
                    self.record_error(TypeChkError::ExpressionTypeMismatch, "=");
                    return Some(Type::Unknown);
                }
                Some(left_type)
            }
            AssignmentOperator::PlusAssign | AssignmentOperator::MinusAssign |
            AssignmentOperator::MultAssign | AssignmentOperator::DivAssign => {
                if !self.is_numeric_type(&left_type) || !self.is_numeric_type(&right_type) {
                    self.record_error(TypeChkError::AttemptedAddOpOnNonNumeric, "+= etc");
                    return Some(Type::Unknown);
                }
                Some(left_type)
            }
            AssignmentOperator::ModAssign => {
                if !self.is_integer_type(&left_type) || !self.is_integer_type(&right_type) {
                    self.record_error(TypeChkError::AttemptedAddOpOnNonNumeric, "%=");
                    return Some(Type::Unknown);
                }
                Some(left_type)
            }
            AssignmentOperator::LShiftAssign | AssignmentOperator::RShiftAssign => {
                if !self.is_integer_type(&left_type) || !self.is_integer_type(&right_type) {
                    self.record_error(TypeChkError::AttemptedShiftOnNonInt, "<<=");
                    return Some(Type::Unknown);
                }
                Some(left_type)
            }
            AssignmentOperator::AndAssign | AssignmentOperator::OrAssign | AssignmentOperator::XorAssign => {
                if !self.is_integer_type(&left_type) || !self.is_integer_type(&right_type) {
                    self.record_error(TypeChkError::AttemptedBitOpOnNonNumeric, "&= etc");
                    return Some(Type::Unknown);
                }
                Some(left_type)
            }
        }
    }

    fn check_conditional_expression(&mut self, condition: &Expression, true_expr: &Expression, false_expr: &Expression) -> Option<Type> {
        // Condition must be boolean
        let cond_type = match self.check_expression(condition) {
            Some(t) => t,
            None => return Some(Type::Unknown), // Error already reported
        };
        
        if cond_type != Type::Bool {
            self.record_error(TypeChkError::ExpectedBooleanExpression, "?:");
        }

        let true_type = match self.check_expression(true_expr) {
            Some(t) => t,
            None => return Some(Type::Unknown), // Error already reported
        };
        
        let false_type = match self.check_expression(false_expr) {
            Some(t) => t,
            None => return Some(Type::Unknown), // Error already reported
        };

        // Both branches should have compatible types
        if !self.are_types_compatible(&true_type, &false_type) {
            self.record_error(TypeChkError::ExpressionTypeMismatch, "?:");
            return Some(Type::Unknown);
        }

        Some(true_type)
    }

    fn check_function_call(&mut self, name: &str, args: &[Expression]) -> Option<Type> {
        // Look up function in symbol table - functions are always in global scope
        let global_scope = self.scope_analyzer.get_global_scope();
        if let Some(symbol) = global_scope.lookup(name) {
            if let SymbolKind::Function { parameters, return_type, .. } = &symbol.kind {
                // Check parameter count
                if args.len() != parameters.len() {
                    self.record_error(TypeChkError::FnCallParamCount, name);
                    // Still check parameter types for the parameters we have
                }

                // Check parameter types (check up to min of args.len() and parameters.len())
                let min_len = args.len().min(parameters.len());
                for i in 0..min_len {
                    if let Some(arg_type) = self.check_expression(&args[i]) {
                        let param_type = self.string_to_type(&parameters[i].param_type);
                        if arg_type != Type::Unknown && !self.are_types_compatible(&param_type, &arg_type) {
                            self.record_error(TypeChkError::FnCallParamType, name);
                        }
                    }
                    // If check_expression returns None, error was already reported
                }

                // Return function's return type
                Some(self.string_to_type(return_type))
            } else {
                // Not a function
                None
            }
        } else {
            // Function not found (should have been caught by scope analyzer)
            None
        }
    }

    fn check_array_access(&mut self, array: &Expression, index: &Expression) -> Option<Type> {
        // Check that index is integer
        let index_type = match self.check_expression(index) {
            Some(t) => t,
            None => return Some(Type::Unknown), // Error already reported
        };
        
        if !self.is_integer_type(&index_type) {
            self.record_error(TypeChkError::ExpressionTypeMismatch, "[]");
        }

        // Array access returns element type (simplified - assumes array type)
        self.check_expression(array)
    }

    // Helper functions

    fn get_variable_type(&self, name: &str) -> Option<Type> {
        let scope = self.current_scope.as_ref()?;
        if let Some(symbol) = scope.lookup(name) {
            match &symbol.kind {
                SymbolKind::Variable { type_spec, .. } => {
                    Some(self.type_specifier_to_type(type_spec))
                }
                SymbolKind::Parameter { param_type } => {
                    Some(self.string_to_type(param_type))
                }
                _ => None,
            }
        } else {
            None
        }
    }

    fn type_specifier_to_type(&self, spec: &TypeSpecifier) -> Type {
        match spec {
            TypeSpecifier::Int => Type::Int,
            TypeSpecifier::Float => Type::Float,
            TypeSpecifier::Double => Type::Double,
            TypeSpecifier::Char => Type::Char,
            TypeSpecifier::Short => Type::Short,
            TypeSpecifier::Long => Type::Long,
            TypeSpecifier::Void => Type::Void,
            TypeSpecifier::Signed | TypeSpecifier::Unsigned => Type::Int, // Simplified
        }
    }

    fn string_to_type(&self, type_str: &str) -> Type {
        match type_str {
            "int" => Type::Int,
            "float" => Type::Float,
            "double" => Type::Double,
            "char" => Type::Char,
            "short" => Type::Short,
            "long" => Type::Long,
            "void" => Type::Void,
            _ => Type::Unknown,
        }
    }

    fn constant_to_type(&self, constant: &Constant) -> Type {
        match constant {
            Constant::Integer(_) => Type::Int,
            Constant::Float(_) => Type::Float,
            Constant::Char(_) => Type::Char,
        }
    }

    fn is_numeric_type(&self, t: &Type) -> bool {
        matches!(t, Type::Int | Type::Float | Type::Double | Type::Char | Type::Short | Type::Long)
    }

    fn is_integer_type(&self, t: &Type) -> bool {
        matches!(t, Type::Int | Type::Char | Type::Short | Type::Long)
    }

    fn find_line_for_context(&self, context: &str) -> Option<usize> {
        if context.is_empty() {
            return None;
        }
        self.source_lines
            .iter()
            .position(|line| line.contains(context))
            .map(|idx| idx + 1)
    }

    fn record_error(&mut self, kind: TypeChkError, context: &str) {
        let line = self.find_line_for_context(context);
        self.errors.push(TypeError {
            error: kind,
            line,
            context: context.to_string(),
        });
    }

    fn are_types_compatible(&self, t1: &Type, t2: &Type) -> bool {
        // Types are compatible if they're the same
        if t1 == t2 {
            return true;
        }

        // String literals are not compatible with numeric types
        if t1 == &Type::String || t2 == &Type::String {
            // String can only be compatible with String or Char (for char*)
            return t1 == &Type::String && t2 == &Type::String;
        }

        // Allow implicit conversions between numeric types
        if self.is_numeric_type(t1) && self.is_numeric_type(t2) {
            return true;
        }

        false
    }

    fn wider_type(&self, t1: &Type, t2: &Type) -> Type {
        // Return the "wider" type for arithmetic operations
        // Order: Double > Float > Long > Int > Short > Char
        match (t1, t2) {
            (Type::Double, _) | (_, Type::Double) => Type::Double,
            (Type::Float, _) | (_, Type::Float) => Type::Float,
            (Type::Long, _) | (_, Type::Long) => Type::Long,
            (Type::Int, _) | (_, Type::Int) => Type::Int,
            (Type::Short, _) | (_, Type::Short) => Type::Short,
            _ => Type::Char,
        }
    }

    pub fn get_errors(&self) -> &[TypeError] {
        &self.errors
    }

    pub fn has_errors(&self) -> bool {
        !self.errors.is_empty()
    }
}

