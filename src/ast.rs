#[derive(Clone)]
pub enum Expression {
    Plus {
        expr: Box<Expression>,
        term: Box<Term>,
    },
    Minus {
        expr: Box<Expression>,
        term: Box<Term>,
    },
    Term(Box<Term>),
}

#[derive(Clone)]
pub enum Term {
    Multiply {
        term: Box<Term>,
        factor: Box<Factor>,
    },
    Divide {
        term: Box<Term>,
        factor: Box<Factor>,
    },
    Factor(Box<Factor>),
}

#[derive(Clone)]
pub enum Factor {
    Int(i32),
    Variable(String),
    Parenthesis(Box<Expression>),
}

pub enum BooleanCompOp {
    Equal,
    NotEqual,
    LessThan,
    LessThanOrEqual,
    GreaterThan,
    GreaterThanOrEqual,
}

pub enum BooleanOp {
    And,
    Or,
    Xor,
}

pub enum BooleanExpression {
    Boolean(bool),
    Compare {
        op: BooleanCompOp,
        expr1: Expression,
        expr2: Expression,
    },
    Not {
        expr: Box<BooleanExpression>,
    },
    BooleanOp {
        op: BooleanOp,
        expr1: Box<BooleanExpression>,
        expr2: Box<BooleanExpression>,
    },
    Parenthesis(Box<BooleanExpression>),
}

pub enum Statement {
    Assignment {
        variable: String,
        expr: Expression,
    },
    If {
        condition: BooleanExpression,
        body: Box<Vec<Statement>>,
        else_body: Option<Box<Vec<Statement>>>,
    },
    While {
        condition: BooleanExpression,
        body: Box<Vec<Statement>>,
    },
    Print {
        expr: Expression,
    },
    Let {
        variable: String,
        expr: Expression,
    },
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

pub struct Ast {
    pub ast: Box<Vec<Statement>>,
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
    EndOfFileInBlock,
}

impl Display for AstCompError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::ExpectedVariable(t) => write!(f, "{:?}: Expected variable found {:?}", self, t),
            Self::ExpectedEqual(t) => write!(f, "{:?}: Expected equal found {:?}", self, t),
            Self::ExpectedExpression(t) => write!(f, "{:?}: Expected expression found {:?}", self, t),
            Self::ExpectedSemicolon(t) => write!(f, "{:?}: Expected semicolon found {:?}", self, t),
            Self::ExpectedLBrace(t) => write!(f, "{:?}: Expected left brace because of {:?}", self, t),
            Self::ExpectedRBrace(t) => write!(f, "{:?}: Expected right brace in this statement {:?}", self, t),
            Self::ExpectedLParen(t) => write!(f, "{:?}: Expected left parenthesis because of {:?}", self, t),
            Self::ExpectedRParen(t) => write!(f, "{:?}: Expected right parenthesis found {:?}", self, t),
            Self::ExpectedBooleanExpression(t) => write!(f, "{:?}: Expected boolean expression found {:?}", self, t),
            Self::ExpectedIfBody(t) => write!(f, "{:?}: Expected if body found {:?}", self, t),
            Self::ExpectedElseBody(t) => write!(f, "{:?}: Expected else body found {:?}", self, t),
            Self::ExpectedWhileBody(t) => write!(f, "{:?}: Expected while body found {:?}", self, t),
            Self::ExpectedStatement(t) => write!(f, "{:?}: Expected statement found {:?}", self, t),
            Self::ExpectedPrintParenthesis(t) => write!(f, "{:?}: Expected print parenthesis found {:?}", self, t),
            Self::EndOfFileInStatement => write!(f, "{:?}: End of file in statement", self),
            Self::EndOfFileInBlock => write!(f, "{:?}: End of file in block", self),
        }
    }
}

impl Error for AstCompError {}

use std::error::Error;
use std::fmt::{self, Display};
use std::slice::Iter;
use std::iter::Peekable;

use crate::lexer::{ Token, TokenType };

pub fn ast_comp(tokens: Vec<Token>) -> Result<Ast, AstCompError> {
    Ok(Ast { ast: block(&mut tokens.iter().peekable())? } )
}

fn expression(token_iter: &mut Peekable<Iter<Token>>) -> Result<Expression, AstCompError> {
    let term = term(token_iter)?;

    match token_iter.peek() {
        Some(Token { token_type: TokenType::Plus, ..}) => {
            token_iter.next();

            Ok(Expression::Plus { 
                expr: Box::new(expression(token_iter)?), 
                term: Box::new(term), 
            })
        },
        Some(Token { token_type: TokenType::Minus, ..}) => {
            token_iter.next();

            Ok(Expression::Minus { 
                expr: Box::new(expression(token_iter)?), 
                term: Box::new(term), 
            })
        },
        _ => Ok(Expression::Term(Box::new(term))),
    }
}

fn term(token_iter: &mut Peekable<Iter<Token>>) -> Result<Term, AstCompError> {
    let factor = factor(token_iter)?;

    match token_iter.peek() {
        Some(Token { token_type: TokenType::Multiply, ..}) => {
            token_iter.next();

            Ok(Term::Multiply { 
                term: Box::new(term(token_iter)?), 
                factor: Box::new(factor), 
            })
        },
        Some(Token { token_type: TokenType::Divide, ..}) => {
            token_iter.next();

            Ok(Term::Divide { 
                term: Box::new(term(token_iter)?), 
                factor: Box::new(factor), 
            })
        },
        _ => Ok(Term::Factor(Box::new(factor))),
    }
}

fn factor(token_iter: &mut Peekable<Iter<Token>>) -> Result<Factor, AstCompError> {
    match token_iter.next() {
        Some(Token { token_type: TokenType::Int(i), .. }) => Ok(Factor::Int(*i)),
        Some(Token { token_type: TokenType::Variable(s), .. }) => Ok(Factor::Variable(s.clone())),
        Some(Token { token_type: TokenType::LParen, .. }) => {
            let expr = expression(token_iter)?;

            match token_iter.next() {
                Some(Token { token_type: TokenType::RParen, .. }) => Ok(Factor::Parenthesis(Box::new(expr))),
                Some(token) => Err(AstCompError::ExpectedRParen(token.clone())),
                None => Err(AstCompError::EndOfFileInStatement),
            }
        },
        Some(token) => Err(AstCompError::ExpectedExpression(token.clone())),
        None => Err(AstCompError::EndOfFileInStatement),
    }
}

impl TryFrom<Token> for BooleanCompOp {
    type Error = AstCompError;

    fn try_from(value: Token) -> Result<Self, AstCompError> {
        match value {
            Token { token_type: TokenType::Equal, .. } => Ok(Self::Equal),
            Token { token_type: TokenType::NotEqual, .. } => Ok(Self::NotEqual),
            Token { token_type: TokenType::GreaterThan, .. } => Ok(Self::GreaterThan),
            Token { token_type: TokenType::LessThan, .. } => Ok(Self::LessThan),
            Token { token_type: TokenType::GreaterThanOrEqual, .. } => Ok(Self::GreaterThanOrEqual),
            Token { token_type: TokenType::LessThanOrEqual, .. } => Ok(Self::LessThanOrEqual),
            _ => Err(AstCompError::ExpectedBooleanExpression(value)),
        }
    }
}

fn boolean_expression(token_iter: &mut Peekable<Iter<Token>>) -> Result<BooleanExpression, AstCompError> {
    match token_iter.peek() {
        Some(Token { token_type: TokenType::TrueKeyword, .. }) => Ok(BooleanExpression::Boolean(true)),
        Some(Token { token_type: TokenType::FalseKeyword, .. }) => Ok(BooleanExpression::Boolean(false)),
        Some(Token { token_type: TokenType::LParen, .. }) => {
            let _ = token_iter.next();

            let expr = boolean_expression(token_iter)?;

            match token_iter.next() {
                Some(Token { token_type: TokenType::RParen, .. }) => Ok(expr),
                Some(token) => Err(AstCompError::ExpectedRParen(token.clone())),
                None => Err(AstCompError::EndOfFileInStatement),
            }
        },
        Some(Token { token_type: TokenType::Not, .. }) => {
            Ok(BooleanExpression::Not { 
                expr: Box::new(boolean_expression(token_iter)?), 
            })
        },
        Some(_) => {
            let expr1 = expression(token_iter)?;
            
            let op = BooleanCompOp::try_from(token_iter.next().unwrap().clone())?;

            Ok(BooleanExpression::Compare { 
                op: op, 
                expr1: expr1, 
                expr2: expression(token_iter)?, 
            })
        }
        None => Err(AstCompError::EndOfFileInStatement),
    }
}

fn block(token_iter: &mut Peekable<Iter<Token>>) -> Result<Box<Vec<Statement>>, AstCompError> {
    let mut statements = Vec::new();
    println!("block");

    match token_iter.next() {
        Some(Token { token_type: TokenType::LBrace, .. }) => (),
        Some(token) => return Err(AstCompError::ExpectedLBrace(token.clone())),
        None => return Err(AstCompError::EndOfFileInBlock),
    }

    loop {
        match token_iter.next() {
            Some(Token { token_type: TokenType::IfKeyword, .. }) => {
                statements.push(Statement::If {  
                    condition: boolean_expression(token_iter)?, 
                    body: block(token_iter)?, 
                    else_body: {
                        if let Some(Token { token_type: TokenType::ElseKeyword, .. }) = token_iter.next() {
                            Some(block(token_iter)?)
                        } else {
                            None
                        }
                    }, 
                });
            },
            Some(Token { token_type: TokenType::WhileKeyword, .. }) => {
                statements.push(Statement::While { 
                    condition: boolean_expression(token_iter)?, 
                    body: block(token_iter)?, 
                });
            },
            Some(Token { token_type: TokenType::LetKeyword, .. }) => {
                println!("let");
                let variable = variable(token_iter)?;

                match token_iter.next() {
                    Some(Token { token_type: TokenType::Equal, .. }) => (),
                    Some(token) => return Err(AstCompError::ExpectedEqual(token.clone())),
                    None => return Err(AstCompError::EndOfFileInStatement),
                }

                statements.push(Statement::Let { 
                    variable: variable, 
                    expr: expression(token_iter)?, 
                });

                match token_iter.next() {
                    Some(Token { token_type: TokenType::Semicolon, .. }) => (),
                    Some(token) => return Err(AstCompError::ExpectedSemicolon(token.clone())),
                    None => return Err(AstCompError::EndOfFileInStatement),
                }
            },
            Some(Token { token_type: TokenType::PrintKeyword, .. }) => {
                match token_iter.next() {
                    Some(Token { token_type: TokenType::LParen, .. }) => (),
                    Some(token) => return Err(AstCompError::ExpectedLParen(token.clone())),
                    None => return Err(AstCompError::EndOfFileInStatement),
                }

                statements.push(Statement::Print { 
                    expr: expression(token_iter)?, 
                });

                match token_iter.next() {
                    Some(Token { token_type: TokenType::RParen, .. }) => (),
                    Some(token) => return Err(AstCompError::ExpectedRParen(token.clone())),
                    None => return Err(AstCompError::EndOfFileInStatement),
                }

                match token_iter.next() {
                    Some(Token { token_type: TokenType::Semicolon, .. }) => (),
                    Some(token) => return Err(AstCompError::ExpectedSemicolon(token.clone())),
                    None => return Err(AstCompError::EndOfFileInStatement),
                }
            },
            Some(Token { token_type: TokenType::Variable(variable), .. }) => {
                match token_iter.next() {
                    Some(Token { token_type: TokenType::Equal, .. }) => (),
                    Some(token) => return Err(AstCompError::ExpectedEqual(token.clone())),
                    None => return Err(AstCompError::EndOfFileInStatement),
                }

                statements.push(Statement::Assignment { 
                    variable: variable.to_string(), 
                    expr: expression(token_iter)?, 
                });

                match token_iter.next() {
                    Some(Token { token_type: TokenType::Semicolon, .. }) => (),
                    Some(token) => return Err(AstCompError::ExpectedSemicolon(token.clone())),
                    None => return Err(AstCompError::EndOfFileInStatement),
                }
            },
            Some(Token { token_type: TokenType::RBrace, .. }) => break,
            Some(token) => return Err(AstCompError::ExpectedStatement(token.clone())),
            None => break,
        }
    }

    Ok(Box::new(statements))
}

fn variable(token_iter: &mut Peekable<Iter<Token>>) -> Result<String, AstCompError> {
    match token_iter.next() {
        Some(Token { token_type: TokenType::Variable(s), .. }) => Ok(s.clone()),
        Some(token) => Err(AstCompError::ExpectedVariable(token.clone())),
        None => Err(AstCompError::EndOfFileInStatement),
    }
}