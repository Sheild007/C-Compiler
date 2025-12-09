use crate::parser::ast::{
    AssignmentOperator, BinaryOperator, Constant, Expression, ExternalDeclaration,
    FunctionDefinition, Initializer, InitializerKind, Statement, TranslationUnit,
    VariableDeclaration,
};
use std::fmt;

#[derive(Debug, Clone)]
pub struct ProgramIr {
    pub globals: Vec<GlobalVariable>,
    pub functions: Vec<FunctionIr>,
}

impl ProgramIr {
    pub fn new() -> Self {
        Self {
            globals: Vec::new(),
            functions: Vec::new(),
        }
    }
}

#[derive(Debug, Clone)]
pub struct GlobalVariable {
    pub name: String,
    pub initializer: Option<ConstantValue>,
}

#[derive(Debug, Clone)]
pub struct FunctionIr {
    pub name: String,
    pub params: Vec<String>,
    pub instructions: Vec<Instruction>,
}

#[derive(Debug, Clone)]
pub enum Instruction {
    Label(String),
    Assign {
        dst: Target,
        src: Operand,
    },
    Binary {
        dst: Target,
        op: BinaryOp,
        left: Operand,
        right: Operand,
    },
    Unary {
        dst: Target,
        op: UnaryOp,
        operand: Operand,
    },
    Param(Operand),
    Call {
        dst: Option<Target>,
        name: String,
        arg_count: usize,
    },
    Return(Option<Operand>),
    Goto(String),
    IfZeroGoto {
        cond: Operand,
        label: String,
    },
}

#[derive(Debug, Clone)]
pub enum Target {
    Temp(String),
    Var(String),
}

#[derive(Debug, Clone)]
pub enum Operand {
    Temp(String),
    Var(String),
    Const(ConstantValue),
}

#[derive(Debug, Clone)]
pub enum ConstantValue {
    Int(i64),
    Float(f64),
    Char(char),
    String(String),
}

#[derive(Debug, Clone)]
pub enum BinaryOp {
    Add,
    Sub,
    Mul,
    Div,
    Mod,
    Less,
    LessEq,
    Greater,
    GreaterEq,
    Equals,
    NotEquals,
    LogicalAnd,
    LogicalOr,
    BitAnd,
    BitOr,
    BitXor,
    LShift,
    RShift,
}

#[derive(Debug, Clone)]
pub enum UnaryOp {
    Negate,
    LogicalNot,
    BitNot,
}

#[derive(Debug, Clone)]
pub enum IrError {
    UnsupportedExpression(String),
    UnsupportedStatement(String),
    InvalidAssignmentTarget,
    BreakOutsideLoop,
    InvalidGlobalInitializer(String),
}

impl fmt::Display for IrError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            IrError::UnsupportedExpression(msg) => {
                write!(f, "Unsupported expression encountered: {}", msg)
            }
            IrError::UnsupportedStatement(msg) => {
                write!(f, "Unsupported statement encountered: {}", msg)
            }
            IrError::InvalidAssignmentTarget => write!(f, "Invalid assignment target"),
            IrError::BreakOutsideLoop => write!(f, "Break statement used outside of loop"),
            IrError::InvalidGlobalInitializer(name) => write!(
                f,
                "Global variable '{}' requires a constant initializer",
                name
            ),
        }
    }
}

// Display implementations for LLVM-like IR output

impl fmt::Display for Target {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Target::Temp(name) => write!(f, "%{}", name),
            Target::Var(name) => write!(f, "{}", name),
        }
    }
}

impl fmt::Display for Operand {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Operand::Temp(name) => write!(f, "%{}", name),
            Operand::Var(name) => write!(f, "{}", name),
            Operand::Const(val) => write!(f, "{}", val),
        }
    }
}

impl fmt::Display for ConstantValue {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ConstantValue::Int(n) => write!(f, "{}", n),
            ConstantValue::Float(n) => write!(f, "{}", n),
            ConstantValue::Char(c) => write!(f, "'{}'", c),
            ConstantValue::String(s) => write!(f, "\"{}\"", s),
        }
    }
}

impl ConstantValue {
    pub fn type_name(&self) -> &'static str {
        match self {
            ConstantValue::Int(_) => "i32",
            ConstantValue::Float(_) => "f32",
            ConstantValue::Char(_) => "i8",
            ConstantValue::String(_) => "str",
        }
    }
}

impl fmt::Display for BinaryOp {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let op_str = match self {
            BinaryOp::Add => "+",
            BinaryOp::Sub => "-",
            BinaryOp::Mul => "*",
            BinaryOp::Div => "/",
            BinaryOp::Mod => "%",
            BinaryOp::Less => "<",
            BinaryOp::LessEq => "<=",
            BinaryOp::Greater => ">",
            BinaryOp::GreaterEq => ">=",
            BinaryOp::Equals => "==",
            BinaryOp::NotEquals => "!=",
            BinaryOp::LogicalAnd => "&&",
            BinaryOp::LogicalOr => "||",
            BinaryOp::BitAnd => "&",
            BinaryOp::BitOr => "|",
            BinaryOp::BitXor => "^",
            BinaryOp::LShift => "<<",
            BinaryOp::RShift => ">>",
        };
        write!(f, "{}", op_str)
    }
}

impl fmt::Display for UnaryOp {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let op_str = match self {
            UnaryOp::Negate => "-",
            UnaryOp::LogicalNot => "!",
            UnaryOp::BitNot => "~",
        };
        write!(f, "{}", op_str)
    }
}

impl fmt::Display for Instruction {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Instruction::Label(name) => write!(f, "{}:", name),
            Instruction::Assign { dst, src } => write!(f, "{} = {}", dst, src),
            Instruction::Binary {
                dst,
                op,
                left,
                right,
            } => write!(f, "{} = {} {} {}", dst, left, op, right),
            Instruction::Unary { dst, op, operand } => write!(f, "{} = {}{}", dst, op, operand),
            Instruction::Param(_operand) => Ok(()), // Params handled in Call
            Instruction::Call {
                dst,
                name,
                arg_count: _,
            } => {
                if let Some(target) = dst {
                    write!(f, "{} = call @{}", target, name)
                } else {
                    write!(f, "call @{}", name)
                }
            }
            Instruction::Return(operand) => {
                if let Some(val) = operand {
                    write!(f, "ret {}", val)
                } else {
                    write!(f, "ret")
                }
            }
            Instruction::Goto(label) => write!(f, "goto {}", label),
            Instruction::IfZeroGoto { cond, label } => {
                write!(f, "if {} == 0 goto {}", cond, label)
            }
        }
    }
}

pub struct IrGenerator {
    temp_counter: usize,
    label_counter: usize,
    errors: Vec<IrError>,
}

impl IrGenerator {
    pub fn new() -> Self {
        Self {
            temp_counter: 0,
            label_counter: 0,
            errors: Vec::new(),
        }
    }

    pub fn generate(&mut self, unit: &TranslationUnit) -> Result<ProgramIr, Vec<IrError>> {
        self.errors.clear();
        let mut program = ProgramIr::new();

        for decl in &unit.external_declarations {
            match decl {
                ExternalDeclaration::Variable(var_decl) => match self.build_global(var_decl) {
                    Ok(global) => program.globals.push(global),
                    Err(err) => self.errors.push(err),
                },
                ExternalDeclaration::Function(func_def) => {
                    let ir_function = self.build_function(func_def);
                    program.functions.push(ir_function);
                }
                ExternalDeclaration::FunctionDeclaration(_decl) => {
                    // Function prototypes don't emit IR
                }
            }
        }

        if self.errors.is_empty() {
            Ok(program)
        } else {
            Err(self.errors.clone())
        }
    }

    fn build_global(&mut self, var_decl: &VariableDeclaration) -> Result<GlobalVariable, IrError> {
        let name = var_decl.declarator.name.clone();
        if let Some(initializer) = &var_decl.initializer {
            if let Some(value) = self.constant_from_initializer(initializer) {
                Ok(GlobalVariable {
                    name,
                    initializer: Some(value),
                })
            } else {
                Err(IrError::InvalidGlobalInitializer(name))
            }
        } else {
            Ok(GlobalVariable {
                name,
                initializer: None,
            })
        }
    }

    fn build_function(&mut self, func_def: &FunctionDefinition) -> FunctionIr {
        let mut ctx = FunctionContext::new();
        let params = func_def
            .parameters
            .iter()
            .map(|p| p.name.clone())
            .collect::<Vec<_>>();

        for stmt in &func_def.body {
            self.emit_statement(stmt, &mut ctx);
        }

        FunctionIr {
            name: func_def.name.clone(),
            params,
            instructions: ctx.instructions,
        }
    }

    fn emit_statement(&mut self, stmt: &Statement, ctx: &mut FunctionContext) {
        match stmt {
            Statement::Declaration(var_decl) => {
                if let Some(initializer) = &var_decl.initializer {
                    if let Some(expr) = self.initializer_expression(initializer) {
                        if let Some(value) = self.emit_expression(&expr, ctx) {
                            ctx.instructions.push(Instruction::Assign {
                                dst: Target::Var(var_decl.declarator.name.clone()),
                                src: value,
                            });
                        }
                    } else {
                        self.errors.push(IrError::UnsupportedStatement(format!(
                            "complex initializer for '{}'",
                            var_decl.declarator.name
                        )));
                    }
                }
            }
            Statement::Assignment(name, expr) => {
                if let Some(value) = self.emit_expression(expr, ctx) {
                    ctx.instructions.push(Instruction::Assign {
                        dst: Target::Var(name.clone()),
                        src: value,
                    });
                }
            }
            Statement::Return(expr_opt) => {
                if let Some(expr) = expr_opt {
                    let value = self.emit_expression(expr, ctx);
                    ctx.instructions.push(Instruction::Return(value));
                } else {
                    ctx.instructions.push(Instruction::Return(None));
                }
            }
            Statement::Expression(expr) => {
                self.emit_expression(expr, ctx);
            }
            Statement::Block(statements) => {
                for inner in statements {
                    self.emit_statement(inner, ctx);
                }
            }
            Statement::If(condition, then_stmt, else_stmt) => {
                if let Some(cond_value) = self.emit_expression(condition, ctx) {
                    let else_label = self.new_label("if_else");
                    let end_label = self.new_label("if_end");
                    ctx.instructions.push(Instruction::IfZeroGoto {
                        cond: cond_value,
                        label: else_label.clone(),
                    });
                    self.emit_statement(then_stmt, ctx);
                    ctx.instructions.push(Instruction::Goto(end_label.clone()));
                    ctx.instructions.push(Instruction::Label(else_label));
                    if let Some(else_branch) = else_stmt {
                        self.emit_statement(else_branch, ctx);
                    }
                    ctx.instructions.push(Instruction::Label(end_label));
                }
            }
            Statement::While(condition, body) => {
                let cond_label = self.new_label("while_cond");
                let body_label = self.new_label("while_body");
                let end_label = self.new_label("while_end");

                ctx.instructions
                    .push(Instruction::Label(cond_label.clone()));
                let cond_value = self.emit_expression(condition, ctx);
                if let Some(cond_operand) = cond_value {
                    ctx.instructions.push(Instruction::IfZeroGoto {
                        cond: cond_operand,
                        label: end_label.clone(),
                    });
                    ctx.loop_end_labels.push(end_label.clone());
                    ctx.instructions
                        .push(Instruction::Label(body_label.clone()));
                    self.emit_statement(body, ctx);
                    ctx.loop_end_labels.pop();
                    ctx.instructions.push(Instruction::Goto(cond_label));
                    ctx.instructions.push(Instruction::Label(end_label));
                }
            }
            Statement::For(init, condition, update, body) => {
                if let Some(init_stmt) = init {
                    self.emit_statement(init_stmt, ctx);
                }
                let cond_label = self.new_label("for_cond");
                let body_label = self.new_label("for_body");
                let end_label = self.new_label("for_end");

                ctx.instructions
                    .push(Instruction::Label(cond_label.clone()));
                if let Some(cond_expr) = condition {
                    if let Some(cond_value) = self.emit_expression(cond_expr, ctx) {
                        ctx.instructions.push(Instruction::IfZeroGoto {
                            cond: cond_value,
                            label: end_label.clone(),
                        });
                    }
                }
                ctx.loop_end_labels.push(end_label.clone());
                ctx.instructions.push(Instruction::Label(body_label));
                self.emit_statement(body, ctx);
                ctx.loop_end_labels.pop();
                if let Some(update_expr) = update {
                    self.emit_expression(update_expr, ctx);
                }
                ctx.instructions.push(Instruction::Goto(cond_label));
                ctx.instructions.push(Instruction::Label(end_label));
            }
            Statement::Break => {
                if let Some(label) = ctx.loop_end_labels.last() {
                    ctx.instructions.push(Instruction::Goto(label.clone()));
                } else {
                    self.errors.push(IrError::BreakOutsideLoop);
                }
            }
        }
    }

    fn emit_expression(&mut self, expr: &Expression, ctx: &mut FunctionContext) -> Option<Operand> {
        match expr {
            Expression::Identifier(name) => Some(Operand::Var(name.clone())),
            Expression::Constant(c) => Some(Operand::Const(self.constant_from_ast(c))),
            Expression::StringLiteral(s) => Some(Operand::Const(ConstantValue::String(s.clone()))),
            Expression::BinaryOp(left, op, right) => {
                let left_val = self.emit_expression(left, ctx)?;
                let right_val = self.emit_expression(right, ctx)?;
                if let Some(mapped) = self.map_binary_op(op) {
                    let temp = self.new_temp();
                    ctx.instructions.push(Instruction::Binary {
                        dst: Target::Temp(temp.clone()),
                        op: mapped,
                        left: left_val,
                        right: right_val,
                    });
                    Some(Operand::Temp(temp))
                } else {
                    self.errors
                        .push(IrError::UnsupportedExpression(format!("{:?}", op)));
                    None
                }
            }
            Expression::UnaryOp(op, inner) => match op {
                crate::parser::ast::UnaryOperator::Plus => self.emit_expression(inner, ctx),
                crate::parser::ast::UnaryOperator::Minus => {
                    let operand = self.emit_expression(inner, ctx)?;
                    let temp = self.new_temp();
                    ctx.instructions.push(Instruction::Unary {
                        dst: Target::Temp(temp.clone()),
                        op: UnaryOp::Negate,
                        operand,
                    });
                    Some(Operand::Temp(temp))
                }
                crate::parser::ast::UnaryOperator::Not => {
                    let operand = self.emit_expression(inner, ctx)?;
                    let temp = self.new_temp();
                    ctx.instructions.push(Instruction::Unary {
                        dst: Target::Temp(temp.clone()),
                        op: UnaryOp::LogicalNot,
                        operand,
                    });
                    Some(Operand::Temp(temp))
                }
                crate::parser::ast::UnaryOperator::BitNot => {
                    let operand = self.emit_expression(inner, ctx)?;
                    let temp = self.new_temp();
                    ctx.instructions.push(Instruction::Unary {
                        dst: Target::Temp(temp.clone()),
                        op: UnaryOp::BitNot,
                        operand,
                    });
                    Some(Operand::Temp(temp))
                }
                crate::parser::ast::UnaryOperator::PreIncrement
                | crate::parser::ast::UnaryOperator::PreDecrement
                | crate::parser::ast::UnaryOperator::AddressOf
                | crate::parser::ast::UnaryOperator::Dereference => {
                    self.errors
                        .push(IrError::UnsupportedExpression(format!("{:?}", op)));
                    None
                }
            },
            Expression::Assignment(left, assign_op, right) => {
                let rhs = self.emit_expression(right, ctx)?;
                match (**left).clone() {
                    Expression::Identifier(name) => match assign_op {
                        AssignmentOperator::Assign => {
                            ctx.instructions.push(Instruction::Assign {
                                dst: Target::Var(name.clone()),
                                src: rhs.clone(),
                            });
                            Some(Operand::Var(name))
                        }
                        _ => {
                            self.errors.push(IrError::UnsupportedExpression(format!(
                                "compound assignment {:?}",
                                assign_op
                            )));
                            None
                        }
                    },
                    _ => {
                        self.errors.push(IrError::InvalidAssignmentTarget);
                        None
                    }
                }
            }
            Expression::Conditional(cond, true_expr, false_expr) => {
                let cond_val = self.emit_expression(cond, ctx)?;
                let true_label = self.new_label("cond_true");
                let false_label = self.new_label("cond_false");
                let end_label = self.new_label("cond_end");
                let temp = self.new_temp();

                ctx.instructions.push(Instruction::IfZeroGoto {
                    cond: cond_val,
                    label: false_label.clone(),
                });
                ctx.instructions
                    .push(Instruction::Label(true_label.clone()));
                if let Some(true_val) = self.emit_expression(true_expr, ctx) {
                    ctx.instructions.push(Instruction::Assign {
                        dst: Target::Temp(temp.clone()),
                        src: true_val,
                    });
                }
                ctx.instructions.push(Instruction::Goto(end_label.clone()));
                ctx.instructions.push(Instruction::Label(false_label));
                if let Some(false_val) = self.emit_expression(false_expr, ctx) {
                    ctx.instructions.push(Instruction::Assign {
                        dst: Target::Temp(temp.clone()),
                        src: false_val,
                    });
                }
                ctx.instructions.push(Instruction::Label(end_label));
                Some(Operand::Temp(temp))
            }
            Expression::FunctionCall(name, args) => {
                for arg in args {
                    if let Some(value) = self.emit_expression(arg, ctx) {
                        ctx.instructions.push(Instruction::Param(value));
                    }
                }
                let temp = self.new_temp();
                ctx.instructions.push(Instruction::Call {
                    dst: Some(Target::Temp(temp.clone())),
                    name: name.clone(),
                    arg_count: args.len(),
                });
                Some(Operand::Temp(temp))
            }
            Expression::ArrayAccess(_, _)
            | Expression::MemberAccess(_, _)
            | Expression::PointerAccess(_, _)
            | Expression::PostfixOp(_, _) => {
                self.errors
                    .push(IrError::UnsupportedExpression(format!("{:?}", expr)));
                None
            }
            Expression::Cast(_, inner) => self.emit_expression(inner, ctx),
        }
    }

    fn map_binary_op(&self, op: &BinaryOperator) -> Option<BinaryOp> {
        use BinaryOperator::*;
        Some(match op {
            Plus => BinaryOp::Add,
            Minus => BinaryOp::Sub,
            Mult => BinaryOp::Mul,
            Div => BinaryOp::Div,
            Mod => BinaryOp::Mod,
            Less => BinaryOp::Less,
            LessEq => BinaryOp::LessEq,
            Greater => BinaryOp::Greater,
            GreaterEq => BinaryOp::GreaterEq,
            Equals => BinaryOp::Equals,
            NotEquals => BinaryOp::NotEquals,
            And => BinaryOp::LogicalAnd,
            Or => BinaryOp::LogicalOr,
            BitAnd => BinaryOp::BitAnd,
            BitOr => BinaryOp::BitOr,
            Xor => BinaryOp::BitXor,
            LShift => BinaryOp::LShift,
            RShift => BinaryOp::RShift,
        })
    }

    fn constant_from_ast(&self, constant: &Constant) -> ConstantValue {
        match constant {
            Constant::Integer(i) => ConstantValue::Int(*i),
            Constant::Float(f) => ConstantValue::Float(*f),
            Constant::Char(c) => ConstantValue::Char(*c),
        }
    }

    fn constant_from_initializer(&self, initializer: &Initializer) -> Option<ConstantValue> {
        match &initializer.kind {
            InitializerKind::Assignment(expr) => self.constant_from_expression(expr),
            _ => None,
        }
    }

    fn constant_from_expression(&self, expr: &Expression) -> Option<ConstantValue> {
        match expr {
            Expression::Constant(c) => Some(self.constant_from_ast(c)),
            Expression::StringLiteral(s) => Some(ConstantValue::String(s.clone())),
            _ => None,
        }
    }

    fn new_temp(&mut self) -> String {
        self.temp_counter += 1;
        format!("t{}", self.temp_counter)
    }

    fn new_label(&mut self, prefix: &str) -> String {
        self.label_counter += 1;
        format!("{}_{}", prefix, self.label_counter)
    }

    fn initializer_expression(&self, initializer: &Initializer) -> Option<Expression> {
        match &initializer.kind {
            InitializerKind::Assignment(expr) => Some(expr.clone()),
            _ => None,
        }
    }
}

struct FunctionContext {
    instructions: Vec<Instruction>,
    loop_end_labels: Vec<String>,
}

impl FunctionContext {
    fn new() -> Self {
        Self {
            instructions: Vec::new(),
            loop_end_labels: Vec::new(),
        }
    }
}
