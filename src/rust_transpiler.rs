use crate::ast::*;

pub fn transpile(ast: Ast) -> String {
    let mut block_iter = ast.ast.iter();

    let mut rust_code = String::new();

    rust_code.push_str("fn main() {\n");

    match block_iter.next() {
        Some(Statement::Print { expr }) => {
            rust_code.push_str(&format!("    println!(\"{{}}\", ({}));\n", expr_comp(expr)));
        },
        _ => {},
    }

    rust_code.push_str("}\n");

    rust_code
}

fn expr_comp(expr: &Expression) -> String {
    match expr {
        Expression::Term(term) => term_comp(term.clone()),
        _ => String::new(),
    }
}

fn term_comp(term: Box<Term>) -> String {
    match *term {
        Term::Factor(factor) => factor_comp(factor),
        _ => String::new(),
    }
}

fn factor_comp(factor: Box<Factor>) -> String {
    match *factor {
        Factor::Int(value) => value.to_string(),
        _ => String::new(),
    }
}