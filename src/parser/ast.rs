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

#