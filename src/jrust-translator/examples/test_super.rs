
//! 测试 super 关键字的编译

use jrust_translator::*;

fn main() {
    let source = "super.foo();";
    
    println!("Compiling: {}", source);
    match compile(source) {
        Ok(result) => {
            println!("\nGenerated Rust code:");
            println!("{}", result.code);
        }
        Err(e) => {
            eprintln!("Error: {:?}", e);
        }
    }
}
