use std::collections::HashMap;
use serde::{Serialize, Deserialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum IrType {
    Void,
    I32,
    I64,
    F32,
    F64,
    Bool,
    Ptr,
    String,
    Struct(String),
    Array(Box<IrType>, usize),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Param {
    pub name: String,
    pub ty: IrType,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LocalVar {
    pub name: String,
    pub ty: IrType,
    pub init: Option<Expr>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum Expr {
    ConstI32(i32),
    ConstI64(i64),
    ConstF32(f32),
    ConstF64(f64),
    ConstBool(bool),
    ConstString(String),
    ConstNull,
    
    Var(String),
    FieldAccess {
        base: Box<Expr>,
        field: String,
    },
    Index {
        base: Box<Expr>,
        index: Box<Expr>,
    },
    
    BinaryOp {
        op: BinOp,
        left: Box<Expr>,
        right: Box<Expr>,
    },
    UnaryOp {
        op: UnaryOp,
        expr: Box<Expr>,
    },
    
    Call {
        func: String,
        args: Vec<Expr>,
    },
    MethodCall {
        base: Box<Expr>,
        method: String,
        args: Vec<Expr>,
    },
    
    New {
        ty: String,
        args: Vec<Expr>,
    },
    Cast {
        expr: Box<Expr>,
        target_ty: IrType,
    },
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum BinOp {
    Add,
    Sub,
    Mul,
    Div,
    Mod,
    And,
    Or,
    Eq,
    Ne,
    Lt,
    Le,
    Gt,
    Ge,
    BitAnd,
    BitOr,
    BitXor,
    Shl,
    Shr,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum UnaryOp {
    Neg,
    Not,
    Deref,
    Ref,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum Stmt {
    VarDecl(LocalVar),
    Assign {
        target: Expr,
        value: Expr,
    },
    ExprStmt(Expr),
    
    If {
        cond: Expr,
        then_block: Vec<Stmt>,
        else_block: Option<Vec<Stmt>>,
    },
    While {
        cond: Expr,
        body: Vec<Stmt>,
    },
    For {
        init: Box<Stmt>,
        cond: Expr,
        update: Box<Stmt>,
        body: Vec<Stmt>,
    },
    Return(Option<Expr>),
    Break,
    Continue,
    
    Block(Vec<Stmt>),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Function {
    pub name: String,
    pub params: Vec<Param>,
    pub return_ty: IrType,
    pub body: Vec<Stmt>,
    pub is_pub: bool,
    pub is_extern: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Struct {
    pub name: String,
    pub fields: Vec<(String, IrType)>,
    pub is_pub: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Enum {
    pub name: String,
    pub variants: Vec<(String, Option<IrType>)>,
    pub is_pub: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Module {
    pub name: String,
    pub functions: HashMap<String, Function>,
    pub structs: HashMap<String, Struct>,
    pub enums: HashMap<String, Enum>,
    pub imports: Vec<String>,
    pub exports: Vec<String>,
}

impl Module {
    pub fn new(name: String) -> Self {
        Self {
            name,
            functions: HashMap::new(),
            structs: HashMap::new(),
            enums: HashMap::new(),
            imports: Vec::new(),
            exports: Vec::new(),
        }
    }
    
    pub fn add_function(&mut self, func: Function) {
        self.functions.insert(func.name.clone(), func);
    }
    
    pub fn add_struct(&mut self, s: Struct) {
        self.structs.insert(s.name.clone(), s);
    }
    
    pub fn add_enum(&mut self, e: Enum) {
        self.enums.insert(e.name.clone(), e);
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Program {
    pub modules: Vec<Module>,
    pub entry_point: Option<String>,
    pub bridge_api: BridgeApi,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BridgeApi {
    pub create_bridge: String,
    pub set_html: String,
    pub set_css: String,
    pub render: String,
    pub on_click: String,
    pub eval_js: String,
}

impl Default for BridgeApi {
    fn default() -> Self {
        Self {
            create_bridge: "WebNativeBridge::new".to_string(),
            set_html: "bridge.set_html".to_string(),
            set_css: "bridge.set_css".to_string(),
            render: "bridge.render".to_string(),
            on_click: "bridge.on_click".to_string(),
            eval_js: "bridge.eval_js".to_string(),
        }
    }
}

impl Program {
    pub fn new() -> Self {
        Self {
            modules: Vec::new(),
            entry_point: None,
            bridge_api: BridgeApi::default(),
        }
    }
    
    pub fn with_entry_point(mut self, entry: String) -> Self {
        self.entry_point = Some(entry);
        self
    }
}
