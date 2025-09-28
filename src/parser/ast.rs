// ast.rs: Defines the Abstract Syntax Tree (AST) structures for the MiniC parser.

#[derive(Debug, Clone)]
pub struct TranslationUnit {
    pub preprocessor_list: Vec<PreprocessorDirective>,
    pub external_declarations: Vec<ExternalDeclaration>,
}
#[derive(Debug, Clone)]
pub enum PreprocessorDirective {
    Include(String), // #include <stdio.h>
    Define(String, Vec<ReplacementItem>), // #define IDENTIFIER replacement_list
    Ifdef(String), // #ifdef IDENTIFIER
    Ifndef(String), // #ifndef IDENTIFIER
    Endif, // #endif
}

#[derive(Debug, Clone)]
pub enum ReplacementItem {
    Identifier(String), // Identifier in replacement_list
    Constant(Constant), // Constant in replacement_list
    StringLiteral(String), // StringLiteral in replacement_list
}

#[derive(Debug, Clone)]
pub enum Constant {
    Integer(i64), // e.g., 42
    Float(f64),   // e.g., 3.14
    Char(char),   // e.g., 'a'
}

#[derive(Debug, Clone)]
pub enum ExternalDeclaration {
    Printf(PrintfStatement), // printf(...);
    Struct(StructDeclaration), // struct ...;
    Union(UnionDeclaration), // union ...;
    Enum(EnumDeclaration), // enum ...;
    Typedef(TypedefDeclaration), // typedef ...;
    Variable(VariableDeclaration), // int x = 5;
    Function(FunctionDefinition), // int function_name(...) { ... }
    FunctionDeclaration(FunctionDeclaration), // int func(int x);
}

#[derive(Debug, Clone)]
pub struct PrintfStatement {
    pub args: Vec<PrintfArg>, // printf_args
}

#[derive(Debug, Clone)]
pub struct FunctionDefinition {
    pub return_type: String, // e.g., "int", "void"
    pub name: String, // function name
    pub parameters: Vec<Parameter>, // function parameters
    pub body: Vec<Statement>, // function body statements
}

#[derive(Debug, Clone)]
pub struct Parameter {
    pub param_type: String, // e.g., "int", "float"
    pub name: String, // parameter name
}

#[derive(Debug, Clone)]
pub enum Statement {
    Declaration(VariableDeclaration), // int x = 5;
    Assignment(String, Expression), // variable_name, expression
    Return(Option<Expression>), // return statement
    Expression(Expression), // expression statement
    Block(Vec<Statement>), // { ... } block
    If(Expression, Box<Statement>, Option<Box<Statement>>), // if (cond) stmt [else stmt]
    While(Expression, Box<Statement>), // while (cond) stmt
    For(Option<Box<Statement>>, Option<Expression>, Option<Expression>, Box<Statement>), // for (init; cond; update) stmt
    DoWhile(Box<Statement>, Expression), // do stmt while (cond);
    Switch(Expression, Vec<Case>), // switch (expr) { cases }
    Break, // break;
    Continue, // continue;
    Goto(String), // goto label;
    Label(String, Box<Statement>), // label: stmt
}
