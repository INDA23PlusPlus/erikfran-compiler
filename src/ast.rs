pub enum BinaryOpTypes {
    Plus,
    Minus,
    Multiply,
    Divide,
    Modulo,
    Pow,
}

pub enum Expression {
    Int(i32),
    Variable(String),
    BinaryOp{
        op: BinaryOpTypes,
        expr1: Box<Expression>,
        expr2: Box<Expression>,
    },
}

pub enum BooleanExpression {
    Boolean(bool),
    Equal {
        expr1: Box<Expression>,
        expr2: Box<Expression>,
    },
    GreaterThan {
        expr1: Box<Expression>,
        expr2: Box<Expression>,
    },
    LessThan {
        expr1: Box<Expression>,
        expr2: Box<Expression>,
    },
    Not {
        expr: Box<BooleanExpression>,
    },
    And {
        expr1: Box<BooleanExpression>,
        expr2: Box<BooleanExpression>,
    },
    Or {
        expr1: Box<BooleanExpression>,
        expr2: Box<BooleanExpression>,
    },
}

pub enum Statement {
    Assignment {
        variable: String,
        expr: Expression,
    },
    If {
        condition: BooleanExpression,
        body: Vec<Statement>,
        else_body: Option<Vec<Statement>>,
    },
    While {
        condition: BooleanExpression,
        body: Vec<Statement>,
    },
    Print {
        expr: Expression,
    },
    Let {
        variable: String,
        expr: Expression,
    },
}

pub struct Ast {
    pub ast: Vec<Statement>,
}

#[derive(Debug)]
pub enum AstCompError {
    ExpectedVariable(Token),
    ExpectedEqual(Token),
    ExpectedExpression(Token),
    ExpectedSemicolon(Token),
    ExpectedLBrace(Token),
    ExpectedRBrace(Token),
    ExpectedLParen(Token),
    ExpectedRParen(Token),
    ExpectedBooleanExpression(Token),
    ExpectedIfBody(Token),
    ExpectedElseBody(Token),
    ExpectedWhileBody(Token),
    ExpectedStatement(Token),
    ExpectedPrintParenthesis(Token),
    EndOfFileInStatement,
}

impl Error for AstCompError {
    fn description(&self) -> &str {
        match self {
            Self::ExpectedVariable(t) => format!("{:?}: Expected variable found {:?}", self, t).as_str(),
            Self::ExpectedEqual(t) => format!("{:?}: Expected equal found {:?}", self, t).as_str(),
            Self::ExpectedExpression(t) => format!("{:?}: Expected expression found {:?}", self, t).as_str(),
            Self::ExpectedSemicolon(t) => format!("{:?}: Expected semicolon found {:?}", self, t).as_str(),
            Self::ExpectedLBrace(t) => format!("{:?}: Expected left brace because of {:?}", self, t).as_str(),
            Self::ExpectedRBrace(t) => format!("{:?}: Expected right brace in this statement {:?}", self, t).as_str(),
            Self::ExpectedLParen(t) => format!("{:?}: Expected left parenthesis because of {:?}", self, t).as_str(),
            Self::ExpectedRParen(t) => format!("{:?}: Expected right parenthesis found {:?}", self, t).as_str(),
            Self::ExpectedBooleanExpression(t) => format!("{:?}: Expected boolean expression found {:?}", self, t).as_str(),
            Self::ExpectedIfBody(t) => format!("{:?}: Expected if body found {:?}", self, t).as_str(),
            Self::ExpectedElseBody(t) => format!("{:?}: Expected else body found {:?}", self, t).as_str(),
            Self::ExpectedWhileBody(t) => format!("{:?}: Expected while body found {:?}", self, t).as_str(),
            Self::ExpectedStatement(t) => format!("{:?}: Expected statement found {:?}", self, t).as_str(),
            Self::ExpectedPrintParenthesis(t) => format!("{:?}: Expected print parenthesis found {:?}", self, t).as_str(),
            Self::EndOfFileInStatement => format!("{:?}: End of file in statement", self).as_str(),
        }
    }
}

use std::error::Error;

use crate::lexer::{ Token, TokenType };

pub fn ast_comp(tokens: Vec<Token>) -> Result<Ast, AstCompError> {
    let mut ast = Vec::new();

    let token_iter = tokens.iter();

    for token in token_iter {
        match token.token_type {

        }
    }

    Ok(Ast { ast })
}

fn expression(token_iter: &mut std::slice::Iter<Token>) -> Result<Expression, AstCompError> {
    match token_iter.next() {
        Some(Token { token_type: TokenType::Int(i), .. }) => {

        },
        Some(Token { token_type: TokenType::Variable(s), .. }) => {
            
        },
        Some(Token { token_type: TokenType::LParen, .. }) => {
            let expr = expression(token_iter)?;
            match token_iter.next() {
                Some(Token { token_type: TokenType::RParen, .. }) => Ok(expr),
                Some(token) => {

                },
                Err(AstCompError::ExpectedRParen(token.clone())),
                None => Err(AstCompError::EndOfFileInStatement),
            }
        },
        Some(token) => Err(AstCompError::ExpectedExpression(token.clone())),
        None => Err(AstCompError::EndOfFileInStatement),
    }
}

fn binary_op(token_iter: &mut std::slice::Iter<Token>) -> Result<Expression, AstCompError> {
    match token_iter.next() {
        Some(Token { token_type: TokenType::Plus, .. }) => {
            Ok(Expression::BinaryOp { 
                op: BinaryOpTypes::Plus, 
                expr1: expression(token_iter)?, 
                expr2: expression(token_iter)?, 
            })
        },
        Some(Token { token_type: TokenType::Minus, .. }) => {
            Ok(Expression::BinaryOp { 
                op: BinaryOpTypes::Minus, 
                expr1: expression(token_iter)?, 
                expr2: expression(token_iter)?, 
            })
        },
        Some(Token { token_type: TokenType::Multiply, .. }) => {
            Ok(Expression::BinaryOp { 
                op: BinaryOpTypes::Multiply, 
                expr1: expression(token_iter)?, 
                expr2: expression(token_iter)?, 
            })
        },
        Some(Token { token_type: TokenType::Divide, .. }) => {
            Ok(Expression::BinaryOp { 
                op: BinaryOpTypes::Divide, 
                expr1: expression(token_iter)?, 
                expr2: expression(token_iter)?, 
            })
        },
        Some(Token { token_type: TokenType::Modulo, .. }) => {
            Ok(Expression::BinaryOp { 
                op: BinaryOpTypes::Modulo, 
                expr1: expression(token_iter)?, 
                expr2: expression(token_iter)?, 
            })
        },
        Some(Token { token_type: TokenType::Pow, .. }) => {
            Ok(Expression::BinaryOp { 
                op: BinaryOpTypes::Pow, 
                expr1: expression(token_iter)?, 
                expr2: expression(token_iter)?, 
            })
        },
        Some(token) => Err(AstCompError::ExpectedExpression(token.clone())),
        None => Err(AstCompError::EndOfFileInStatement),
    }
}