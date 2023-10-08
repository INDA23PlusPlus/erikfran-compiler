mod lexer;
mod ast;

fn main() {
    let tokens = lexer::tokenize("src/fibonacci.txt");

    for token in tokens {
        println!("{}", token);
    }

    let ast = ast::ast_comp(tokens);
}