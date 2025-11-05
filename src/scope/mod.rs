use crate::parser::ast::*;
use std::collections::HashMap;
use std::rc::RC;


#[derive(Debug,Clone)]
pub enum ScopeError{

    UndeclaredVariable(String),
    UndefinedFunctionCalled(String),
    VariableRedefinition(String),
    FunctionPrototypeRedefinition(String),

}


