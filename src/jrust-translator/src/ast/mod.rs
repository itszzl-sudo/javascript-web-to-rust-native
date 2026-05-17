use std::fmt;

#[derive(Debug, Clone, PartialEq)]
pub enum LiteralValue {
    Null,
    Undefined,
    Boolean(bool),
    Number(f64),
    String(String),
    RegExp { pattern: String, flags: String },
}

#[derive(Debug, Clone, PartialEq)]
pub struct Identifier {
    pub name: String,
    pub loc: SourceLocation,
}

#[derive(Debug, Clone, PartialEq)]
pub struct SourceLocation {
    pub start: Position,
    pub end: Position,
}

#[derive(Debug, Clone, PartialEq)]
pub struct Position {
    pub line: u32,
    pub column: u32,
}

impl Position {
    pub fn new(line: u32, column: u32) -> Self {
        Self { line, column }
    }
}

impl SourceLocation {
    pub fn new(start: Position, end: Position) -> Self {
        Self { start, end }
    }
}

impl Default for SourceLocation {
    fn default() -> Self {
        Self {
            start: Position::default(),
            end: Position::default(),
        }
    }
}

impl Default for Position {
    fn default() -> Self {
        Self { line: 1, column: 0 }
    }
}

impl fmt::Display for Position {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}:{}", self.line, self.column)
    }
}

pub const COMMENT_DEBUGGER: &str = "// debugger";
pub const COMMENT_WITH_DEPRECATED: &str = "// with statement (deprecated in strict mode)";
pub const RUST_NONE: &str = "None";
pub const RUST_UNIT: &str = "()";
pub const RUST_SELF: &str = "self";
pub const RUST_TRUE: &str = "true";
pub const RUST_FALSE: &str = "false";
pub const RUST_NAN: &str = "f64::NAN";
pub const RUST_INFINITY: &str = "f64::INFINITY";
pub const RUST_NEG_INFINITY: &str = "f64::NEG_INFINITY";
pub const INDENT: &str = "    ";
pub const MAX_FILE_SIZE: usize = 10 * 1024 * 1024;
pub const MAX_RECURSION_DEPTH: usize = 1000;

#[derive(Debug, Clone, PartialEq)]
pub enum Expression {
    Identifier(Identifier),
    Literal {
        value: LiteralValue,
        raw: String,
        loc: SourceLocation,
    },
    This {
        loc: SourceLocation,
    },
    ArrayExpression {
        elements: Vec<Option<Expression>>,
        loc: SourceLocation,
    },
    ObjectExpression {
        properties: Vec<ObjectProperty>,
        loc: SourceLocation,
    },
    FunctionExpression {
        id: Option<Identifier>,
        params: Vec<Pattern>,
        body: Box<FunctionBody>,
        loc: SourceLocation,
    },
    ArrowFunctionExpression {
        params: Vec<Pattern>,
        body: Box<ArrowFunctionBody>,
        loc: SourceLocation,
    },
    ClassExpression {
        id: Option<Identifier>,
        super_class: Option<Box<Expression>>,
        body: Box<ClassBody>,
        loc: SourceLocation,
    },
    TaggedTemplateExpression {
        tag: Box<Expression>,
        quasi: Box<Expression>,
        loc: SourceLocation,
    },
    MemberExpression {
        object: Box<Expression>,
        property: Box<Expression>,
        computed: bool,
        loc: SourceLocation,
    },
    CallExpression {
        callee: Box<Expression>,
        arguments: Vec<Expression>,
        loc: SourceLocation,
    },
    NewExpression {
        callee: Box<Expression>,
        arguments: Vec<Expression>,
        loc: SourceLocation,
    },
    UnaryExpression {
        operator: UnaryOperator,
        argument: Box<Expression>,
        prefix: bool,
        loc: SourceLocation,
    },
    BinaryExpression {
        operator: BinaryOperator,
        left: Box<Expression>,
        right: Box<Expression>,
        loc: SourceLocation,
    },
    LogicalExpression {
        operator: LogicalOperator,
        left: Box<Expression>,
        right: Box<Expression>,
        loc: SourceLocation,
    },
    NullishCoalescingExpression {
        left: Box<Expression>,
        right: Box<Expression>,
        loc: SourceLocation,
    },
    OptionalMemberExpression {
        object: Box<Expression>,
        property: Box<Expression>,
        computed: bool,
        loc: SourceLocation,
    },
    OptionalCallExpression {
        callee: Box<Expression>,
        arguments: Vec<Expression>,
        loc: SourceLocation,
    },
    AssignmentExpression {
        operator: AssignmentOperator,
        left: Box<PatternOrExpression>,
        right: Box<Expression>,
        loc: SourceLocation,
    },
    UpdateExpression {
        operator: UpdateOperator,
        argument: Box<Expression>,
        prefix: bool,
        loc: SourceLocation,
    },
    AwaitExpression {
        argument: Box<Expression>,
        loc: SourceLocation,
    },
    YieldExpression {
        argument: Option<Box<Expression>>,
        delegate: bool,
        loc: SourceLocation,
    },
    ConditionalExpression {
        test: Box<Expression>,
        consequent: Box<Expression>,
        alternate: Box<Expression>,
        loc: SourceLocation,
    },
    SequenceExpression {
        expressions: Vec<Expression>,
        loc: SourceLocation,
    },
    TemplateLiteral {
        quasis: Vec<TemplateElement>,
        expressions: Vec<Expression>,
        loc: SourceLocation,
    },
    SpreadElement {
        argument: Box<Expression>,
        loc: SourceLocation,
    },
    Super {
        loc: SourceLocation,
    },
    Import {
        loc: SourceLocation,
    },
}

#[derive(Debug, Clone, PartialEq)]
pub enum Pattern {
    Identifier {
        id: Identifier,
        init: Option<Box<Expression>>,
    },
    ObjectPattern {
        properties: Vec<ObjectPatternProperty>,
        loc: SourceLocation,
    },
    ArrayPattern {
        elements: Vec<Option<Pattern>>,
        loc: SourceLocation,
    },
    RestElement {
        argument: Box<Pattern>,
        loc: SourceLocation,
    },
    AssignmentPattern {
        left: Box<Pattern>,
        right: Box<Expression>,
        loc: SourceLocation,
    },
}

pub type PatternOrExpression = Expression;

#[derive(Debug, Clone, PartialEq)]
pub struct ObjectProperty {
    pub key: Expression,
    pub value: Expression,
    pub kind: PropertyKind,
    pub method: bool,
    pub shorthand: bool,
    pub computed: bool,
    pub loc: SourceLocation,
}

#[derive(Debug, Clone, PartialEq)]
pub enum PropertyKind {
    Init,
    Get,
    Set,
}

#[derive(Debug, Clone, PartialEq)]
pub struct ObjectPatternProperty {
    pub key: Expression,
    pub value: Option<Pattern>,
    pub computed: bool,
    pub loc: SourceLocation,
}

#[derive(Debug, Clone, PartialEq)]
pub enum AssignmentOperator {
    Simple,
    Plus,
    Minus,
    Multiply,
    Divide,
    Modulo,
    BitAnd,
    BitOr,
    BitXor,
    ShiftLeft,
    ShiftRight,
    ShiftRightUnsigned,
}

#[derive(Debug, Clone, PartialEq)]
pub enum UpdateOperator {
    Increment,
    Decrement,
}

#[derive(Debug, Clone, PartialEq)]
pub enum UnaryOperator {
    Minus,
    Plus,
    Not,
    BitNot,
    TypeOf,
    Void,
    Delete,
}

#[derive(Debug, Clone, PartialEq)]
pub enum BinaryOperator {
    Exponent,
    Multiply,
    Divide,
    Modulo,
    Add,
    Subtract,
    ShiftLeft,
    ShiftRight,
    ShiftRightUnsigned,
    LessThan,
    LessThanEqual,
    GreaterThan,
    GreaterThanEqual,
    In,
    InstanceOf,
    Equal,
    NotEqual,
    StrictEqual,
    StrictNotEqual,
    BitAnd,
    BitXor,
    BitOr,
    LogicalAnd,
    LogicalOr,
    NullishCoalescing,
}

#[derive(Debug, Clone, PartialEq)]
pub enum LogicalOperator {
    And,
    Or,
}

#[derive(Debug, Clone, PartialEq)]
pub enum BinaryBitwiseOperator {
    BitAnd,
    BitXor,
    BitOr,
}

#[derive(Debug, Clone, PartialEq)]
pub struct TemplateElement {
    pub value: TemplateElementValue,
    pub tail: bool,
    pub loc: SourceLocation,
}

#[derive(Debug, Clone, PartialEq)]
pub struct TemplateElementValue {
    pub raw: String,
    pub cooked: String,
}

#[derive(Debug, Clone, PartialEq)]
pub struct SwitchCase {
    pub test: Option<Expression>,
    pub consequent: Vec<Statement>,
    pub loc: SourceLocation,
}

#[derive(Debug, Clone, PartialEq)]
pub struct CatchClause {
    pub param: Pattern,
    pub body: Box<Statement>,
    pub loc: SourceLocation,
}

#[derive(Debug, Clone, PartialEq)]
pub struct FunctionBody {
    pub body: Vec<Statement>,
    pub loc: SourceLocation,
}

#[derive(Debug, Clone, PartialEq)]
pub enum ArrowFunctionBody {
    Expression(Expression),
    BlockFunctionBody(FunctionBody),
}

#[derive(Debug, Clone, PartialEq)]
pub struct ClassBody {
    pub body: Vec<ClassElement>,
    pub loc: SourceLocation,
}

#[derive(Debug, Clone, PartialEq)]
pub struct ClassElement {
    pub key: Option<Expression>,
    pub value: Option<MethodDefinition>,
    pub kind: ClassElementKind,
    pub computed: bool,
    pub static_: bool,
    pub loc: SourceLocation,
}

#[derive(Debug, Clone, PartialEq)]
pub enum ClassElementKind {
    Method,
    Constructor,
    Field,
    Get,
    Set,
}

#[derive(Debug, Clone, PartialEq)]
pub struct MethodDefinition {
    pub key: Expression,
    pub value: Box<FunctionExpression>,
    pub kind: ClassElementKind,
    pub computed: bool,
    pub static_: bool,
    pub loc: SourceLocation,
}

#[derive(Debug, Clone, PartialEq)]
pub enum FunctionExpression {
    FunctionExpression {
        id: Option<Identifier>,
        params: Vec<Pattern>,
        body: Box<FunctionBody>,
        generator: bool,
        async_: bool,
        loc: SourceLocation,
    },
}

impl FunctionExpression {
    pub fn new_function(
        params: Vec<Pattern>,
        body: Vec<Statement>,
        loc: SourceLocation,
    ) -> Self {
        Self::FunctionExpression {
            id: None,
            params,
            body: Box::new(FunctionBody { body, loc: loc.clone() }),
            generator: false,
            async_: false,
            loc,
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum Statement {
    ExpressionStatement {
        expression: Expression,
        loc: SourceLocation,
    },
    BlockStatement {
        body: Vec<Statement>,
        loc: SourceLocation,
    },
    EmptyStatement {
        loc: SourceLocation,
    },
    DebuggerStatement {
        loc: SourceLocation,
    },
    WithStatement {
        object: Expression,
        body: Box<Statement>,
        loc: SourceLocation,
    },
    ReturnStatement {
        argument: Option<Expression>,
        loc: SourceLocation,
    },
    LabeledStatement {
        label: Identifier,
        body: Box<Statement>,
        loc: SourceLocation,
    },
    BreakStatement {
        label: Option<Identifier>,
        loc: SourceLocation,
    },
    ContinueStatement {
        label: Option<Identifier>,
        loc: SourceLocation,
    },
    IfStatement {
        test: Expression,
        consequent: Box<Statement>,
        alternate: Option<Box<Statement>>,
        loc: SourceLocation,
    },
    SwitchStatement {
        discriminant: Expression,
        cases: Vec<SwitchCase>,
        loc: SourceLocation,
    },
    ThrowStatement {
        argument: Expression,
        loc: SourceLocation,
    },
    TryStatement {
        block: Box<Statement>,
        handler: Option<Box<CatchClause>>,
        finalizer: Option<Box<Statement>>,
        loc: SourceLocation,
    },
    WhileStatement {
        test: Expression,
        body: Box<Statement>,
        loc: SourceLocation,
    },
    DoWhileStatement {
        body: Box<Statement>,
        test: Expression,
        loc: SourceLocation,
    },
    ForStatement {
        init: Option<ForStatementInit>,
        test: Option<Expression>,
        update: Option<Expression>,
        body: Box<Statement>,
        loc: SourceLocation,
    },
    ForInStatement {
        left: ForStatementLeft,
        right: Expression,
        body: Box<Statement>,
        loc: SourceLocation,
    },
    ForOfStatement {
        left: ForStatementLeft,
        right: Expression,
        body: Box<Statement>,
        loc: SourceLocation,
    },
    FunctionDeclaration {
        id: Identifier,
        params: Vec<Pattern>,
        body: Box<FunctionBody>,
        generator: bool,
        async_: bool,
        loc: SourceLocation,
    },
    ClassDeclaration {
        id: Identifier,
        super_class: Option<Box<Expression>>,
        body: Box<ClassBody>,
        loc: SourceLocation,
    },
    ImportDeclaration {
        specifiers: Vec<ImportSpecifier>,
        source: String,
        loc: SourceLocation,
    },
    ExportNamedDeclaration {
        declaration: Option<Box<Statement>>,
        specifiers: Vec<ExportSpecifier>,
        source: Option<String>,
        loc: SourceLocation,
    },
    ExportDefaultDeclaration {
        declaration: Box<Statement>,
        loc: SourceLocation,
    },
    ExportAllDeclaration {
        source: String,
        loc: SourceLocation,
    },
    VariableDeclaration {
        declarations: Vec<VariableDeclarator>,
        kind: VariableDeclarationKind,
        loc: SourceLocation,
    },
}

#[derive(Debug, Clone, PartialEq)]
pub enum ForStatementInit {
    Variable(Box<Statement>),
    Expression(Expression),
}

#[derive(Debug, Clone, PartialEq)]
pub enum ForStatementLeft {
    Variable(Box<Statement>),
    Pattern(Pattern),
}

#[derive(Debug, Clone, PartialEq)]
pub struct ImportSpecifier {
    pub imported: Identifier,
    pub local: Identifier,
    pub loc: SourceLocation,
}

#[derive(Debug, Clone, PartialEq)]
pub struct ExportSpecifier {
    pub local: Identifier,
    pub exported: Option<Identifier>,
    pub loc: SourceLocation,
}

#[derive(Debug, Clone, PartialEq)]
pub enum VariableDeclarationKind {
    Var,
    Let,
    Const,
}

#[derive(Debug, Clone, PartialEq)]
pub struct VariableDeclarator {
    pub id: Pattern,
    pub init: Option<Expression>,
    pub loc: SourceLocation,
}

#[derive(Debug, Clone, PartialEq)]
pub enum SourceType {
    Script,
    Module,
}

#[derive(Debug, Clone, PartialEq)]
pub struct Program {
    pub source_type: SourceType,
    pub body: Vec<Statement>,
    pub loc: SourceLocation,
}
