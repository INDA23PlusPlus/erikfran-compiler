use std::error::Error;
use std::fmt::{self, Display};

use crate::ast::*;

#[derive(Debug)]
pub enum SemanticAnalyzerError {
    UndefinedVariable(String),
}

impl Display for SemanticAnalyzerError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::UndefinedVariable(t) => write!(f, "{:?}: Undefined variable {}", self, t),
        }
    }
}

impl Error for SemanticAnalyzerError {}

pub fn analyze(ast: &Ast) -> Result<(), SemanticAnalyzerError> {
    let variables: Vec<&str> = Vec::new();
    block_analyze(&*ast.ast, &variables)
}

fn block_analyze(block: &Vec<Statement>, variables: &Vec<&str>) -> Result<(), SemanticAnalyzerError> {
    let mut local_variables: Vec<&str> = variables.clone();

    for statement in block {
        match statement {
            Statement::Print { expr } => {
                expr_analyze(expr, &local_variables)?;
            },
            Statement::Let { variable, expr } => {
                expr_analyze(expr, &local_variables)?;
                local_variables.push(variable.as_str());
            },
            Statement::Assignment { variable, expr } => {
                variable_analyze(variable, &local_variables)?;

                expr_analyze(expr, &local_variables)?;
            },
            Statement::If { condition, body, else_body } => {
                bool_analyze(condition, &local_variables)?;
                block_analyze(body, &local_variables)?;
                if let Some(else_body) = else_body {
                    block_analyze(else_body, &local_variables)?;
                }
            }
            Statement::While { condition, body } => {
                bool_analyze(condition, &local_variables)?;
                block_analyze(body, &local_variables)?;
            },
        }
    }

    Ok(())
}

fn variable_analyze(variable: &str, variables: &Vec<&str>) -> Result<(), SemanticAnalyzerError> {
    if !variables.contains(&variable) {
        return Err(SemanticAnalyzerError::UndefinedVariable(variable.to_string()));
    }

    Ok(())
}

fn expr_analyze(expr: &Expression, variables: &Vec<&str>) -> Result<(), SemanticAnalyzerError> {
    match expr {
        Expression::Term(term) => term_analyze(term.clone(), variables),
        Expression::Plus { expr, term } => {
            expr_analyze(expr, variables)?;
            term_analyze(term.clone(), variables)
        },
        Expression::Minus { expr, term } => {
            expr_analyze(expr, variables)?;
            term_analyze(term.clone(), variables)
        },
    }
}

fn term_analyze(term: Box<Term>, variables: &Vec<&str>) -> Result<(), SemanticAnalyzerError> {
    match *term {
        Term::Factor(factor) => factor_analyze(factor, variables),
        Term::Divide { term, factor } => {
            term_analyze(term, variables)?;
            factor_analyze(factor, variables)
        },
        Term::Multiply { term, factor } => {
            term_analyze(term, variables)?;
            factor_analyze(factor, variables)
        },
    }
}

fn factor_analyze(factor: Box<Factor>, variables: &Vec<&str>) -> Result<(), SemanticAnalyzerError> {
    match *factor {
        Factor::Int(value) => Ok(()),
        Factor::Variable(name) => variable_analyze(name.as_str(), variables),
        Factor::Parenthesis(expr) => expr_analyze(&*expr, variables),
    }
}

fn bool_analyze(bool: &BooleanExpression, variables: &Vec<&str>) -> Result<(), SemanticAnalyzerError> {
    match bool {
        BooleanExpression::BooleanOp { op, expr1, expr2 } => {
            bool_analyze(&*expr1, &variables)?;
            bool_analyze(&*expr2, &variables)
        },
        BooleanExpression::Compare { op, expr1, expr2 } => {
            expr_analyze(&expr1, variables)?;
            expr_analyze(&expr2, variables)
        },
        BooleanExpression::Not { expr } => {
            bool_analyze(&*expr, variables)
        },
        BooleanExpression::Boolean(bool) => Ok(()),
        BooleanExpression::Parenthesis(expr) => {
            bool_analyze(&*expr, variables)
        },
    }
}
