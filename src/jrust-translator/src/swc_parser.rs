use swc_core::common::sync::Lrc;
use swc_core::common::SourceMap;
use swc_core::common::FileName;
use swc_core::ecma::ast as swc_ast;
use swc_core::ecma::parser::{lexer::Lexer, Parser, StringInput, Syntax};
use thiserror::Error;

use crate::ast::*;

#[derive(Error, Debug)]
pub enum SwcParseError {
    #[error("SWC parse error: {0}")]
    SwcError(String),
}

pub struct SwcParser;

impl SwcParser {
    pub fn new() -> Self {
        SwcParser
    }

    pub fn parse(&self, source: &str) -> Result<swc_ast::Module, SwcParseError> {
        let cm: Lrc<SourceMap> = Default::default();
        let fm = cm.new_source_file(
            FileName::Custom("input.js".into()).into(),
            source.to_string(),
        );

        let syntax = Syntax::Es(Default::default());

        let lexer = Lexer::new(
            syntax,
            Default::default(),
            StringInput::from(&*fm),
            None,
        );

        let mut parser = Parser::new_from(lexer);

        let module = parser
            .parse_module()
            .map_err(|e| SwcParseError::SwcError(format!("Parse error: {:?}", e)))?;

        Ok(module)
    }

    pub fn parse_expression(&self, source: &str) -> Result<swc_ast::Expr, SwcParseError> {
        let cm: Lrc<SourceMap> = Default::default();
        let fm = cm.new_source_file(
            FileName::Custom("expr.js".into()).into(),
            source.to_string(),
        );

        let syntax = Syntax::Es(Default::default());

        let lexer = Lexer::new(
            syntax,
            Default::default(),
            StringInput::from(&*fm),
            None,
        );

        let mut parser = Parser::new_from(lexer);

        let expr = parser
            .parse_expr()
            .map_err(|e| SwcParseError::SwcError(format!("Parse error: {:?}", e)))?;

        Ok(*expr)
    }
}

impl Default for SwcParser {
    fn default() -> Self {
        Self::new()
    }
}

pub fn swc_module_to_ast(module: swc_ast::Module) -> Vec<Statement> {
    let mut statements = Vec::new();

    for item in module.body {
        match item {
            swc_ast::ModuleItem::Stmt(stmt) => {
                if let Some(s) = swc_stmt_to_stmt(stmt) {
                    statements.push(s);
                }
            }
            swc_ast::ModuleItem::ModuleDecl(decl) => {
                if let Some(stmt) = swc_module_decl_to_stmt(decl) {
                    statements.push(stmt);
                }
            }
        }
    }

    statements
}

fn swc_stmt_to_stmt(stmt: swc_ast::Stmt) -> Option<Statement> {
    match stmt {
        swc_ast::Stmt::Expr(expr) => Some(Statement::ExpressionStatement {
            expression: swc_expr_to_ast(*expr.expr),
            loc: SourceLocation::default(),
        }),
        swc_ast::Stmt::Block(block) => Some(swc_block_stmt_to_ast(block)),
        swc_ast::Stmt::If(if_stmt) => Some(Statement::IfStatement {
            test: swc_expr_to_ast(*if_stmt.test),
            consequent: Box::new(swc_stmt_to_stmt(*if_stmt.cons).unwrap_or(Statement::EmptyStatement { loc: SourceLocation::default() })),
            alternate: if_stmt.alt.map(|a| Box::new(swc_stmt_to_stmt(*a).unwrap_or(Statement::EmptyStatement { loc: SourceLocation::default() }))),
            loc: SourceLocation::default(),
        }),
        swc_ast::Stmt::For(for_stmt) => Some(swc_for_stmt_to_stmt(for_stmt)),
        swc_ast::Stmt::ForIn(for_in) => Some(swc_for_in_stmt_to_stmt(for_in)),
        swc_ast::Stmt::ForOf(for_of) => Some(swc_for_of_stmt_to_stmt(for_of)),
        swc_ast::Stmt::While(while_) => Some(Statement::WhileStatement {
            test: swc_expr_to_ast(*while_.test),
            body: Box::new(swc_stmt_to_stmt(*while_.body).unwrap_or(Statement::EmptyStatement { loc: SourceLocation::default() })),
            loc: SourceLocation::default(),
        }),
        swc_ast::Stmt::DoWhile(do_while) => Some(Statement::DoWhileStatement {
            body: Box::new(swc_stmt_to_stmt(*do_while.body).unwrap_or(Statement::EmptyStatement { loc: SourceLocation::default() })),
            test: swc_expr_to_ast(*do_while.test),
            loc: SourceLocation::default(),
        }),
        swc_ast::Stmt::Return(ret) => Some(Statement::ReturnStatement {
            argument: ret.arg.map(|e| swc_expr_to_ast(*e)),
            loc: SourceLocation::default(),
        }),
        swc_ast::Stmt::Break(_) => Some(Statement::BreakStatement {
            label: None,
            loc: SourceLocation::default(),
        }),
        swc_ast::Stmt::Continue(_) => Some(Statement::ContinueStatement {
            label: None,
            loc: SourceLocation::default(),
        }),
        swc_ast::Stmt::Throw(throw) => Some(Statement::ThrowStatement {
            argument: swc_expr_to_ast(*throw.arg),
            loc: SourceLocation::default(),
        }),
        swc_ast::Stmt::Try(try_) => Some(Statement::TryStatement {
            block: Box::new(swc_block_stmt_to_ast(try_.block)),
            handler: try_.handler.map(|h| Box::new(CatchClause {
                param: h.param.map(swc_pat_to_pattern).unwrap_or(Pattern::Identifier {
                    id: Identifier {
                        name: "_".to_string(),
                        loc: SourceLocation::default(),
                    },
                    init: None,
                }),
                body: Box::new(swc_block_stmt_to_ast(h.body)),
                loc: SourceLocation::default(),
            })),
            finalizer: try_.finalizer.map(|f| Box::new(swc_block_stmt_to_ast(f))),
            loc: SourceLocation::default(),
        }),
        swc_ast::Stmt::Switch(switch_) => Some(Statement::SwitchStatement {
            discriminant: swc_expr_to_ast(*switch_.discriminant),
            cases: switch_.cases.into_iter().map(swc_switch_case_to_case).collect(),
            loc: SourceLocation::default(),
        }),
        swc_ast::Stmt::Labeled(label) => Some(Statement::LabeledStatement {
            label: Identifier {
                name: String::from(&*label.label.sym),
                loc: SourceLocation::default(),
            },
            body: Box::new(swc_stmt_to_stmt(*label.body).unwrap_or(Statement::EmptyStatement { loc: SourceLocation::default() })),
            loc: SourceLocation::default(),
        }),
        swc_ast::Stmt::With(with_) => Some(Statement::WithStatement {
            object: swc_expr_to_ast(*with_.obj),
            body: Box::new(swc_stmt_to_stmt(*with_.body).unwrap_or(Statement::EmptyStatement { loc: SourceLocation::default() })),
            loc: SourceLocation::default(),
        }),
        swc_ast::Stmt::Debugger(_) => Some(Statement::DebuggerStatement {
            loc: SourceLocation::default(),
        }),
        swc_ast::Stmt::Empty(_) => Some(Statement::EmptyStatement {
            loc: SourceLocation::default(),
        }),
        swc_ast::Stmt::Decl(decl) => swc_decl_to_stmt(decl),
        _ => None,
    }
}

fn swc_decl_to_stmt(decl: swc_ast::Decl) -> Option<Statement> {
    match decl {
        swc_ast::Decl::Var(var_decl) => Some(Statement::VariableDeclaration {
            kind: match var_decl.kind {
                swc_ast::VarDeclKind::Var => VariableDeclarationKind::Var,
                swc_ast::VarDeclKind::Let => VariableDeclarationKind::Let,
                swc_ast::VarDeclKind::Const => VariableDeclarationKind::Const,
            },
            declarations: var_decl.decls.into_iter().map(swc_var_declarator_to_pattern).collect(),
            loc: SourceLocation::default(),
        }),
        swc_ast::Decl::Fn(fn_decl) => Some(Statement::FunctionDeclaration {
            id: Identifier {
                name: String::from(&*fn_decl.ident.sym),
                loc: SourceLocation::default(),
            },
            params: fn_decl.function.params.into_iter().map(|p| swc_pat_to_pattern(p.pat)).collect(),
            body: Box::new(swc_block_to_function_body(fn_decl.function.body)),
            generator: fn_decl.function.is_generator,
            async_: fn_decl.function.is_async,
            loc: SourceLocation::default(),
        }),
        swc_ast::Decl::Class(class) => Some(Statement::ClassDeclaration {
            id: Identifier {
                name: String::from(&*class.ident.sym),
                loc: SourceLocation::default(),
            },
            super_class: class.class.super_class.map(|e| Box::new(swc_expr_to_ast(*e))),
            body: Box::new(ClassBody {
                body: class.class.body.iter().cloned().map(swc_class_member_to_element).collect(),
                loc: SourceLocation::default(),
            }),
            loc: SourceLocation::default(),
        }),
        _ => None,
    }
}

fn swc_block_to_function_body(block: Option<swc_ast::BlockStmt>) -> FunctionBody {
    match block {
        Some(b) => FunctionBody {
            body: b.stmts.into_iter().filter_map(swc_stmt_to_stmt).collect(),
            loc: SourceLocation::default(),
        },
        None => FunctionBody {
            body: Vec::new(),
            loc: SourceLocation::default(),
        },
    }
}

fn swc_module_decl_to_stmt(decl: swc_ast::ModuleDecl) -> Option<Statement> {
    match decl {
        swc_ast::ModuleDecl::ExportNamed(named) => {
            Some(Statement::ExportNamedDeclaration {
                declaration: None,
                specifiers: Vec::new(),
                source: named.src.map(|s| s.value.to_string_lossy().to_string()),
                loc: SourceLocation::default(),
            })
        }
        swc_ast::ModuleDecl::Import(import) => Some(Statement::ImportDeclaration {
            specifiers: import.specifiers.into_iter().map(swc_import_spec_to_spec).collect(),
            source: import.src.value.to_string_lossy().to_string(),
            loc: SourceLocation::default(),
        }),
        _ => None,
    }
}

fn swc_import_spec_to_spec(spec: swc_ast::ImportSpecifier) -> crate::ast::ImportSpecifier {
    match spec {
        swc_ast::ImportSpecifier::Default(default) => crate::ast::ImportSpecifier {
            imported: Identifier {
                name: "_".to_string(),
                loc: SourceLocation::default(),
            },
            local: Identifier {
                name: String::from(&*default.local.sym),
                loc: SourceLocation::default(),
            },
            loc: SourceLocation::default(),
        },
        swc_ast::ImportSpecifier::Named(named) => crate::ast::ImportSpecifier {
            imported: named.imported.map(|i| match i {
                swc_ast::ModuleExportName::Ident(ident) => Identifier {
                    name: String::from(&*ident.sym),
                    loc: SourceLocation::default(),
                },
                swc_ast::ModuleExportName::Str(s) => Identifier {
                    name: s.value.to_string_lossy().to_string(),
                    loc: SourceLocation::default(),
                },
            }).unwrap_or(Identifier {
                name: String::from(&*named.local.sym),
                loc: SourceLocation::default(),
            }),
            local: Identifier {
                name: String::from(&*named.local.sym),
                loc: SourceLocation::default(),
            },
            loc: SourceLocation::default(),
        },
        swc_ast::ImportSpecifier::Namespace(ns) => crate::ast::ImportSpecifier {
            imported: Identifier {
                name: "_".to_string(),
                loc: SourceLocation::default(),
            },
            local: Identifier {
                name: String::from(&*ns.local.sym),
                loc: SourceLocation::default(),
            },
            loc: SourceLocation::default(),
        },
    }
}

fn swc_class_member_to_element(member: swc_ast::ClassMember) -> ClassElement {
    match member {
        swc_ast::ClassMember::Constructor(constructor) => ClassElement {
            key: Some(swc_prop_name_to_expr(constructor.key)),
            value: None,
            kind: ClassElementKind::Constructor,
            computed: false,
            static_: false,
            loc: SourceLocation::default(),
        },
        swc_ast::ClassMember::Method(method) => {
            let kind = match method.kind {
                swc_ast::MethodKind::Method => ClassElementKind::Method,
                swc_ast::MethodKind::Getter => ClassElementKind::Get,
                swc_ast::MethodKind::Setter => ClassElementKind::Set,
            };
            let key = method.key;
            let key_expr = swc_prop_name_to_expr(key.clone());
            ClassElement {
                key: Some(key_expr.clone()),
                value: Some(MethodDefinition {
                    key: key_expr,
                    value: Box::new(crate::ast::FunctionExpression::FunctionExpression {
                        id: None,
                        params: method.function.params.iter().map(|p| swc_pat_to_pattern(p.pat.clone())).collect(),
                        body: Box::new(swc_block_to_function_body(method.function.body)),
                        generator: method.function.is_generator,
                        async_: method.function.is_async,
                        loc: SourceLocation::default(),
                    }),
                    kind: kind.clone(),
                    computed: false,
                    static_: method.is_static,
                    loc: SourceLocation::default(),
                }),
                kind,
                computed: false,
                static_: method.is_static,
                loc: SourceLocation::default(),
            }
        }
        swc_ast::ClassMember::ClassProp(prop) => ClassElement {
            key: Some(swc_prop_name_to_expr(prop.key)),
            value: None,
            kind: ClassElementKind::Field,
            computed: false,
            static_: prop.is_static,
            loc: SourceLocation::default(),
        },
        swc_ast::ClassMember::Empty(_) => ClassElement {
            key: Some(Expression::Identifier(Identifier {
                name: "empty".to_string(),
                loc: SourceLocation::default(),
            })),
            value: None,
            kind: ClassElementKind::Method,
            computed: false,
            static_: false,
            loc: SourceLocation::default(),
        },
        swc_ast::ClassMember::AutoAccessor(accessor) => {
            let expr = match accessor.key {
                swc_ast::Key::Public(p) => swc_prop_name_to_expr(p),
                swc_ast::Key::Private(p) => Expression::Identifier(Identifier {
                    name: String::from(&*p.name),
                    loc: SourceLocation::default(),
                }),
            };
            ClassElement {
                key: Some(expr),
                value: None,
                kind: ClassElementKind::Field,
                computed: false,
                static_: accessor.is_static,
                loc: SourceLocation::default(),
            }
        },
        _ => ClassElement {
            key: Some(Expression::Identifier(Identifier {
                name: "unknown".to_string(),
                loc: SourceLocation::default(),
            })),
            value: None,
            kind: ClassElementKind::Field,
            computed: false,
            static_: false,
            loc: SourceLocation::default(),
        },
    }
}

fn swc_var_declarator_to_pattern(declarator: swc_ast::VarDeclarator) -> VariableDeclarator {
    VariableDeclarator {
        id: swc_pat_to_pattern(declarator.name),
        init: declarator.init.map(|e| swc_expr_to_ast(*e)),
        loc: SourceLocation::default(),
    }
}

fn swc_pat_to_pattern(pat: swc_ast::Pat) -> Pattern {
    match pat {
        swc_ast::Pat::Ident(ident) => Pattern::Identifier {
            id: Identifier {
                name: String::from(&*ident.id.sym),
                loc: SourceLocation::default(),
            },
            init: None,
        },
        swc_ast::Pat::Array(arr) => Pattern::ArrayPattern {
            elements: arr.elems.into_iter().map(|e| e.map(swc_pat_to_pattern)).collect::<Vec<_>>(),
            loc: SourceLocation::default(),
        },
        swc_ast::Pat::Object(obj) => Pattern::ObjectPattern {
            properties: obj.props.into_iter().map(swc_pat_prop_to_prop).collect(),
            loc: SourceLocation::default(),
        },
        swc_ast::Pat::Rest(rest) => Pattern::RestElement {
            argument: Box::new(swc_pat_to_pattern(*rest.arg)),
            loc: SourceLocation::default(),
        },
        swc_ast::Pat::Assign(assign) => Pattern::AssignmentPattern {
            left: Box::new(swc_pat_to_pattern(*assign.left)),
            right: Box::new(swc_expr_to_ast(*assign.right)),
            loc: SourceLocation::default(),
        },
        _ => Pattern::Identifier {
            id: Identifier {
                name: "_".to_string(),
                loc: SourceLocation::default(),
            },
            init: None,
        },
    }
}

fn swc_pat_prop_to_prop(prop: swc_ast::ObjectPatProp) -> ObjectPatternProperty {
    match prop {
        swc_ast::ObjectPatProp::KeyValue(kv) => ObjectPatternProperty {
            key: swc_prop_name_to_expr(kv.key),
            value: Some(swc_pat_to_pattern(*kv.value)),
            computed: false,
            loc: SourceLocation::default(),
        },
        swc_ast::ObjectPatProp::Assign(assign) => ObjectPatternProperty {
            key: Expression::Identifier(Identifier {
                name: String::from(&*assign.key.sym),
                loc: SourceLocation::default(),
            }),
            value: Some(Pattern::Identifier {
                id: Identifier {
                    name: String::from(&*assign.key.sym),
                    loc: SourceLocation::default(),
                },
                init: assign.value.map(|e| Box::new(swc_expr_to_ast(*e))),
            }),
            computed: false,
            loc: SourceLocation::default(),
        },
        swc_ast::ObjectPatProp::Rest(rest) => ObjectPatternProperty {
            key: Expression::Identifier(Identifier {
                name: "...".to_string(),
                loc: SourceLocation::default(),
            }),
            value: Some(swc_pat_to_pattern(*rest.arg)),
            computed: false,
            loc: SourceLocation::default(),
        },
    }
}

fn swc_prop_name_to_expr(prop_name: swc_ast::PropName) -> Expression {
    match prop_name {
        swc_ast::PropName::Ident(ident) => Expression::Identifier(Identifier {
            name: String::from(&*ident.sym),
            loc: SourceLocation::default(),
        }),
        swc_ast::PropName::Str(s) => Expression::Literal {
            value: LiteralValue::String(s.value.to_string_lossy().to_string()),
            raw: s.raw.map(|r| String::from(&*r)).unwrap_or_default(),
            loc: SourceLocation::default(),
        },
        swc_ast::PropName::Num(n) => Expression::Literal {
            value: LiteralValue::Number(n.value),
            raw: n.raw.map(|r| r.to_string()).unwrap_or_default(),
            loc: SourceLocation::default(),
        },
        swc_ast::PropName::BigInt(b) => Expression::Literal {
            value: LiteralValue::Number(b.value.to_string().parse().unwrap_or(0.0)),
            raw: b.raw.map(|r| r.to_string()).unwrap_or_default(),
            loc: SourceLocation::default(),
        },
        swc_ast::PropName::Computed(c) => swc_expr_to_ast(*c.expr),
    }
}

fn swc_member_prop_to_expr(prop: swc_ast::MemberProp) -> Expression {
    match prop {
        swc_ast::MemberProp::Ident(ident) => Expression::Identifier(Identifier {
            name: String::from(&*ident.sym),
            loc: SourceLocation::default(),
        }),
        swc_ast::MemberProp::PrivateName(_) => Expression::Identifier(Identifier {
            name: "#private".to_string(),
            loc: SourceLocation::default(),
        }),
        swc_ast::MemberProp::Computed(c) => swc_expr_to_ast(*c.expr),
    }
}

fn swc_expr_to_ast(expr: swc_ast::Expr) -> Expression {
    match expr {
        swc_ast::Expr::Ident(ident) => Expression::Identifier(Identifier {
            name: String::from(&*ident.sym),
            loc: SourceLocation::default(),
        }),
        swc_ast::Expr::Lit(lit) => swc_lit_to_expr(lit),
        swc_ast::Expr::Bin(bin) => Expression::BinaryExpression {
            operator: swc_bin_op_to_op(bin.op),
            left: Box::new(swc_expr_to_ast(*bin.left)),
            right: Box::new(swc_expr_to_ast(*bin.right)),
            loc: SourceLocation::default(),
        },
        swc_ast::Expr::Unary(unary) => Expression::UnaryExpression {
            operator: swc_unary_op_to_op(unary.op),
            argument: Box::new(swc_expr_to_ast(*unary.arg)),
            prefix: true,
            loc: SourceLocation::default(),
        },
        swc_ast::Expr::Assign(assign) => {
            let left = match assign.left {
                swc_ast::AssignTarget::Simple(s) => {
                    let e: Box<swc_ast::Expr> = s.into();
                    swc_expr_to_ast(*e)
                },
                swc_ast::AssignTarget::Pat(p) => Expression::Identifier(Identifier {
                    name: String::new(),
                    loc: SourceLocation::default(),
                }),
            };
            Expression::AssignmentExpression {
                operator: swc_assign_op_to_op(assign.op),
                left: Box::new(left),
                right: Box::new(swc_expr_to_ast(*assign.right)),
                loc: SourceLocation::default(),
            }
        },
        swc_ast::Expr::Member(member) => swc_member_to_expr(member),
        swc_ast::Expr::Call(call) => {
            let callee_expr = match call.callee {
                swc_ast::Callee::Expr(e) => swc_expr_to_ast(*e),
                swc_ast::Callee::Super(_) => Expression::Super {
                    loc: SourceLocation::default(),
                },
                swc_ast::Callee::Import(_) => Expression::Identifier(Identifier {
                    name: "import".to_string(),
                    loc: SourceLocation::default(),
                }),
            };
            Expression::CallExpression {
                callee: Box::new(callee_expr),
                arguments: call.args.into_iter().map(|a| swc_expr_to_ast(*a.expr)).collect(),
                loc: SourceLocation::default(),
            }
        },
        swc_ast::Expr::New(new) => Expression::NewExpression {
            callee: Box::new(swc_expr_to_ast(*new.callee)),
            arguments: new.args.map(|args| args.into_iter().map(|a| swc_expr_to_ast(*a.expr)).collect::<Vec<_>>()).unwrap_or_default(),
            loc: SourceLocation::default(),
        },
        swc_ast::Expr::Arrow(arrow) => Expression::ArrowFunctionExpression {
            params: arrow.params.into_iter().map(swc_pat_to_pattern).collect(),
            body: Box::new(swc_arrow_body_to_body(*arrow.body)),
            loc: SourceLocation::default(),
        },
        swc_ast::Expr::Fn(func) => Expression::FunctionExpression {
            id: func.ident.map(|i| Identifier {
                name: String::from(&*i.sym),
                loc: SourceLocation::default(),
            }),
            params: func.function.params.into_iter().map(|p| swc_pat_to_pattern(p.pat)).collect(),
            body: Box::new(swc_block_to_function_body(func.function.body)),
            loc: SourceLocation::default(),
        },
        swc_ast::Expr::Cond(cond) => Expression::ConditionalExpression {
            test: Box::new(swc_expr_to_ast(*cond.test)),
            consequent: Box::new(swc_expr_to_ast(*cond.cons)),
            alternate: Box::new(swc_expr_to_ast(*cond.alt)),
            loc: SourceLocation::default(),
        },
        swc_ast::Expr::Array(arr) => Expression::ArrayExpression {
            elements: arr.elems.into_iter().map(|e| e.map(|expr_or_spread| {
                if expr_or_spread.spread.is_some() {
                    Expression::SpreadElement {
                        argument: Box::new(swc_expr_to_ast(*expr_or_spread.expr)),
                        loc: SourceLocation::default(),
                    }
                } else {
                    swc_expr_to_ast(*expr_or_spread.expr)
                }
            })).collect::<Vec<_>>(),
            loc: SourceLocation::default(),
        },
        swc_ast::Expr::Object(obj) => Expression::ObjectExpression {
            properties: obj.props.into_iter().filter_map(|prop_or_spread| match prop_or_spread {
                swc_ast::PropOrSpread::Prop(prop) => swc_prop_to_prop(*prop),
                swc_ast::PropOrSpread::Spread(_) => None,
            }).collect(),
            loc: SourceLocation::default(),
        },
        swc_ast::Expr::Tpl(tpl) => {
            Expression::TemplateLiteral {
                quasis: tpl.quasis.into_iter().map(|quasi| TemplateElement {
                    value: TemplateElementValue {
                        raw: String::from(&*quasi.raw),
                        cooked: String::from(&*quasi.raw),
                    },
                    tail: quasi.tail,
                    loc: SourceLocation::default(),
                }).collect(),
                expressions: tpl.exprs.into_iter().map(|expr| swc_expr_to_ast(*expr)).collect(),
                loc: SourceLocation::default(),
            }
        }
        swc_ast::Expr::Await(await_) => Expression::AwaitExpression {
            argument: Box::new(swc_expr_to_ast(*await_.arg)),
            loc: SourceLocation::default(),
        },
        swc_ast::Expr::Yield(yield_) => Expression::YieldExpression {
            argument: yield_.arg.map(|e| Box::new(swc_expr_to_ast(*e))),
            delegate: yield_.delegate,
            loc: SourceLocation::default(),
        },
        swc_ast::Expr::This(_) => Expression::This {
            loc: SourceLocation::default(),
        },
        swc_ast::Expr::Seq(seq) => Expression::SequenceExpression {
            expressions: seq.exprs.into_iter().map(|e| swc_expr_to_ast(*e)).collect::<Vec<_>>(),
            loc: SourceLocation::default(),
        },
        swc_ast::Expr::Update(update) => Expression::UpdateExpression {
            operator: if update.op == swc_ast::UpdateOp::PlusPlus {
                UpdateOperator::Increment
            } else {
                UpdateOperator::Decrement
            },
            argument: Box::new(swc_expr_to_ast(*update.arg)),
            prefix: update.prefix,
            loc: SourceLocation::default(),
        },
        swc_ast::Expr::OptChain(opt) => {
            let (object, property, computed) = match *opt.base {
                swc_ast::OptChainBase::Member(member) => {
                    let is_computed = matches!(member.prop, swc_ast::MemberProp::Computed(_));
                    (
                        Box::new(swc_expr_to_ast(*member.obj)),
                        Box::new(swc_member_prop_to_expr(member.prop)),
                        is_computed,
                    )
                },
                swc_ast::OptChainBase::Call(_) => (
                    Box::new(Expression::Identifier(Identifier {
                        name: "unknown_call".to_string(),
                        loc: SourceLocation::default(),
                    })),
                    Box::new(Expression::Identifier(Identifier {
                        name: "unknown_call".to_string(),
                        loc: SourceLocation::default(),
                    })),
                    false,
                ),
            };
            Expression::OptionalMemberExpression {
                object,
                property,
                computed,
                loc: SourceLocation::default(),
            }
        },
        swc_ast::Expr::Paren(paren) => swc_expr_to_ast(*paren.expr),
        swc_ast::Expr::Class(class) => Expression::ClassExpression {
            id: class.ident.map(|i| Identifier {
                name: i.sym.to_string(),
                loc: SourceLocation::default(),
            }),
            super_class: class.class.super_class.map(|e| Box::new(swc_expr_to_ast(*e))),
            body: Box::new(ClassBody {
                body: class.class.body.iter().cloned().map(swc_class_member_to_element).collect(),
                loc: SourceLocation::default(),
            }),
            loc: SourceLocation::default(),
        },
        swc_ast::Expr::MetaProp(meta) => Expression::Identifier(Identifier {
            name: "new.target".to_string(),
            loc: SourceLocation::default(),
        }),
        _ => Expression::Identifier(Identifier {
            name: "unknown".to_string(),
            loc: SourceLocation::default(),
        }),
    }
}

fn swc_arrow_body_to_body(body: swc_ast::BlockStmtOrExpr) -> ArrowFunctionBody {
    match body {
        swc_ast::BlockStmtOrExpr::Expr(expr) => ArrowFunctionBody::Expression(swc_expr_to_ast(*expr)),
        swc_ast::BlockStmtOrExpr::BlockStmt(block) => ArrowFunctionBody::BlockFunctionBody(swc_block_to_function_body(Some(block))),
    }
}

fn swc_block_stmt_to_ast(block: swc_ast::BlockStmt) -> Statement {
    Statement::BlockStatement {
        body: block.stmts.into_iter().filter_map(swc_stmt_to_stmt).collect(),
        loc: SourceLocation::default(),
    }
}

fn swc_member_to_expr(member: swc_ast::MemberExpr) -> Expression {
    let is_computed = matches!(member.prop, swc_ast::MemberProp::Computed(_));
    let property = match member.prop {
        swc_ast::MemberProp::Ident(ident) => Expression::Identifier(Identifier {
            name: String::from(&*ident.sym),
            loc: SourceLocation::default(),
        }),
        swc_ast::MemberProp::Computed(c) => swc_expr_to_ast(*c.expr),
        swc_ast::MemberProp::PrivateName(_) => Expression::Identifier(Identifier {
            name: "#private".to_string(),
            loc: SourceLocation::default(),
        }),
    };
    Expression::MemberExpression {
        object: Box::new(swc_expr_to_ast(*member.obj)),
        property: Box::new(property),
        computed: is_computed,
        loc: SourceLocation::default(),
    }
}

fn swc_lit_to_expr(lit: swc_ast::Lit) -> Expression {
    match lit {
        swc_ast::Lit::Str(s) => Expression::Literal {
            value: LiteralValue::String(s.value.to_string_lossy().to_string()),
            raw: s.raw.map(|r| String::from(&*r)).unwrap_or_default(),
            loc: SourceLocation::default(),
        },
        swc_ast::Lit::Num(n) => Expression::Literal {
            value: LiteralValue::Number(n.value),
            raw: n.raw.map(|r| r.to_string()).unwrap_or_default(),
            loc: SourceLocation::default(),
        },
        swc_ast::Lit::Bool(b) => Expression::Literal {
            value: LiteralValue::Boolean(b.value),
            raw: if b.value { "true" } else { "false" }.to_string(),
            loc: SourceLocation::default(),
        },
        swc_ast::Lit::Null(_) => Expression::Literal {
            value: LiteralValue::Null,
            raw: "null".to_string(),
            loc: SourceLocation::default(),
        },
        swc_ast::Lit::BigInt(b) => Expression::Literal {
            value: LiteralValue::Number(b.value.to_string().parse().unwrap_or(0.0)),
            raw: b.raw.map(|r| r.to_string()).unwrap_or_default(),
            loc: SourceLocation::default(),
        },
        swc_ast::Lit::Regex(regex) => Expression::Literal {
            value: LiteralValue::RegExp {
                pattern: regex.exp.to_string(),
                flags: regex.flags.to_string(),
            },
            raw: format!("/{}/{}", regex.exp, regex.flags),
            loc: SourceLocation::default(),
        },
        _ => Expression::Literal {
            value: LiteralValue::Null,
            raw: "".to_string(),
            loc: SourceLocation::default(),
        },
    }
}

fn swc_bin_op_to_op(op: swc_ast::BinaryOp) -> BinaryOperator {
    match op {
        swc_ast::BinaryOp::Add => BinaryOperator::Add,
        swc_ast::BinaryOp::Sub => BinaryOperator::Subtract,
        swc_ast::BinaryOp::Mul => BinaryOperator::Multiply,
        swc_ast::BinaryOp::Div => BinaryOperator::Divide,
        swc_ast::BinaryOp::Mod => BinaryOperator::Modulo,
        swc_ast::BinaryOp::EqEq => BinaryOperator::Equal,
        swc_ast::BinaryOp::NotEq => BinaryOperator::NotEqual,
        swc_ast::BinaryOp::EqEqEq => BinaryOperator::StrictEqual,
        swc_ast::BinaryOp::NotEqEq => BinaryOperator::StrictNotEqual,
        swc_ast::BinaryOp::Lt => BinaryOperator::LessThan,
        swc_ast::BinaryOp::LtEq => BinaryOperator::LessThanEqual,
        swc_ast::BinaryOp::Gt => BinaryOperator::GreaterThan,
        swc_ast::BinaryOp::GtEq => BinaryOperator::GreaterThanEqual,
        swc_ast::BinaryOp::LShift => BinaryOperator::ShiftLeft,
        swc_ast::BinaryOp::RShift => BinaryOperator::ShiftRight,
        swc_ast::BinaryOp::ZeroFillRShift => BinaryOperator::ShiftRightUnsigned,
        swc_ast::BinaryOp::BitAnd => BinaryOperator::BitAnd,
        swc_ast::BinaryOp::BitOr => BinaryOperator::BitOr,
        swc_ast::BinaryOp::BitXor => BinaryOperator::LogicalAnd,
        swc_ast::BinaryOp::In => BinaryOperator::In,
        swc_ast::BinaryOp::InstanceOf => BinaryOperator::InstanceOf,
        swc_ast::BinaryOp::Exp => BinaryOperator::Exponent,
        _ => BinaryOperator::LogicalAnd,
    }
}

fn swc_unary_op_to_op(op: swc_ast::UnaryOp) -> UnaryOperator {
    match op {
        swc_ast::UnaryOp::Minus => UnaryOperator::Minus,
        swc_ast::UnaryOp::Plus => UnaryOperator::Plus,
        swc_ast::UnaryOp::Bang => UnaryOperator::Not,
        swc_ast::UnaryOp::Tilde => UnaryOperator::BitNot,
        swc_ast::UnaryOp::TypeOf => UnaryOperator::TypeOf,
        swc_ast::UnaryOp::Void => UnaryOperator::Void,
        swc_ast::UnaryOp::Delete => UnaryOperator::Delete,
    }
}

fn swc_assign_op_to_op(op: swc_ast::AssignOp) -> AssignmentOperator {
    match op {
        swc_ast::AssignOp::Assign => AssignmentOperator::Simple,
        swc_ast::AssignOp::AddAssign => AssignmentOperator::Plus,
        swc_ast::AssignOp::SubAssign => AssignmentOperator::Minus,
        swc_ast::AssignOp::MulAssign => AssignmentOperator::Multiply,
        swc_ast::AssignOp::DivAssign => AssignmentOperator::Divide,
        swc_ast::AssignOp::ModAssign => AssignmentOperator::Modulo,
        swc_ast::AssignOp::LShiftAssign => AssignmentOperator::ShiftLeft,
        swc_ast::AssignOp::RShiftAssign => AssignmentOperator::ShiftRight,
        swc_ast::AssignOp::ZeroFillRShiftAssign => AssignmentOperator::ShiftRightUnsigned,
        swc_ast::AssignOp::BitAndAssign => AssignmentOperator::BitAnd,
        swc_ast::AssignOp::BitOrAssign => AssignmentOperator::BitOr,
        swc_ast::AssignOp::BitXorAssign => AssignmentOperator::BitXor,
        _ => AssignmentOperator::Simple,
    }
}

fn swc_prop_to_prop(prop: swc_ast::Prop) -> Option<ObjectProperty> {
    match prop {
        swc_ast::Prop::KeyValue(kv) => Some(ObjectProperty {
            key: swc_prop_name_to_expr(kv.key),
            value: swc_expr_to_ast(*kv.value),
            kind: PropertyKind::Init,
            method: false,
            shorthand: false,
            computed: false,
            loc: SourceLocation::default(),
        }),
        swc_ast::Prop::Shorthand(ident) => Some(ObjectProperty {
            key: Expression::Identifier(Identifier {
                name: String::from(&*ident.sym),
                loc: SourceLocation::default(),
            }),
            value: Expression::Identifier(Identifier {
                name: String::from(&*ident.sym),
                loc: SourceLocation::default(),
            }),
            kind: PropertyKind::Init,
            method: false,
            shorthand: true,
            computed: false,
            loc: SourceLocation::default(),
        }),
        swc_ast::Prop::Getter(getter) => {
            let _body_expr = if let Some(body) = getter.body {
                swc_block_stmt_to_ast(body)
            } else {
                Statement::EmptyStatement {
                    loc: SourceLocation::default(),
                }
            };
            Some(ObjectProperty {
                key: swc_prop_name_to_expr(getter.key),
                value: Expression::Identifier(Identifier {
                    name: "getter_body".to_string(),
                    loc: SourceLocation::default(),
                }),
                kind: PropertyKind::Get,
                method: false,
                shorthand: false,
                computed: false,
                loc: SourceLocation::default(),
            })
        }
        swc_ast::Prop::Setter(setter) => {
            Some(ObjectProperty {
                key: swc_prop_name_to_expr(setter.key),
                value: Expression::Identifier(Identifier {
                    name: "setter_body".to_string(),
                    loc: SourceLocation::default(),
                }),
                kind: PropertyKind::Set,
                method: false,
                shorthand: false,
                computed: false,
                loc: SourceLocation::default(),
            })
        }
        swc_ast::Prop::Method(method) => Some(ObjectProperty {
            key: swc_prop_name_to_expr(method.key),
            value: swc_expr_to_ast(swc_ast::Expr::Fn(swc_ast::FnExpr {
                ident: None,
                function: method.function,
            })),
            kind: PropertyKind::Init,
            method: true,
            shorthand: false,
            computed: false,
            loc: SourceLocation::default(),
        }),
        _ => None,
    }
}

fn swc_for_stmt_to_stmt(for_stmt: swc_ast::ForStmt) -> Statement {
    Statement::ForStatement {
        init: for_stmt.init.map(swc_for_init_to_stmt),
        test: for_stmt.test.map(|e| swc_expr_to_ast(*e)),
        update: for_stmt.update.map(|e| swc_expr_to_ast(*e)),
        body: Box::new(swc_stmt_to_stmt(*for_stmt.body).unwrap_or(Statement::EmptyStatement { loc: SourceLocation::default() })),
        loc: SourceLocation::default(),
    }
}

fn swc_for_init_to_stmt(init: swc_ast::VarDeclOrExpr) -> ForStatementInit {
    match init {
        swc_ast::VarDeclOrExpr::Expr(expr) => ForStatementInit::Expression(swc_expr_to_ast(*expr)),
        swc_ast::VarDeclOrExpr::VarDecl(decl) => ForStatementInit::Variable(Box::new(Statement::VariableDeclaration {
            kind: match decl.kind {
                swc_ast::VarDeclKind::Var => VariableDeclarationKind::Var,
                swc_ast::VarDeclKind::Let => VariableDeclarationKind::Let,
                swc_ast::VarDeclKind::Const => VariableDeclarationKind::Const,
            },
            declarations: decl.decls.into_iter().map(swc_var_declarator_to_pattern).collect(),
            loc: SourceLocation::default(),
        })),
    }
}

fn swc_for_in_stmt_to_stmt(for_in: swc_ast::ForInStmt) -> Statement {
    let left = match for_in.left {
        swc_ast::ForHead::VarDecl(decl) => ForStatementLeft::Variable(Box::new(Statement::VariableDeclaration {
            kind: match decl.kind {
                swc_ast::VarDeclKind::Var => VariableDeclarationKind::Var,
                swc_ast::VarDeclKind::Let => VariableDeclarationKind::Let,
                swc_ast::VarDeclKind::Const => VariableDeclarationKind::Const,
            },
            declarations: decl.decls.into_iter().map(swc_var_declarator_to_pattern).collect(),
            loc: SourceLocation::default(),
        })),
        swc_ast::ForHead::Pat(pat) => ForStatementLeft::Pattern(swc_pat_to_pattern(*pat)),
        _ => ForStatementLeft::Pattern(Pattern::Identifier {
            id: Identifier {
                name: "unknown".to_string(),
                loc: SourceLocation::default(),
            },
            init: None,
        }),
    };
    Statement::ForInStatement {
        left,
        right: swc_expr_to_ast(*for_in.right),
        body: Box::new(swc_stmt_to_stmt(*for_in.body).unwrap_or(Statement::EmptyStatement { loc: SourceLocation::default() })),
        loc: SourceLocation::default(),
    }
}

fn swc_for_of_stmt_to_stmt(for_of: swc_ast::ForOfStmt) -> Statement {
    let left = match for_of.left {
        swc_ast::ForHead::VarDecl(decl) => ForStatementLeft::Variable(Box::new(Statement::VariableDeclaration {
            kind: match decl.kind {
                swc_ast::VarDeclKind::Var => VariableDeclarationKind::Var,
                swc_ast::VarDeclKind::Let => VariableDeclarationKind::Let,
                swc_ast::VarDeclKind::Const => VariableDeclarationKind::Const,
            },
            declarations: decl.decls.into_iter().map(swc_var_declarator_to_pattern).collect(),
            loc: SourceLocation::default(),
        })),
        swc_ast::ForHead::Pat(pat) => ForStatementLeft::Pattern(swc_pat_to_pattern(*pat)),
        _ => ForStatementLeft::Pattern(Pattern::Identifier {
            id: Identifier {
                name: "unknown".to_string(),
                loc: SourceLocation::default(),
            },
            init: None,
        }),
    };
    Statement::ForOfStatement {
        left,
        right: swc_expr_to_ast(*for_of.right),
        body: Box::new(swc_stmt_to_stmt(*for_of.body).unwrap_or(Statement::EmptyStatement { loc: SourceLocation::default() })),
        loc: SourceLocation::default(),
    }
}

fn swc_switch_case_to_case(case: swc_ast::SwitchCase) -> SwitchCase {
    SwitchCase {
        test: case.test.map(|e| swc_expr_to_ast(*e)),
        consequent: case.cons.into_iter().filter_map(swc_stmt_to_stmt).collect(),
        loc: SourceLocation::default(),
    }
}
