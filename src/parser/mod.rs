// mod.rs: Implements the parser logic and entry points for MiniC source code.
pub mod ast;
use ast::*;

pub struct Parser {
	pub tokens: Vec<crate::rules::Token>,
	pub pos: usize,
}

impl Parser {
	pub fn new(tokens: Vec<crate::rules::Token>) -> Self {
		Parser { tokens, pos: 0 }
	}

	pub fn parse_translation_unit(&mut self) -> TranslationUnit {
		// TODO: Implement parsing logic
		TranslationUnit {
			preprocessor_list: vec![],
			external_declarations: vec![],
		}
	}
}
