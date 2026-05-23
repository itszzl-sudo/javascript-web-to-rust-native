pub mod ast;
pub mod error;
pub mod parser;
pub mod swc_parser;
pub mod analyzer;
pub mod codegen;
pub mod compiler;
pub mod ir_gen;
pub mod splitter;
pub mod framework;
pub mod external_deps;
pub mod dep_processor;
pub mod resource;

pub use compiler::{CompileResult, Compiler};
pub use error::{Error, Result};
pub use ir_gen::IrGenerator;
pub use splitter::{CodeSplitter, SplitAnalysis, CodeRole};
pub use framework::{Framework, detect_framework, FrameworkDetectionResult};
pub use external_deps::{
    ExternalDepDetector, ExternalDependency, ExternalDepType,
    detect_external_deps,
};
pub use dep_processor::{
    ExternalDepProcessor, ProcessError, process_external_deps,
};

pub fn compile(source: &str) -> Result<CompileResult> {
    let mut compiler = Compiler::new();
    compiler.compile(source)
}

#[cfg(test)]
mod tests {
    use crate::{compile, Compiler};
    use crate::analyzer::Analyzer;
    use crate::ast::*;

    #[test]
    fn test_literal_number() {
        let source = "42;";
        let result = compile(source).unwrap();
        assert!(result.code.contains("42"));
    }

    #[test]
    fn test_literal_string() {
        let source = r#""hello";"#;
        let result = compile(source).unwrap();
        assert!(result.code.contains("\"hello\""));
    }

    #[test]
    fn test_literal_boolean_true() {
        let source = "true;";
        let result = compile(source).unwrap();
        assert!(result.code.contains("true"));
    }

    #[test]
    fn test_literal_boolean_false() {
        let source = "false;";
        let result = compile(source).unwrap();
        assert!(result.code.contains("false"));
    }

    #[test]
    fn test_literal_null() {
        let source = "null;";
        let result = compile(source).unwrap();
        assert!(result.code.contains("None"));
    }

    #[test]
    fn test_literal_undefined() {
        let source = "undefined;";
        let result = compile(source).unwrap();
        assert!(result.code.contains("()"));
    }

    #[test]
    fn test_variable_let() {
        let source = "let x = 5;";
        let result = compile(source).unwrap();
        assert!(result.code.contains("let x"));
    }

    #[test]
    fn test_variable_const() {
        let source = "const y = 10;";
        let result = compile(source).unwrap();
        assert!(result.code.contains("let y"));
    }

    #[test]
    fn test_binary_expression_add() {
        let source = "1 + 2;";
        let result = compile(source).unwrap();
        assert!(result.code.contains("1 + 2"));
    }

    #[test]
    fn test_binary_expression_multiply() {
        let source = "3 * 4;";
        let result = compile(source).unwrap();
        assert!(result.code.contains("3 * 4"));
    }

    #[test]
    fn test_assignment() {
        let source = "x = 5;";
        let result = compile(source).unwrap();
        assert!(result.code.contains("x = 5"));
    }

    #[test]
    fn test_function_declaration() {
        let source = "function add(a, b) { return a + b; }";
        let result = compile(source).unwrap();
        assert!(result.code.contains("fn add(a, b)"));
        assert!(result.code.contains("return"));
    }

    #[test]
    fn test_if_statement() {
        let source = "if (x > 0) { x = 1; }";
        let result = compile(source).unwrap();
        assert!(result.code.contains("if"));
        assert!(result.code.contains("x > 0"));
    }

    #[test]
    fn test_if_else_statement() {
        let source = "if (x > 0) { x = 1; } else { x = 0; }";
        let result = compile(source).unwrap();
        assert!(result.code.contains("if"));
        assert!(result.code.contains("else"));
    }

    #[test]
    fn test_for_loop() {
        let source = "for (let i = 0; i < 10; i++) { x = i; }";
        let result = compile(source).unwrap();
        assert!(result.code.contains("for"));
    }

    #[test]
    fn test_while_loop() {
        let source = "while (x < 10) { x = x + 1; }";
        let result = compile(source).unwrap();
        assert!(result.code.contains("while"));
    }

    #[test]
    fn test_return_statement() {
        let source = "function foo() { return 42; }";
        let result = compile(source).unwrap();
        assert!(result.code.contains("return 42"));
    }

    #[test]
    fn test_call_expression() {
        let source = "console.log(1);";
        let result = compile(source).unwrap();
        assert!(result.code.contains("console.log"));
    }

    #[test]
    fn test_member_expression() {
        let source = "obj.prop;";
        let result = compile(source).unwrap();
        assert!(result.code.contains("obj.prop"));
    }

    #[test]
    fn test_array_expression() {
        let source = "[1, 2, 3];";
        let result = compile(source).unwrap();
        assert!(result.code.contains("[1, 2, 3]"));
    }

    #[test]
    fn test_object_expression() {
        let source = "({ a: 1, b: 2 });";
        let result = compile(source).unwrap();
        eprintln!("Generated code: {}", result.code);
        assert!(result.code.contains("a: 1"));
        assert!(result.code.contains("b: 2"));
    }

    #[test]
    fn test_unary_not() {
        let source = "!x;";
        let result = compile(source).unwrap();
        assert!(result.code.contains("!x"));
    }

    #[test]
    fn test_conditional_expression() {
        let source = "x ? 1 : 2;";
        let result = compile(source).unwrap();
        assert!(result.code.contains("if"));
    }

    #[test]
    fn test_identifier_sanitization_keyword() {
        let source = "let type_ = 5;";
        let result = compile(source).unwrap();
        assert!(result.code.contains("type_"));
    }

    #[test]
    fn test_identifier_sanitization_numeric_start() {
        let source = "let _2fast = 5;";
        let result = compile(source).unwrap();
        assert!(result.code.contains("_2fast"));
    }

    #[test]
    fn test_block_statement() {
        let source = "{ let x = 1; let y = 2; }";
        let result = compile(source).unwrap();
        assert!(result.code.contains("{"));
        assert!(result.code.contains("}"));
    }

    #[test]
    fn test_empty_statement() {
        let source = ";";
        let result = compile(source).unwrap();
        assert!(result.code.is_empty() || result.code.contains("// Generated"));
    }

    #[test]
    fn test_break_statement() {
        let source = "break;";
        let result = compile(source).unwrap();
        assert!(result.code.contains("break"));
    }

    #[test]
    fn test_continue_statement() {
        let source = "continue;";
        let result = compile(source).unwrap();
        assert!(result.code.contains("continue"));
    }

    #[test]
    fn test_switch_statement() {
        let source = "switch (x) { case 1: a; break; default: b; }";
        let result = compile(source).unwrap();
        assert!(result.code.contains("match"));
    }

    #[test]
    fn test_try_catch() {
        let source = "try { x = 1; } catch (e) { x = 2; }";
        let result = compile(source).unwrap();
        assert!(result.code.contains("_result"));
    }

    #[test]
    fn test_class_declaration() {
        let source = "class Foo { bar() {} }";
        let result = compile(source).unwrap();
        assert!(result.code.contains("struct Foo"));
    }

    #[test]
    fn test_arrow_function() {
        let source = "(a, b) => a + b";
        let result = compile(source).unwrap();
        assert!(result.code.contains("|"));
    }

    #[test]
    fn test_arrow_function_block() {
        let source = "(a) => { return a; }";
        let result = compile(source).unwrap();
        assert!(result.code.contains("|"));
        assert!(result.code.contains("return"));
    }

    #[test]
    fn test_template_literal() {
        let source = "`hello ${name}`;";
        let result = compile(source).unwrap();
        eprintln!("Generated code: {}", result.code);
        assert!(result.code.contains("format!"));
    }

    #[test]
    fn test_debugger_statement() {
        let source = "debugger;";
        let result = compile(source).unwrap();
        assert!(result.code.contains("// debugger"));
    }

    #[test]
    fn test_import_comment() {
        let source = "import { foo } from 'module';";
        let result = compile(source).unwrap();
        assert!(result.code.contains("// import"));
    }

    #[test]
    fn test_export_comment() {
        let source = "export { foo };";
        let result = compile(source).unwrap();
        assert!(result.code.contains("// export"));
    }

    #[test]
    fn test_new_expression() {
        let source = "new Foo();";
        let result = compile(source).unwrap();
        assert!(result.code.contains("Foo::new"));
    }

    #[test]
    fn test_this() {
        let source = "this.x;";
        let result = compile(source).unwrap();
        assert!(result.code.contains("self"));
    }

    #[test]
    #[ignore] // SWC的super必须在类构造函数上下文中解析
    fn test_super() {
        let source = "super.foo();";
        let result = compile(source).unwrap();
        eprintln!("Generated code: {}", result.code);
        assert!(result.code.contains("super"));
    }

    #[test]
    fn test_spread_element() {
        let source = "[...arr];";
        let result = compile(source).unwrap();
        eprintln!("Generated code: {}", result.code);
        assert!(result.code.contains("_spread"));
    }

    #[test]
    fn test_throw_statement() {
        let source = "throw new Error('msg');";
        let result = compile(source).unwrap();
        assert!(result.code.contains("panic!"));
    }

    #[test]
    fn test_nan() {
        let source = "NaN;";
        let result = compile(source).unwrap();
        assert!(result.code.contains("f64::NAN"));
    }

    #[test]
    fn test_infinity() {
        let source = "Infinity;";
        let result = compile(source).unwrap();
        assert!(result.code.contains("f64::INFINITY"));
    }

    #[test]
    fn test_negative_infinity() {
        let source = "-Infinity;";
        let result = compile(source).unwrap();
        assert!(result.code.contains("f64::NEG_INFINITY"));
    }

    #[test]
    fn test_multiple_statements() {
        let source = "let a = 1; let b = 2; let c = 3;";
        let result = compile(source).unwrap();
        assert!(result.code.contains("let a"));
        assert!(result.code.contains("let b"));
        assert!(result.code.contains("let c"));
    }

    #[test]
    fn test_nested_expressions() {
        let source = "a + b * c - d / e;";
        let result = compile(source).unwrap();
        assert!(result.code.contains("a + (b * c)"));
    }

    #[test]
    fn test_comparison_operators() {
        let source = "a === b;";
        let result = compile(source).unwrap();
        assert!(result.code.contains("a == b"));
    }

    #[test]
    fn test_logical_operators() {
        let source = "a && b;";
        let result = compile(source).unwrap();
        eprintln!("Generated code: {}", result.code);
        assert!(result.code.contains("a && b"));
    }

    #[test]
    fn test_bitwise_operators() {
        let source = "a | b;";
        let result = compile(source).unwrap();
        assert!(result.code.contains("a | b"));
    }

    #[test]
    fn test_do_while() {
        let source = "do { x = 1; } while (x < 10);";
        let result = compile(source).unwrap();
        assert!(result.code.contains("loop"));
        assert!(result.code.contains("break"));
    }

    #[test]
    fn test_labeled_statement() {
        let source = "label: x = 1;";
        let result = compile(source).unwrap();
        assert!(result.code.contains("// label:"));
    }

    #[test]
    fn test_nested_if() {
        let source = "if (a) { if (b) { x = 1; } }";
        let result = compile(source).unwrap();
        assert!(result.code.contains("if"));
    }

    #[test]
    fn test_nested_for() {
        let source = "for (let i = 0; i < 3; i++) { for (let j = 0; j < 3; j++) { x = i + j; } }";
        let result = compile(source).unwrap();
        assert!(result.code.contains("for"));
    }

    #[test]
    fn test_call_nested() {
        let source = "foo(bar(baz()));";
        let result = compile(source).unwrap();
        assert!(result.code.contains("foo("));
        assert!(result.code.contains("bar("));
        assert!(result.code.contains("baz("));
    }

    #[test]
    fn test_member_chained() {
        let source = "a.b.c.d;";
        let result = compile(source).unwrap();
        assert!(result.code.contains("a.b"));
        assert!(result.code.contains("c.d"));
    }

    #[test]
    fn test_array_nested() {
        let source = "[[1, 2], [3, 4]];";
        let result = compile(source).unwrap();
        assert!(result.code.contains("[[1, 2], [3, 4]]"));
    }

    #[test]
    fn test_object_nested() {
        let source = "({ a: { b: 1 }, c: { d: 2 } });";
        let result = compile(source).unwrap();
        eprintln!("Generated code: {}", result.code);
        assert!(result.code.contains("a:"));
        assert!(result.code.contains("b:"));
    }

    #[test]
    fn test_with_statement() {
        let source = "with (obj) { x; }";
        let result = compile(source).unwrap();
        assert!(result.code.contains("// with"));
    }

    #[test]
    fn test_compiler_reuse() {
        let mut compiler = Compiler::new();

        let result1 = compiler.compile("let a = 1;").unwrap();
        assert!(result1.code.contains("let a"));

        let result2 = compiler.compile("let b = 2;").unwrap();
        assert!(result2.code.contains("let b"));
    }

    #[test]
    fn test_analyzer_scope() {
        let source = "let x = 1; function foo() { let y = 2; }";
        let mut compiler = Compiler::new();
        let result = compiler.compile(source).unwrap();

        assert!(!result.analysis.scopes.is_empty());
    }

    #[test]
    fn test_analyzer_functions() {
        let source = "function foo() {} function bar() {}";
        let mut compiler = Compiler::new();
        let result = compiler.compile(source).unwrap();

        assert!(result.analysis.functions.contains("foo"));
        assert!(result.analysis.functions.contains("bar"));
    }

    #[test]
    fn test_code_generation_header() {
        let source = "let x = 1;";
        let result = compile(source).unwrap();
        assert!(result.code.contains("// Generated by jrust-translator"));
    }

    #[test]
    fn test_nan_number() {
        let source = "0 / 0;";
        let result = compile(source).unwrap();
        assert!(result.code.contains("0 / 0"));
    }

    #[test]
    fn test_hex_number() {
        let source = "0xFF;";
        let result = compile(source).unwrap();
        assert!(result.code.contains("0xFF"));
    }
}
