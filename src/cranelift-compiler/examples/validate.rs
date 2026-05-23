use cranelift_compiler::{Validator, ValidateLevel};

fn main() {
    let validator = Validator::new();
    
    println!("=== 测试 1: 正确的 Rust 代码 ===");
    let valid_code = r#"
fn main() {
    let x: i32 = 42;
    let y = x + 1;
    println!("{}", y);
}
"#;
    
    match validator.validate(valid_code, ValidateLevel::Full) {
        Ok(()) => println!("✅ 验证通过\n"),
        Err(e) => println!("❌ 验证失败: {}\n", e),
    }
    
    println!("=== 测试 2: 语法错误 ===");
    let syntax_error = r#"
fn main() {
    let x = 1
    let y = 2;
}
"#;
    
    match validator.validate(syntax_error, ValidateLevel::Syntax) {
        Ok(()) => println!("✅ 验证通过\n"),
        Err(e) => println!("❌ 验证失败: {}\n", e),
    }
    
    println!("=== 测试 3: 类型错误 ===");
    let type_error = r#"
fn foo(x: i64) -> i64 {
    x + 1
}

fn main() {
    let result = foo("string");
}
"#;
    
    match validator.validate(type_error, ValidateLevel::Full) {
        Ok(()) => println!("✅ 验证通过\n"),
        Err(e) => println!("❌ 验证失败（预期）:\n{}\n", e),
    }
    
    println!("=== 测试 4: 仅语法验证（跳过类型检查）===");
    match validator.validate(type_error, ValidateLevel::Syntax) {
        Ok(()) => println!("✅ 语法验证通过（类型错误未检查）\n"),
        Err(e) => println!("❌ 验证失败: {}\n", e),
    }
}
