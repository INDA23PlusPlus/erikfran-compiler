mod lexer;

fn main() {
    let tokens = lexer::tokenize("src/fibonacci.txt");

    for token in tokens {
        println!("{}", token);
    }
}