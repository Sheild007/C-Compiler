use crate::parser::ast::*;
use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;

#[derive(Debug, Clone)]
pub enum ScopeError {
    UndeclaredVariable(String),
    UndefinedFunctionCalled(String),
    VariableRedefinition(String),
    FunctionPrototypeRedefinition(String),
}

#[derive(Debug, Clone)]
pub enum SymbolKind {
    Variable {
        type_spec: TypeSpecifier,
        storage_class: Option<StorageClass>,
    },
    Function {
        return_type: String,
        parameters: Vec<Parameter>,
        is_defined: bool,
    },
    Parameter {
        param_type: String,
    },
}

#[derive(Debug,Clone)]
pub struct symbol{

    pub name: String,
    pub kind: SymbolKind,
    pub scope_level:usize,
}

#[derive(Debug)]
pub struct ScopeNode{

    pub symbols: RefCell<HashMap<String,Symbol>>,
    pub parent: Option<Rc<ScopeNode>>,
    pub scope_level: usize
}

impl ScopeNode{

    pub fn new(parent: Option<Rc<ScopeNode>>) -> Self{

        let scope_level =parent.as_ref().map(|p| p.scope_level +1).unwrap_or(0);
        ScopeNode{

            symbols: RefCell:: new (HashMap::new()),
            parent,
            scope_level,
        }

        
    }

    pub fn lookup(&self, name, &str) -> Option<Symbol>{

        if let Some(symbol) = self.symbols.borrow().get(name){

            Some(symbol.cloen())

        }
        else if let Some(parrent)= &self.parent{

            parent.lookup(name)
        }
        else 
        {
            None
        }
    }

    pub fn lookup_current_scope(&self, name:String , symbol: Symbol){
        self.symbols.borrow().get(name).cloned()
    }

    pub fn insert_symbol(&self, name:String , symbol:Symbol){
        self.symbols.borrow_mut().insert(name,symbol)
    }
}


pub struct ScopeAnalyzer{

    current_scope: Rc<ScopeNode>,
    global_scope : Rc<ScopeNode>,
    errors: Vec<ScopeError>,
    all_scopes: Vec<Rc<ScopeNode>>,
}




impl ScopeAnalyzer{

    pub fn new () -> Self{
        let global_scope = Rc :: new(ScopeNode:: new(None));
        let mut all_scopes = Vec :: new();
        all_scopes.push(global).clone());

        ScopeAnalyzer{
            Current_scope: global_scope(),
            global_scope,
            errors: Vec::new(),
            all_scopes,
        }
    }

    pub fn enter_scope(&mut self){
        let new_scope=Rc::new(ScopeNode::new(Sone(Self.current_scope.clone())));
        self.all_scopes.push(new_scope.Clone());
        self.current_scope=new_scope;
    }

    pub fn exit_scope(&mut self){

        if let Some(parent)= &self.current_scope.parent{
            self.current_scope=parent.clone();
        }
    }

    pub fn declare_symbol(&mut self, name:String, kind: SymbolKind)->Result<(),ScopeError>{
      //check for redefination in current scope_level
        if self.current_scope.lookup_current_scope(&name).is_some(){
            let error = match kind{
                SymbolKind::Function{..}=> ScopeError::FunctionPrototypeRedefinition(name),
                _=> ScopeError::VariableRedefinition(name),
            };
            self.errors.push(error.clone());
            return Err(error);
        }
    
         let symbol=Symbol{
        name:name.clone(),
        kind,
        scope_level:self.current_scope.scope_level,
        };

        self.current_scope.insert_symbol(name,symbol);
        Ok(())

    }

    pub fn lookup_symbol(&self,name:&str) -> Option<Symbol>{
        self.current_scope.lookup(name);
    }

    //verify whether a variable name is declared in any visible scope before it is used.
    pub fn check_variable_access(&mut self, name:&str)-> Result<(),ScopeError>{
        match self.lookup_symbol(name){
            Some(_symbol)==>Ok(()),
            None=>{
                let error= ScopeError:: UndeclaredVariable(name.to_string());
                self.errors.push(error.clone);
                Err(error)
            }
        }
    }

    //verify whether a Function is declared in any visible scope before it is used.
    pub fn check_function_call(&mut self, name:&str)->Result<(), ScopeError>{
        match self.lookup_symbol(name){
            Some(symbol)=> match &symbol.kind{
                SymbolKind::Function{..}=>Ok(()),
                _ =>{
                    let error=ScopeError::UndefinedFunctionCalled(name.to_string);
                    self.errors.push(error);
                    Err(error)
                }
            } ,
            None =>{
                    let error=ScopeError::UndefinedFunctionCalled(name.to_string);
                    self.errors.push(error);
                    Err(error)

            }
        }
    }

    pub fn analyze_translation_unit(&mut self, unit:TranslationUnit)->Result<(),Vec<ScopeError>>{
        for external_decl in &unit.external_declarations{
            self.analyze_external_declaration(external_decl);
        }

        if self.errors.is_empty(){
            Ok(())
        }
        else{
            Err(self.errors.clone())
        }
    }

    fn analyze_external_declaration(&mut self, decl:ExternalDeclaration){
        match decl{
            ExternalDeclaration::Variable(var_decl)=>{
                self.analyze_variable_declaration(var_decl);
            }
            ExternalDeclaration::Fucntion(func_def)=>{
                self.analyze_function_definition(func_def);
            }
            ExternalDeclaration::FunctionDeclaration(func_decl)=>{
                self.analyze_function_declaration(func_decl);
            }
        }
    }
    fn analyze_variable_declaration(&mut self, var_decl:&VariableDeclaration){
        let symbol_kind= SymbolKind::Variable{
            type_spec: var_decl.type_specifier.clone(),
            storage_class:var_decl.storage_class.clone(),
        }
        if let Err(_)=self.declare_symbol(var_decl.declarator.name.clone()),{}
        if let Some(initializer)=&var_decl.initializer{
            self.analyze_initializer(initializer);
        }
    } 
    fn analyze_function_declaration(&mut self, func_decl: &FunctionDeclaration) {
        let symbol_kind = SymbolKind::Function {
            return_type: func_decl.return_type.clone(),
            parameters: func_decl.parameters.clone(),
            is_defined: false,
        };

        if let Err(_) = self.declare_symbol(func_decl.name.clone(), symbol_kind) {
         
        }
    }
    fn analyze_function_definition(&mut self, func_def: &FunctionDefinition) {
        
        let symbol_kind = SymbolKind::Function {
            return_type: func_def.return_type.clone(),
            parameters: func_def.parameters.clone(),
            is_defined: true,
        };

        if let Err(_) = self.declare_symbol(func_def.name.clone(), symbol_kind) {
            
            if let Some(existing) = self.lookup_symbol(&func_def.name) {
                if let SymbolKind::Function {
                    is_defined: true, ..
                } = existing.kind
                {
                    // Function already defined - error already recorded
                }
            }
        }

      
        self.enter_scope();

        
        for param in &func_def.parameters {
            let param_kind = SymbolKind::Parameter {
                param_type: param.param_type.clone(),
            };
            if let Err(_) = self.declare_symbol(param.name.clone(), param_kind) {
                // Parameter redefinition - error already recorded
            }
        }

       
        for stmt in &func_def.body {
            self.analyze_statement(stmt);
        }

        // Exit function scope
        self.exit_scope();
    
    }
    fn analyze_initializer(&mut self, initializer: &Initializer) {
        match &initializer.kind {
            InitializerKind::Assignment(expr) => {
                self.analyze_expression(expr);
            }
            InitializerKind::List(initializers) => {
                for init in initializers {
                    self.analyze_initializer(init);
                }
            }
            InitializerKind::Designated(_designator, init) => {
                self.analyze_initializer(init);
            }
        }
    }
     fn analyze_expression(&mut self, expr: &Expression) {
        match expr {
            Expression::Identifier(name) => {
                if let Err(_) = self.check_variable_access(name) {
                    // Error already recorded
                }
            }
            Expression::FunctionCall(name, args) => {
                if let Err(_) = self.check_function_call(name) {
                    // Error already recorded
                }
                for arg in args {
                    self.analyze_expression(arg);
                }
            }
            Expression::BinaryOp(left, _op, right) => {
                self.analyze_expression(left);
                self.analyze_expression(right);
            }
            Expression::UnaryOp(_op, expr) => {
                self.analyze_expression(expr);
            }
            Expression::Assignment(left, _op, right) => {
                self.analyze_expression(left);
                self.analyze_expression(right);
            }
            Expression::Conditional(condition, true_expr, false_expr) => {
                self.analyze_expression(condition);
                self.analyze_expression(true_expr);
                self.analyze_expression(false_expr);
            }
            Expression::ArrayAccess(array, index) => {
                self.analyze_expression(array);
                self.analyze_expression(index);
            }
            Expression::MemberAccess(obj, _member) => {
                self.analyze_expression(obj);
            }
            Expression::PointerAccess(ptr, _member) => {
                self.analyze_expression(ptr);
            }
            Expression::PostfixOp(expr, _op) => {
                self.analyze_expression(expr);
            }
        
            Expression::Constant(_) | Expression::StringLiteral(_) => {
                // No scope analysis needed for literals
            }
        }
    }




}
