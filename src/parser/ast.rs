// ast.rs: Defines the Abstract Syntax Tree (AST) structures for the MiniC parser.
#[derive(Debug, Clone)]
pub enum PreprocessorDirective {
	Include(String),
	Define(String, Vec<String>),
	Ifdef(String),
	Ifndef(String),
	Endif,
	Macro(String, Vec<String>, Vec<String>),
	// Extend as needed
}

#[derive(Debug, Clone)]
pub struct TranslationUnit {
	pub preprocessor_list: Vec<PreprocessorDirective>,
	pub external_declarations: Vec<ExternalDeclaration>,
}

#[derive(Debug, Clone)]
pub enum ExternalDeclaration {
	// Extend as you add parsing rules
}
