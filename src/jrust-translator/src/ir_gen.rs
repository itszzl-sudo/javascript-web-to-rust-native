use crate::ast::{Expression, LiteralValue, Statement, Program, BinaryOperator, UnaryOperator};
use crate::error::Result;
use cranelift_compiler::ir::*;

pub struct IrGenerator {
    current_module: Module,
}

impl IrGenerator {
    pub fn new() -> Self {
        Self {
            current_module: Module::new("main".to_string()),
        }
    }
    
    pub fn generate(&mut self, program: &Program) -> Result<cranelift_compiler::Program> {
        for stmt in &program.body {
            self.translate_statement(stmt)?;
        }
        
        Ok(cranelift_compiler::Program {
            modules: vec![self.current_module.clone()],
            entry_point: Some("app_main".to_string()),
            bridge_api: BridgeApi::default(),
        })
    }
    
    fn translate_statement(&mut self, stmt: &Statement) -> Result<()> {
        match stmt {
            Statement::FunctionDeclaration { id, params, body, .. } => {
                let func = Function {
                    name: id.name.clone(),
                    params: params.iter().map(|p| self.translate_param(p)).collect(),
                    return_ty: IrType::Void,
                    body: body.body.iter().map(|s| self.translate_stmt(s)).collect::<Result<Vec<_>>>()?,
                    is_pub: false,
                    is_extern: false,
                };
                self.current_module.add_function(func);
            }
            
            Statement::VariableDeclaration { declarations, .. } => {
                for decl in declarations {
                    if let Some(init) = &decl.init {
                        let expr = self.translate_expr(init)?;
                        let var_name = self.get_pattern_name(&decl.id);
                        let var_decl = LocalVar {
                            name: var_name,
                            ty: IrType::I64,
                            init: Some(expr),
                        };
                        self.current_module.functions.entry("__top_level__".to_string())
                            .or_insert_with(|| Function {
                                name: "__top_level__".to_string(),
                                params: vec![],
                                return_ty: IrType::Void,
                                body: vec![],
                                is_pub: false,
                                is_extern: false,
                            })
                            .body.push(Stmt::VarDecl(var_decl));
                    }
                }
            }
            
            Statement::ExpressionStatement { expression, .. } => {
                let expr = self.translate_expr(expression)?;
                if let Some(func) = self.current_module.functions.get_mut("__top_level__") {
                    func.body.push(Stmt::ExprStmt(expr));
                }
            }
            
            _ => {}
        }
        
        Ok(())
    }
    
    fn translate_stmt(&self, stmt: &Statement) -> Result<Stmt> {
        match stmt {
            Statement::VariableDeclaration { declarations, .. } => {
                if let Some(decl) = declarations.first() {
                    let var_name = self.get_pattern_name(&decl.id);
                    let init_expr = if let Some(init) = &decl.init {
                        self.translate_expr(init)?
                    } else {
                        Expr::ConstI64(0)
                    };
                    
                    Ok(Stmt::VarDecl(LocalVar {
                        name: var_name,
                        ty: IrType::I64,
                        init: Some(init_expr),
                    }))
                } else {
                    Ok(Stmt::Block(vec![]))
                }
            }
            
            Statement::ExpressionStatement { expression, .. } => {
                Ok(Stmt::ExprStmt(self.translate_expr(expression)?))
            }
            
            Statement::ReturnStatement { argument, .. } => {
                let ret_val = argument.as_ref()
                    .map(|arg| self.translate_expr(arg))
                    .transpose()?;
                Ok(Stmt::Return(ret_val))
            }
            
            Statement::IfStatement { test, consequent, alternate, .. } => {
                let cond = self.translate_expr(test)?;
                let then_block = vec![self.translate_stmt(consequent)?];
                let else_block = alternate.as_ref()
                    .map(|alt| self.translate_stmt(alt))
                    .transpose()?
                    .map(|s| vec![s]);
                
                Ok(Stmt::If {
                    cond,
                    then_block,
                    else_block,
                })
            }
            
            Statement::WhileStatement { test, body, .. } => {
                Ok(Stmt::While {
                    cond: self.translate_expr(test)?,
                    body: vec![self.translate_stmt(body)?],
                })
            }
            
            Statement::BlockStatement { body, .. } => {
                let stmts: Vec<Stmt> = body.iter()
                    .map(|s| self.translate_stmt(s))
                    .collect::<Result<Vec<_>>>()?;
                Ok(Stmt::Block(stmts))
            }
            
            _ => Ok(Stmt::Block(vec![]))
        }
    }
    
    fn translate_expr(&self, expr: &Expression) -> Result<Expr> {
        match expr {
            Expression::Literal { value, .. } => {
                Ok(self.translate_literal(value))
            }
            
            Expression::Identifier(id) => {
                Ok(Expr::Var(id.name.clone()))
            }
            
            Expression::BinaryExpression { operator, left, right, .. } => {
                Ok(Expr::BinaryOp {
                    op: self.translate_binop(operator),
                    left: Box::new(self.translate_expr(left)?),
                    right: Box::new(self.translate_expr(right)?),
                })
            }
            
            Expression::UnaryExpression { operator, argument, .. } => {
                Ok(Expr::UnaryOp {
                    op: self.translate_unop(operator),
                    expr: Box::new(self.translate_expr(argument)?),
                })
            }
            
            Expression::CallExpression { callee, arguments, .. } => {
                let callee_name = match callee.as_ref() {
                    Expression::Identifier(id) => id.name.clone(),
                    _ => "unknown".to_string(),
                };
                
                Ok(Expr::Call {
                    func: callee_name,
                    args: arguments.iter()
                        .map(|arg| self.translate_expr(arg))
                        .collect::<Result<Vec<_>>>()?,
                })
            }
            
            Expression::MemberExpression { object, property, .. } => {
                Ok(Expr::FieldAccess {
                    base: Box::new(self.translate_expr(object)?),
                    field: match property.as_ref() {
                        Expression::Identifier(id) => id.name.clone(),
                        _ => "unknown".to_string(),
                    },
                })
            }
            
            Expression::AssignmentExpression { left, right: _, .. } => {
                let target = self.translate_expr(left)?;
                
                Ok(Expr::Cast {
                    expr: Box::new(Expr::Var(match target {
                        Expr::Var(ref name) => name.clone(),
                        _ => "tmp".to_string(),
                    })),
                    target_ty: IrType::I64,
                })
            }
            
            _ => Ok(Expr::ConstI64(0))
        }
    }
    
    fn translate_literal(&self, value: &LiteralValue) -> Expr {
        match value {
            LiteralValue::Number(n) => {
                if n.fract() == 0.0 {
                    Expr::ConstI64(*n as i64)
                } else {
                    Expr::ConstF64(*n)
                }
            }
            LiteralValue::Boolean(b) => Expr::ConstBool(*b),
            LiteralValue::String(s) => Expr::ConstString(s.clone()),
            LiteralValue::Null => Expr::ConstNull,
            LiteralValue::Undefined => Expr::ConstNull,
            LiteralValue::RegExp { pattern, .. } => Expr::ConstString(pattern.clone()),
        }
    }
    
    fn translate_binop(&self, op: &BinaryOperator) -> BinOp {
        match op {
            BinaryOperator::Add => BinOp::Add,
            BinaryOperator::Subtract => BinOp::Sub,
            BinaryOperator::Multiply => BinOp::Mul,
            BinaryOperator::Divide => BinOp::Div,
            BinaryOperator::Modulo => BinOp::Mod,
            BinaryOperator::Equal => BinOp::Eq,
            BinaryOperator::NotEqual => BinOp::Ne,
            BinaryOperator::LessThan => BinOp::Lt,
            BinaryOperator::LessThanEqual => BinOp::Le,
            BinaryOperator::GreaterThan => BinOp::Gt,
            BinaryOperator::GreaterThanEqual => BinOp::Ge,
            BinaryOperator::LogicalAnd => BinOp::And,
            BinaryOperator::LogicalOr => BinOp::Or,
            BinaryOperator::BitAnd => BinOp::BitAnd,
            BinaryOperator::BitOr => BinOp::BitOr,
            BinaryOperator::BitXor => BinOp::BitXor,
            BinaryOperator::ShiftLeft => BinOp::Shl,
            BinaryOperator::ShiftRight => BinOp::Shr,
            _ => BinOp::Add,
        }
    }
    
    fn translate_unop(&self, op: &UnaryOperator) -> UnaryOp {
        match op {
            UnaryOperator::Not => UnaryOp::Not,
            UnaryOperator::Minus => UnaryOp::Neg,
            _ => UnaryOp::Not,
        }
    }
    
    fn translate_param(&self, pattern: &crate::ast::Pattern) -> Param {
        Param {
            name: self.get_pattern_name(pattern),
            ty: IrType::I64,
        }
    }
    
    fn get_pattern_name(&self, pattern: &crate::ast::Pattern) -> String {
        match pattern {
            crate::ast::Pattern::Identifier { id, .. } => id.name.clone(),
            _ => "_".to_string(),
        }
    }
}

impl Default for IrGenerator {
    fn default() -> Self {
        Self::new()
    }
}
