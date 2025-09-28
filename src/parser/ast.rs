// ast.rs: Defines the Abstract Syntax Tree (AST) structures for the MiniC parser.

#[derive(Debug, Clone)]
pub struct TranslationUnit {
    pub preprocessor_list: Vec<PreprocessorDirective>,
    pub external_declarations: Vec<ExternalDeclaration>,
}
