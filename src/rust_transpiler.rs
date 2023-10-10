use crate::ast::*;

pub enum RustTranspilerError {
    ExpectedBooleanExpression,
    ExpectedExpression,
    ExpectedFactor,
    ExpectedTerm,
}

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

fn bool_comp(bool: Box<BooleanExpression>) -> String {
    match *bool {
        BooleanExpression::And { expr1, expr2 } => {
            format!("({} && {})", expr_comp(expr1), expr_comp(expr2))
        },
        BooleanExpression::Or { expr1, expr2 } => {
            format!("({} || {})", expr_comp(expr1), expr_comp(expr2))
        },
        BooleanExpression::Not { expr } => {
            format!("(!{})", expr_comp(expr))
        },
        BooleanExpression::Equal { expr1, expr2 } => {
            format!("({} == {})", expr_comp(&expr1), expr_comp(&expr2))
        },
        BooleanExpression::NotEquals { expr1, expr2 } => {
            format!("({} != {})", expr_comp(&expr1), expr_comp(&expr2))
        },
        BooleanExpression::LessThan { expr1, expr2 } => {
            format!("({} < {})", expr_comp(&expr1), expr_comp(&expr2))
        },
        BooleanExpression::LessThanOrEqual { expr1, expr2 } => {
            format!("({} <= {})", expr_comp(&expr1), expr_comp(&expr2))
        },
        BooleanExpression::GreaterThan { expr1, expr2 } => {
            format!("({} > {})", expr_comp(&expr1), expr_comp(&expr2))
        },
        BooleanExpression::GreaterThanOrEqual { expr1, expr2 } => {
            format!("({} >= {})", expr_comp(&expr1), expr_comp(&expr2))
        },
        BooleanExpression::True => "true".to_string(),
        BooleanExpression::False => "false".to_string(),
    }
}
