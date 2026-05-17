use std::env;
use std::fs;

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        print_usage();
        return;
    }

    let input_file = &args[1];
    let output_file = if args.len() > 3 && args[2] == "-o" {
        Some(&args[3])
    } else {
        None
    };

    let source = match fs::read_to_string(input_file) {
        Ok(s) => s,
        Err(e) => {
            eprintln!("Error reading file '{}': {}", input_file, e);
            std::process::exit(1);
        }
    };

    if source.len() > jrust_translator::ast::MAX_FILE_SIZE {
        eprintln!(
            "Error: File '{}' is too large ({} bytes, max: {} bytes)",
            input_file,
            source.len(),
            jrust_translator::ast::MAX_FILE_SIZE
        );
        std::process::exit(1);
    }

    match jrust_translator::compile(&source) {
        Ok(result) => {
            if let Some(output_file) = output_file {
                if let Err(e) = fs::write(output_file, &result.code) {
                    eprintln!("Error writing file '{}': {}", output_file, e);
                    std::process::exit(1);
                }
                println!("Successfully compiled {} to {}", input_file, output_file);
            } else {
                println!("{}", result.code);
            }
        }
        Err(e) => {
            eprintln!("Compilation error: {:?}", e);
            std::process::exit(1);
        }
    }
}

fn print_usage() {
    println!("JRust Translator");
    println!("Usage: jrust-translator <input.js> [-o <output.rs>]");
    println!();
    println!("Options:");
    println!("  <input.js>    Input JavaScript file");
    println!("  -o <output.rs> Output Rust file (optional, prints to stdout if not specified)");
    println!();
    println!("Limits:");
    println!("  Max file size: {} bytes", jrust_translator::ast::MAX_FILE_SIZE);
}
