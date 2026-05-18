use crate::ast::{Expression, Program, Statement, LiteralValue};
use std::collections::{HashMap, HashSet};

#[derive(Debug, Clone, PartialEq)]
pub enum CodeRole {
    Initializer,
    EventHandler,
    Mixed,
}

#[derive(Debug, Clone)]
pub struct SplitAnalysis {
    pub initializer_functions: HashSet<String>,
    pub event_handlers: HashSet<String>,
    pub dom_operations: Vec<DomOperation>,
    pub event_bindings: Vec<EventBinding>,
}

#[derive(Debug, Clone)]
pub struct DomOperation {
    pub operation: String,
    pub target: String,
    pub location: usize,
}

#[derive(Debug, Clone)]
pub struct EventBinding {
    pub event_type: String,
    pub target: String,
    pub handler: String,
    pub location: usize,
}

pub struct CodeSplitter {
    analysis: SplitAnalysis,
}

impl CodeSplitter {
    pub fn new() -> Self {
        Self {
            analysis: SplitAnalysis {
                initializer_functions: HashSet::new(),
                event_handlers: HashSet::new(),
                dom_operations: Vec::new(),
                event_bindings: Vec::new(),
            }
        }
    }
    
    pub fn analyze(&mut self, program: &Program) -> &SplitAnalysis {
        for (idx, stmt) in program.body.iter().enumerate() {
            self.analyze_statement(stmt, idx);
        }
        
        self.classify_functions(program);
        
        &self.analysis
    }
    
    fn analyze_statement(&mut self, stmt: &Statement, location: usize) {
        match stmt {
            Statement::ExpressionStatement { expression, .. } => {
                self.analyze_expression(expression, location);
            }
            
            Statement::VariableDeclaration { declarations, .. } => {
                for decl in declarations {
                    if let Some(init) = &decl.init {
                        self.analyze_expression(init, location);
                    }
                }
            }
            
            Statement::FunctionDeclaration { id, body, .. } => {
                for stmt in &body.body {
                    self.analyze_statement(stmt, location);
                }
            }
            
            _ => {}
        }
    }
    
    fn analyze_expression(&mut self, expr: &Expression, location: usize) {
        match expr {
            Expression::CallExpression { callee, arguments, .. } => {
                let callee_name = self.get_callee_name(callee);
                
                if callee_name.contains("addEventListener") {
                    self.extract_event_binding(callee, arguments, location);
                }
                
                if callee_name.contains("on") && callee_name.len() > 2 {
                    let event_type = callee_name[2..].to_lowercase();
                    if self.is_event_type(&event_type) {
                        self.extract_inline_event(callee, arguments, location);
                    }
                }
                
                for arg in arguments {
                    self.analyze_expression(arg, location);
                }
            }
            
            Expression::MemberExpression { object, property, .. } => {
                let prop_name = self.get_property_name(property);
                
                if self.is_dom_write_property(&prop_name) {
                    self.analysis.dom_operations.push(DomOperation {
                        operation: prop_name.clone(),
                        target: self.get_object_name(object),
                        location,
                    });
                }
                
                self.analyze_expression(object, location);
            }
            
            Expression::AssignmentExpression { left, right, .. } => {
                if let Expression::MemberExpression { property, .. } = left.as_ref() {
                    let prop_name = self.get_property_name(property);
                    if self.is_dom_write_property(&prop_name) {
                        self.analysis.dom_operations.push(DomOperation {
                            operation: prop_name,
                            target: "unknown".to_string(),
                            location,
                        });
                    }
                }
                self.analyze_expression(right, location);
            }
            
            _ => {}
        }
    }
    
    fn extract_event_binding(&mut self, callee: &Expression, args: &[Expression], location: usize) {
        let target = match callee {
            Expression::MemberExpression { object, .. } => self.get_object_name(object),
            _ => "document".to_string(),
        };
        
        if args.len() >= 2 {
            let event_type = match &args[0] {
                Expression::Literal { value: LiteralValue::String(s), .. } => s.clone(),
                _ => "unknown".to_string(),
            };
            
            let handler = match &args[1] {
                Expression::Identifier(id) => id.name.clone(),
                Expression::FunctionExpression { .. } => "anonymous".to_string(),
                _ => "unknown".to_string(),
            };
            
            self.analysis.event_bindings.push(EventBinding {
                event_type,
                target,
                handler,
                location,
            });
            
            if let Expression::Identifier(id) = &args[1] {
                self.analysis.event_handlers.insert(id.name.clone());
            }
        }
    }
    
    fn extract_inline_event(&mut self, callee: &Expression, args: &[Expression], location: usize) {
        let target = match callee {
            Expression::MemberExpression { object, .. } => self.get_object_name(object),
            _ => "element".to_string(),
        };
        
        let callee_name = self.get_callee_name(callee);
        let event_type = if callee_name.starts_with("on") {
            callee_name[2..].to_lowercase()
        } else {
            "click".to_string()
        };
        
        if !args.is_empty() {
            let handler = match &args[0] {
                Expression::Identifier(id) => id.name.clone(),
                Expression::FunctionExpression { .. } => "anonymous".to_string(),
                _ => "unknown".to_string(),
            };
            
            self.analysis.event_bindings.push(EventBinding {
                event_type,
                target,
                handler,
                location,
            });
        }
    }
    
    fn classify_functions(&mut self, program: &Program) {
        for stmt in &program.body {
            if let Statement::FunctionDeclaration { id, .. } = stmt {
                if !self.analysis.event_handlers.contains(&id.name) {
                    self.analysis.initializer_functions.insert(id.name.clone());
                }
            }
        }
    }
    
    fn get_callee_name(&self, expr: &Expression) -> String {
        match expr {
            Expression::Identifier(id) => id.name.clone(),
            Expression::MemberExpression { property, .. } => self.get_property_name(property),
            _ => "unknown".to_string(),
        }
    }
    
    fn get_property_name(&self, expr: &Expression) -> String {
        match expr {
            Expression::Identifier(id) => id.name.clone(),
            _ => "unknown".to_string(),
        }
    }
    
    fn get_object_name(&self, expr: &Expression) -> String {
        match expr {
            Expression::Identifier(id) => id.name.clone(),
            Expression::CallExpression { callee, .. } => self.get_callee_name(callee),
            _ => "unknown".to_string(),
        }
    }
    
    fn is_event_type(&self, name: &str) -> bool {
        matches!(name, "click" | "submit" | "change" | "input" | "keydown" | "keyup" | "load" | "mouse")
    }
    
    fn is_dom_write_property(&self, name: &str) -> bool {
        matches!(name, 
            "innerHTML" | "outerHTML" | "textContent" | 
            "value" | "src" | "href" | "className" | "style"
        )
    }
    
    pub fn split(&self, program: &Program) -> (Vec<Statement>, Vec<Statement>) {
        let mut initializer = Vec::new();
        let mut event_handler = Vec::new();
        
        for stmt in &program.body {
            let role = self.classify_statement(stmt);
            
            match role {
                CodeRole::Initializer => initializer.push(stmt.clone()),
                CodeRole::EventHandler => event_handler.push(stmt.clone()),
                CodeRole::Mixed => {
                    initializer.push(stmt.clone());
                }
            }
        }
        
        (initializer, event_handler)
    }
    
    fn classify_statement(&self, stmt: &Statement) -> CodeRole {
        match stmt {
            Statement::FunctionDeclaration { id, body, .. } => {
                if self.analysis.event_handlers.contains(&id.name) {
                    return CodeRole::EventHandler;
                }
                
                let has_event = body.body.iter().any(|s| {
                    self.statement_location(s).map_or(false, |loc| {
                        self.analysis.event_bindings.iter().any(|b| b.location == loc)
                    })
                });
                
                if has_event {
                    return CodeRole::EventHandler;
                }
                
                CodeRole::Initializer
            }
            
            Statement::ExpressionStatement { expression, .. } => {
                if self.contains_event_binding(expression) {
                    return CodeRole::EventHandler;
                }
                if self.contains_dom_write(expression) {
                    return CodeRole::Mixed;
                }
                CodeRole::Initializer
            }
            
            Statement::VariableDeclaration { declarations, .. } => {
                for decl in declarations {
                    if let Some(init) = &decl.init {
                        if self.contains_event_binding(init) {
                            return CodeRole::EventHandler;
                        }
                    }
                }
                CodeRole::Initializer
            }
            
            _ => CodeRole::Initializer,
        }
    }
    
    fn contains_event_binding(&self, expr: &Expression) -> bool {
        match expr {
            Expression::CallExpression { callee, arguments, .. } => {
                let name = self.get_callee_name(callee);
                if name.contains("addEventListener") || 
                   (name.starts_with("on") && self.is_event_type(&name[2..].to_lowercase())) {
                    return true;
                }
                arguments.iter().any(|arg| self.contains_event_binding(arg))
            }
            Expression::MemberExpression { object, property, .. } => {
                let prop = self.get_property_name(property);
                if prop.starts_with("on") {
                    return true;
                }
                self.contains_event_binding(object)
            }
            _ => false,
        }
    }
    
    fn contains_dom_write(&self, expr: &Expression) -> bool {
        match expr {
            Expression::AssignmentExpression { left, .. } => {
                if let Expression::MemberExpression { property, .. } = left.as_ref() {
                    let prop = self.get_property_name(property);
                    if self.is_dom_write_property(&prop) {
                        return true;
                    }
                }
                false
            }
            Expression::MemberExpression { property, .. } => {
                let prop = self.get_property_name(property);
                self.is_dom_write_property(&prop)
            }
            _ => false,
        }
    }
    
    fn statement_location(&self, stmt: &Statement) -> Option<usize> {
        match stmt {
            Statement::ExpressionStatement { expression, .. } => {
                self.find_expression_location(expression)
            }
            _ => None,
        }
    }
    
    fn find_expression_location(&self, expr: &Expression) -> Option<usize> {
        match expr {
            Expression::CallExpression { .. } => {
                self.analysis.event_bindings.iter()
                    .map(|b| b.location)
                    .next()
            }
            _ => None,
        }
    }
}

impl Default for CodeSplitter {
    fn default() -> Self {
        Self::new()
    }
}
