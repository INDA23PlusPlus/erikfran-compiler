use std::{fs, process::{self, Stdio}};

mod lexer;
mod ast;
mod rust_transpiler;

fn main() {
    let path = "src/test.txt";
    let string = fs::read_to_string(path).unwrap();

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
        Ok(ast) => ast,
        Err(e) => {
            println!("{}", e);
            return;
        },
    };

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

    let exe_path = rs_path.replace(".rs", ".exe").replace("examples/", "/target/debug/examples/");
    println!("Exe file path: {}", exe_path);
    println!("Running Rust code:\n");
    process::Command::new(".\\".to_string() + exe_path.as_str())
        .stderr(Stdio::inherit())
        .stdout(Stdio::inherit())
        .output()
        .unwrap();
}