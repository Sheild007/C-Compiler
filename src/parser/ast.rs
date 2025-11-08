// ast.rs: Defines the Abstract Syntax Tree (AST) structures for the MiniC parser.

#[derive(Debug, Clone)]
pub struct TranslationUnit {
    pub preprocessor_list: Vec<PreprocessorDirective>,
    pub external_declarations: Vec<ExternalDeclaration>,
}

#[derive(Debug, Clone)]
pub enum PreprocessorDirective {
    Include(String),                      // #include <stdio.h>
    Define(String, Vec<ReplacementItem>), // #define IDENTIFIER replacement_list
    Ifdef(String),                        // #ifdef IDENTIFIER
    Ifndef(String),                       // #ifndef IDENTIFIER
    Endif,                                // #endif
}

#[derive(Debug, Clone)]
pub enum ReplacementItem {
    Identifier(String),    // Identifier in replacement_list
    Constant(Constant),    // Constant in replacement_list
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
    Variable(VariableDeclaration),            // int x = 5;
    Function(FunctionDefinition),             // int function_name(...) { ... }
    FunctionDeclaration(FunctionDeclaration), // int func(int x);
}


#[derive(Debug, Clone)]
pub struct FunctionDefinition {
    pub return_type: String,        // e.g., "int", "void"
    pub name: String,               // function name
    pub parameters: Vec<Parameter>, // function parameters
    pub body: Vec<Statement>,       // function body statements
}

#[derive(Debug, Clone)]
pub struct Parameter {
    pub param_type: String, // e.g., "int", "float"
    pub name: String,       // parameter name
}

#[derive(Debug, Clone)]
pub enum Statement {
    Declaration(VariableDeclaration),                       // int x = 5;
    Assignment(String, Expression),                         // variable_name, expression
    Return(Option<Expression>),                             // return statement
    Expression(Expression),                                 // expression statement
    Block(Vec<Statement>),                                  // { ... } block
    If(Expression, Box<Statement>, Option<Box<Statement>>), // if (cond) stmt [else stmt]
    While(Expression, Box<Statement>),                      // while (cond) stmt
    For(
        Option<Box<Statement>>,
        Option<Expression>,
        Option<Expression>,
        Box<Statement>,
    ), // for (init; cond; update) stmt
    Break,                                                  // break;
}



#[derive(Debug, Clone)]
pub enum SpecifierQualifier {
    TypeSpecifier(TypeSpecifier), // type_specifier
    TypeQualifier(TypeQualifier), // type_qualifier
}

#[derive(Debug, Clone)]
pub enum TypeSpecifier {
    Int,
    Float,
    Double,
    Char,
    Short,
    Long,
    Signed,
    Unsigned,
    Void,
}

#[derive(Debug, Clone)]
pub enum TypeQualifier {
    Const,
    // Add more as needed based on grammar expansion
}

#[derive(Debug, Clone)]
pub struct Declarator {
    pub name: String,                            // Identifier in declarator
    pub pointer_depth: u32,                      // number of * before name
    pub array_sizes: Vec<Option<Expression>>,    // array dimensions
    pub function_params: Option<Vec<Parameter>>, // function parameters
}

#[derive(Debug, Clone)]
pub enum Expression {
    Identifier(String),    // Identifier in expression
    Constant(Constant),    // Constant in expression
    StringLiteral(String), // StringLiteral in expression
    BinaryOp(Box<Expression>, BinaryOperator, Box<Expression>), // Binary operations
    UnaryOp(UnaryOperator, Box<Expression>), // Unary operations
    Assignment(Box<Expression>, AssignmentOperator, Box<Expression>), // Assignment operations
    Conditional(Box<Expression>, Box<Expression>, Box<Expression>), // Ternary operator: cond ? true_expr : false_expr
    FunctionCall(String, Vec<Expression>),                          // Function calls: func(args)
    ArrayAccess(Box<Expression>, Box<Expression>),                  // Array access: arr[index]
    MemberAccess(Box<Expression>, String),                          // Member access: obj.member
    PointerAccess(Box<Expression>, String),                         // Pointer access: ptr->member
    PostfixOp(Box<Expression>, PostfixOperator), // Postfix operations: expr++, expr--
    Cast(TypeSpecifier, Box<Expression>),        // (type)expr
}

#[derive(Debug, Clone)]
pub enum BinaryOperator {
    Plus,
    Minus,
    Mult,
    Div,
    Mod,
    Less,
    LessEq,
    Greater,
    GreaterEq,
    Equals,
    NotEquals,
    And,
    Or,
    BitAnd,
    BitOr,
    Xor,
    LShift,
    RShift,
}

#[derive(Debug, Clone)]
pub enum UnaryOperator {
    Plus,
    Minus,
    Not,
    BitNot,
    AddressOf,
    Dereference,
    PreIncrement,
    PreDecrement, // ++expr, --expr
}

#[derive(Debug, Clone)]
pub enum AssignmentOperator {
    Assign,
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
}

#[derive(Debug, Clone)]
pub enum PostfixOperator {
    PlusPlus,
    MinusMinus,
}

#[derive(Debug, Clone)]
pub struct Initializer {
    pub kind: InitializerKind,
}

#[derive(Debug, Clone)]
pub enum InitializerKind {
    Assignment(Expression),                   // assignment_expression
    List(Vec<Initializer>),                   // { initializer_list } or { initializer_list , }
    Designated(Designator, Box<Initializer>), // .field = value
}

#[derive(Debug, Clone)]
pub struct ParameterTypeList {
    pub parameters: Vec<ParameterDeclaration>, // parameter_list
    pub variadic: bool,                        // true if '...' is present
}

#[derive(Debug, Clone)]
pub struct ParameterDeclaration {
    pub specifiers: Vec<SpecifierQualifier>, // declaration_specifiers
    pub declarator: Option<Declarator>,      // declarator or abstract_declarator
}

#[derive(Debug, Clone)]
pub enum Comment {
    Line(String),  // // comment_text \n
    Block(String), // /* comment_text */
}

// ===== MISSING AST STRUCTURES FOR MINI-C =====

#[derive(Debug, Clone)]
pub struct VariableDeclaration {
    pub storage_class: Option<StorageClass>,
    pub type_qualifiers: Vec<TypeQualifier>,
    pub type_specifier: TypeSpecifier,
    pub declarator: Declarator,
    pub initializer: Option<Initializer>,
}


#[derive(Debug, Clone)]
pub struct FunctionDeclaration {
    pub return_type: String,
    pub name: String,
    pub parameters: Vec<Parameter>,
}


#[derive(Debug, Clone)]
pub enum StorageClass {
    Auto,
    Register,
    Static,
    Extern,
    Typedef,
}

#[derive(Debug, Clone)]
pub enum Designator {
    Member(String),    // .field
    Array(Expression), // [index]
}

#[derive(Debug, Clone)]
pub enum ParseError {
    UnexpectedEOF,
    FailedToFindToken(String),
    ExpectedTypeToken,
    ExpectedIdentifier,
    UnexpectedToken(String),
    ExpectedFloatLit,
    ExpectedIntLit,
    ExpectedStringLit,
    ExpectedBoolLit,
    ExpectedExpr,
}
