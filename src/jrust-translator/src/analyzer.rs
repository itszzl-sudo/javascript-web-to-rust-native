use crate::ast::{Expression, Program, SourceLocation, Statement};
use crate::error::Result;
use std::collections::{HashMap, HashSet};

pub struct Analyzer {
    scopes: Vec<Scope>,
    current_scope: usize,
    call_graph: CallGraph,
    functions: HashSet<String>,
    types: HashMap<String, JSType>,
    used_variables: HashSet<String>,
    defined_variables: HashSet<String>,
}

#[derive(Debug, Clone)]
pub struct Scope {
    variables: HashMap<String, VariableInfo>,
    parent: Option<usize>,
}

#[derive(Debug, Clone)]
pub struct VariableInfo {
    pub name: String,
    pub declared_at: SourceLocation,
    pub defined: bool,
    pub used: bool,
    pub types: Vec<JSType>,
}

#[derive(Debug, Clone, PartialEq)]
pub enum JSType {
    Undefined,
    Null,
    Boolean,
    Number,
    String,
    Object,
    Array,
    Function,
    Class,
    Unknown,
}

#[derive(Debug, Clone)]
pub struct CallGraph {
    nodes: HashMap<String, FunctionNode>,
    edges: HashMap<String, HashSet<String>>,
}

#[derive(Debug, Clone)]
pub struct FunctionNode {
    pub name: String,
    pub params: Vec<String>,
    pub calls: Vec<String>,
    pub can_inline: bool,
}

impl Analyzer {
    pub fn new() -> Self {
        Self {
            scopes: vec![Scope::new(None)],
            current_scope: 0,
            call_graph: CallGraph::new(),
            functions: HashSet::new(),
            types: HashMap::new(),
            used_variables: HashSet::new(),
            defined_variables: HashSet::new(),
        }
    }

    pub fn analyze(&mut self, program: &Program) -> Result<AnalysisResult> {
        self.collect_functions(program);
        self.analyze_program(program)?;
        self.analyze_call_graph();
        self.compute_inlining();

        Ok(AnalysisResult {
            scopes: self.scopes.clone(),
            call_graph: self.call_graph.clone(),
            types: self.types.clone(),
            functions: self.functions.clone(),
            used_variables: self.used_variables.clone(),
            defined_variables: self.defined_variables.clone(),
        })
    }

    fn collect_functions(&mut self, program: &Program) {
        for stmt in &program.body {
            self.collect_statement_functions(stmt);
        }
    }

    fn collect_statement_functions(&mut self, stmt: &Statement) {
        match stmt {
            Statement::FunctionDeclaration { id, params, body, .. } => {
                let name = id.name.clone();
                self.functions.insert(name.clone());
                let params: Vec<String> = params
                    .iter()
                    .map(|p| self.get_pattern_name(p))
                    .collect();
                self.call_graph.add_node(name.clone(), params);
                for s in &body.body {
                    self.collect_statement_functions(s);
                }
            }
            Statement::ClassDeclaration { body, .. } => {
                for elem in &body.body {
                    if let Some(crate::ast::MethodDefinition { value, .. }) = &elem.value {
                        if let crate::ast::FunctionExpression::FunctionExpression { id, params, body, .. } = value.as_ref() {
                            let params: Vec<String> = params
                                .iter()
                                .map(|p| self.get_pattern_name(p))
                                .collect();
                            if let Some(name) = id {
                                self.functions.insert(name.name.clone());
                                self.call_graph.add_node(name.name.clone(), params);
                            }
                            for s in &body.body {
                                self.collect_statement_functions(s);
                            }
                        }
                    }
                }
            }
            Statement::ForStatement { body, .. } => {
                self.collect_statement_functions(body);
            }
            Statement::WhileStatement { body, .. } => {
                self.collect_statement_functions(body);
            }
            Statement::DoWhileStatement { body, .. } => {
                self.collect_statement_functions(body);
            }
            Statement::IfStatement { consequent, alternate, .. } => {
                self.collect_statement_functions(consequent);
                if let Some(alt) = alternate {
                    self.collect_statement_functions(alt);
                }
            }
            Statement::BlockStatement { body, .. } => {
                for s in body {
                    self.collect_statement_functions(s);
                }
            }
            _ => {}
        }
    }

    fn analyze_program(&mut self, program: &Program) -> Result<()> {
        for stmt in &program.body {
            self.analyze_statement(stmt)?;
        }
        Ok(())
    }

    fn analyze_statement(&mut self, stmt: &Statement) -> Result<()> {
        match stmt {
            Statement::VariableDeclaration { declarations, kind, .. } => {
                for decl in declarations {
                    let name = self.get_pattern_name(&decl.id);
                    let var_info = VariableInfo {
                        name: name.clone(),
                        declared_at: SourceLocation::default(),
                        defined: decl.init.is_some(),
                        used: false,
                        types: Vec::new(),
                    };
                    self.declare_variable(name, var_info);
                    if let Some(init) = &decl.init {
                        self.infer_type(init);
                    }
                }
                Ok(())
            }
            Statement::FunctionDeclaration { id, params, body, .. } => {
                self.enter_scope();
                for param in params {
                    let name = self.get_pattern_name(param);
                    let var_info = VariableInfo {
                        name: name.clone(),
                        declared_at: SourceLocation::default(),
                        defined: true,
                        used: false,
                        types: vec![JSType::Unknown],
                    };
                    self.declare_variable(name, var_info);
                }
                for s in &body.body {
                    self.analyze_statement(s)?;
                }
                self.exit_scope();
                self.types.insert(id.name.clone(), JSType::Function);
                Ok(())
            }
            Statement::ClassDeclaration { id, super_class, body, .. } => {
                self.enter_scope();
                for elem in &body.body {
                    if let Some(crate::ast::MethodDefinition { value, .. }) = &elem.value {
                        if let crate::ast::FunctionExpression::FunctionExpression { params, body, .. } = value.as_ref() {
                            for param in params {
                                let name = self.get_pattern_name(param);
                                let var_info = VariableInfo {
                                    name: name.clone(),
                                    declared_at: SourceLocation::default(),
                                    defined: true,
                                    used: false,
                                    types: vec![JSType::Unknown],
                                };
                                self.declare_variable(name, var_info);
                            }
                            for s in &body.body {
                                self.analyze_statement(s)?;
                            }
                        }
                    }
                }
                self.exit_scope();
                let mut class_type = JSType::Class;
                if super_class.is_some() {
                    class_type = JSType::Object;
                }
                self.types.insert(id.name.clone(), class_type);
                Ok(())
            }
            Statement::IfStatement { test, consequent, alternate, .. } => {
                self.infer_type(test);
                self.analyze_statement(consequent)?;
                if let Some(alt) = alternate {
                    self.analyze_statement(alt)?;
                }
                Ok(())
            }
            Statement::ForStatement { init, test, update, body, .. } => {
                if let Some(init) = init {
                    match init {
                        crate::ast::ForStatementInit::Variable(stmt) => {
                            self.analyze_statement(stmt)?;
                        }
                        crate::ast::ForStatementInit::Expression(expr) => {
                            self.infer_type(expr);
                        }
                    }
                }
                if let Some(test) = test {
                    self.infer_type(test);
                }
                if let Some(update) = update {
                    self.infer_type(update);
                }
                self.analyze_statement(body)?;
                Ok(())
            }
            Statement::WhileStatement { test, body, .. } => {
                self.infer_type(test);
                self.analyze_statement(body)?;
                Ok(())
            }
            Statement::DoWhileStatement { body, test, .. } => {
                self.analyze_statement(body)?;
                self.infer_type(test);
                Ok(())
            }
            Statement::SwitchStatement { discriminant, cases, .. } => {
                self.infer_type(discriminant);
                for case in cases {
                    for stmt in &case.consequent {
                        self.analyze_statement(stmt)?;
                    }
                }
                Ok(())
            }
            Statement::BlockStatement { body, .. } => {
                self.enter_scope();
                for stmt in body {
                    self.analyze_statement(stmt)?;
                }
                self.exit_scope();
                Ok(())
            }
            Statement::ReturnStatement { argument, .. } => {
                if let Some(expr) = argument {
                    self.infer_type(expr);
                }
                Ok(())
            }
            Statement::TryStatement { block, handler, finalizer, .. } => {
                self.analyze_statement(block)?;
                if let Some(catch) = handler {
                    self.enter_scope();
                    let param_name = self.get_pattern_name(&catch.param);
                    let var_info = VariableInfo {
                        name: param_name.clone(),
                        declared_at: SourceLocation::default(),
                        defined: true,
                        used: false,
                        types: vec![JSType::Unknown],
                    };
                    self.declare_variable(param_name, var_info);
                    self.analyze_statement(catch.body.as_ref())?;
                    self.exit_scope();
                }
                if let Some(finally) = finalizer {
                    self.analyze_statement(finally)?;
                }
                Ok(())
            }
            Statement::ExpressionStatement { expression, .. } => {
                self.infer_type(expression);
                Ok(())
            }
            Statement::BreakStatement { .. } => Ok(()),
            Statement::ContinueStatement { .. } => Ok(()),
            Statement::ThrowStatement { argument, .. } => {
                self.infer_type(argument);
                Ok(())
            }
            Statement::ImportDeclaration { .. } => Ok(()),
            Statement::ExportNamedDeclaration { declaration, .. } => {
                if let Some(decl) = declaration {
                    self.analyze_statement(decl.as_ref())?;
                }
                Ok(())
            }
            Statement::ExportDefaultDeclaration { declaration, .. } => {
                self.analyze_statement(declaration.as_ref())?;
                Ok(())
            }
            Statement::ExportAllDeclaration { .. } => Ok(()),
            Statement::EmptyStatement { .. } => Ok(()),
            Statement::LabeledStatement { body, .. } => {
                self.analyze_statement(body)?;
                Ok(())
            }
            _ => Ok(()),
        }
    }

    fn infer_type(&mut self, expr: &Expression) -> JSType {
        match expr {
            Expression::Literal { value, .. } => match value {
                crate::ast::LiteralValue::Null => JSType::Null,
                crate::ast::LiteralValue::Undefined => JSType::Undefined,
                crate::ast::LiteralValue::Boolean(_) => JSType::Boolean,
                crate::ast::LiteralValue::Number(_) => JSType::Number,
                crate::ast::LiteralValue::String(_) => JSType::String,
                crate::ast::LiteralValue::RegExp { .. } => JSType::Object,
            },
            Expression::Identifier(id) => {
                self.mark_variable_used(&id.name);
                self.lookup_type(&id.name).unwrap_or(JSType::Unknown)
            }
            Expression::ArrayExpression { .. } => JSType::Array,
            Expression::ObjectExpression { .. } => JSType::Object,
            Expression::FunctionExpression { .. } => JSType::Function,
            Expression::ArrowFunctionExpression { .. } => JSType::Function,
            Expression::ClassExpression { .. } => JSType::Class,
            Expression::This { .. } => JSType::Object,
            Expression::BinaryExpression { left, right, .. } => {
                let left_type = self.infer_type(left);
                let right_type = self.infer_type(right);
                Self::widen_types(left_type, right_type)
            }
            Expression::UnaryExpression { argument, .. } => self.infer_type(argument),
            Expression::AssignmentExpression { right, .. } => self.infer_type(right),
            Expression::CallExpression { callee, .. } => {
                if let Expression::Identifier(id) = callee.as_ref() {
                    self.mark_variable_used(&id.name);
                    if self.functions.contains(&id.name) {
                        self.call_graph.add_call(id.name.clone());
                        return JSType::Unknown;
                    }
                }
                JSType::Unknown
            }
            Expression::MemberExpression { .. } => JSType::Unknown,
            Expression::NewExpression { .. } => JSType::Object,
            Expression::ConditionalExpression { consequent, alternate, .. } => {
                let cons_type = self.infer_type(consequent);
                let alt_type = self.infer_type(alternate);
                Self::widen_types(cons_type, alt_type)
            }
            Expression::TemplateLiteral { .. } => JSType::String,
            Expression::SpreadElement { argument, .. } => self.infer_type(argument),
            Expression::AwaitExpression { argument, .. } => self.infer_type(argument),
            Expression::YieldExpression { argument, .. } => {
                argument.as_ref().map_or(JSType::Unknown, |a| self.infer_type(a))
            }
            Expression::UpdateExpression { argument, .. } => self.infer_type(argument),
            Expression::Super { .. } => JSType::Object,
            Expression::Import { .. } => JSType::Unknown,
            _ => JSType::Unknown,
        }
    }

    fn widen_types(a: JSType, b: JSType) -> JSType {
        if a == b {
            a
        } else if a == JSType::Unknown {
            b
        } else if b == JSType::Unknown {
            a
        } else {
            JSType::Unknown
        }
    }

    fn get_pattern_name(&self, pattern: &crate::ast::Pattern) -> String {
        match pattern {
            crate::ast::Pattern::Identifier { id, .. } => id.name.clone(),
            _ => String::from("_"),
        }
    }

    fn enter_scope(&mut self) {
        let parent = Some(self.current_scope);
        self.scopes.push(Scope::new(parent));
        self.current_scope = self.scopes.len() - 1;
    }

    fn exit_scope(&mut self) {
        if let Some(parent) = self.scopes[self.current_scope].parent {
            self.current_scope = parent;
        }
    }

    fn declare_variable(&mut self, name: String, info: VariableInfo) {
        self.scopes[self.current_scope]
            .variables
            .insert(name.clone(), info);
        self.defined_variables.insert(name);
    }

    fn mark_variable_used(&mut self, name: &str) {
        self.used_variables.insert(name.to_string());
        for i in (0..=self.current_scope).rev() {
            if let Some(info) = self.scopes[i].variables.get_mut(name) {
                info.used = true;
                break;
            }
        }
    }

    fn lookup_type(&self, name: &str) -> Option<JSType> {
        for i in (0..=self.current_scope).rev() {
            if let Some(info) = self.scopes[i].variables.get(name) {
                return Some(info.types.first().cloned().unwrap_or(JSType::Unknown));
            }
        }
        None
    }

    fn analyze_call_graph(&mut self) {
        for (name, node) in &mut self.call_graph.nodes {
            if node.calls.len() <= 2 && node.params.len() <= 3 {
                node.can_inline = true;
            }
        }
    }

    fn compute_inlining(&mut self) {
        let names: Vec<String> = self.call_graph.nodes.keys().cloned().collect();
        for name in names {
            let can_inline = self.call_graph.nodes.get(&name).map(|n| n.can_inline).unwrap_or(false);
            if self.has_no_recursion(&name) && can_inline {
                if let Some(node) = self.call_graph.nodes.get_mut(&name) {
                    node.can_inline = true;
                }
            }
        }
    }

    fn has_recursion(&self, name: &str, visited: &mut HashSet<String>) -> bool {
        if visited.contains(name) {
            return true;
        }
        visited.insert(name.to_string());
        if let Some(calls) = self.call_graph.edges.get(name) {
            for callee in calls {
                if callee == name {
                    return true;
                }
                if self.has_recursion(callee, visited) {
                    return true;
                }
            }
        }
        false
    }

    fn has_no_recursion(&self, name: &str) -> bool {
        let mut visited = HashSet::new();
        !self.has_recursion(name, &mut visited)
    }
}

impl Scope {
    pub fn new(parent: Option<usize>) -> Self {
        Self {
            variables: HashMap::new(),
            parent,
        }
    }
}

impl CallGraph {
    pub fn new() -> Self {
        Self {
            nodes: HashMap::new(),
            edges: HashMap::new(),
        }
    }

    pub fn add_node(&mut self, name: String, params: Vec<String>) {
        let name_for_edges = name.clone();
        let name_for_node = name.clone();
        self.nodes.insert(
            name,
            FunctionNode {
                name: name_for_node,
                params,
                calls: Vec::new(),
                can_inline: false,
            },
        );
        self.edges.insert(name_for_edges, HashSet::new());
    }

    pub fn add_call(&mut self, caller: String) {
        if let Some(node) = self.nodes.get_mut(&caller) {
            if let Some(calls) = self.edges.get_mut(&caller) {
                calls.insert(caller.clone());
            }
        }
    }
}

impl Default for Analyzer {
    fn default() -> Self {
        Self::new()
    }
}

#[derive(Debug, Clone)]
pub struct AnalysisResult {
    pub scopes: Vec<Scope>,
    pub call_graph: CallGraph,
    pub types: HashMap<String, JSType>,
    pub functions: HashSet<String>,
    pub used_variables: HashSet<String>,
    pub defined_variables: HashSet<String>,
}
