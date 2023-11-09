use std::{fs, process::{self, Stdio}, env::args};

mod lexer;
mod ast;
mod semantic_analyzer;
mod rust_transpiler;
//mod emulator_compiler;

fn main() {
    let path = args().nth(2).expect("No file path provided");
    let string = fs::read_to_string(&path).unwrap();

    println!("Source code:\n\n{}\n", string);

    let tokens = lexer::tokenize(&string);

    println!("Tokens:\n");
    for token in &tokens {
        match token.token_type {
            lexer::TokenType::Semicolon => println!("{} ", token),
            lexer::TokenType::LBrace => println!("{} ", token),
            _ => print!("{} ", token),
        }
    }

    println!("");
    let ast = match ast::ast_comp(tokens) {
        Ok(ast) => {
            println!("AST parse successful");
            ast
        },
        Err(e) => {
            println!("{}", e);
            return;
        },
    };

    match semantic_analyzer::analyze(&ast) {
        Ok(_) => println!("Semantic analysis successful"),
        Err(e) => {
            println!("{}", e);
            return;
        },
    };

    let target = args().nth(1).expect("No target provided");

    if target == "rust" {
        let rust_string = rust_transpiler::transpile(ast);
        println!("Rust code:\n\n{}\n", rust_string);

        let rs_path = path.replace(".txt", ".rs").replace("src/", "examples/");
        println!("Rust code file path: {}", rs_path);
        fs::write(&rs_path, rust_string).unwrap();
    
        println!("Compiling Rust code:\n");
        process::Command::new("rustc")
            .arg(&rs_path)
            .stderr(Stdio::inherit())
            .stdout(Stdio::inherit())
            .output()
            .unwrap();
    
        let exe_path = rs_path.replace(".rs", ".exe").replace("examples/", "target/debug/examples/");
        println!("Exe file path: {}", exe_path);
        println!("Running Rust code:\n");
        process::Command::new(".\\".to_string() + exe_path.as_str())
            .stderr(Stdio::inherit())
            .stdout(Stdio::inherit())
            .output()
            .unwrap();
    }
    else if target == "emulator" {
        todo!("Emulator not implemented");
        /* let emulator_string = emulator_compiler::compile(ast);
        println!("Emulator assembly code:\n\n{}\n", emulator_string);

        let asm_path = path.replace(".txt", ".asm");
        println!("Emulator assembly code file path: {}", asm_path);
        fs::write(&asm_path, emulator_string).unwrap();

        println!("Assembling emulator assembly code\n");
        assembler::run(&asm_path);

        let bin_path = path.replace(".txt", ".bin");
        println!("Emulator binary file path: {}", bin_path);

        println!("Running emulator binary:\n");
        emulator::run(&bin_path); */
    }
    else {
        println!("Unknown target: {}", target);
        return;
    }
}