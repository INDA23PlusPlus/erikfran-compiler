use crate::ast::*;

use std::fmt::{self, Display};

pub fn transpile(ast: Ast) -> String {
    let mut rust_code = String::new();

    rust_code.push_str("fn main() {\n");

    rust_code.push_str(&block_comp(&*ast.ast));

    rust_code.push_str("}\n");

    rust_code
}

fn block_comp(block: &Vec<Statement>) -> String {
    let mut out = String::new();

    for statement in block {
        out.push_str(&match statement {
            Statement::Print { expr } => {
                format!("    println!(\"{{}}\", {});\n", expr_comp(expr))
            },
            Statement::Let { variable, expr } => {
                format!("    let mut {} = {};\n", variable, expr_comp(expr))
            },
            Statement::Assignment { variable, expr } => {
                format!("    {} = {};\n", variable, expr_comp(expr))
            },
            Statement::If { condition, body, else_body } => {
                format!("    if {} {{\n{}}}\n", bool_comp(condition), block_comp(body)) +
                if let Some(else_body) = else_body {
                    format!("    else {{\n{}}}\n", block_comp(else_body))
                } else {
                    "".to_string()
                }.as_str()
            }
            Statement::While { condition, body } => {
                format!("    while {} {{\n{}}}\n", bool_comp(condition), block_comp(body))
            },
        });
    }

    out
}

fn expr_comp(expr: &Expression) -> String {
    match expr {
        Expression::Term(term) => term_comp(term.clone()),
        Expression::Plus { expr, term } => {
            format!("{} + {}", expr_comp(expr), term_comp(term.clone()))
        },
        Expression::Minus { expr, term } => {
            format!("{} - {}", expr_comp(expr), term_comp(term.clone()))
        },
    }
}

fn term_comp(term: Box<Term>) -> String {
    match *term {
        Term::Factor(factor) => factor_comp(factor),
        Term::Divide { term, factor } => {
            format!("{} / {}", term_comp(term), factor_comp(factor))
        },
        Term::Multiply { term, factor } => {
            format!("{} * {}", term_comp(term), factor_comp(factor))
        },
    }
}

fn factor_comp(factor: Box<Factor>) -> String {
    match *factor {
        Factor::Int(value) => value.to_string(),
        Factor::Variable(name) => name,
        Factor::Parenthesis(expr) => format!("({})", expr_comp(&*expr)),
    }
}

impl Display for BooleanOp {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            BooleanOp::And => write!(f, "&&"),
            BooleanOp::Or => write!(f, "||"),
            BooleanOp::Xor => write!(f, "^"),
        }
    }
}

impl Display for BooleanCompOp {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            BooleanCompOp::Equal => write!(f, "=="),
            BooleanCompOp::NotEqual => write!(f, "!="),
            BooleanCompOp::GreaterThan => write!(f, ">"),
            BooleanCompOp::LessThan => write!(f, "<"),
            BooleanCompOp::GreaterThanOrEqual => write!(f, ">="),
            BooleanCompOp::LessThanOrEqual => write!(f, "<="),
        }
    }
}

fn bool_comp(bool: &BooleanExpression) -> String {
    match bool {
        BooleanExpression::BooleanOp { op, expr1, expr2 } => {
            format!("{} {} {}", bool_comp(&*expr1), op, bool_comp(&*expr2))
        },
        BooleanExpression::Compare { op, expr1, expr2 } => {
            format!("{} {} {}", expr_comp(&expr1), op, expr_comp(&expr2))
        },
        BooleanExpression::Not { expr } => {
            format!("!{}", bool_comp(&*expr))
        },
        BooleanExpression::Boolean(bool) => bool.to_string(),
        BooleanExpression::Parenthesis(expr) => {
            format!("{}", bool_comp(&*expr))
        },
    }
}
