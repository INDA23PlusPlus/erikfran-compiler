use std::{ fmt::{self, Debug, Display}, fs };

#[derive(Debug, PartialEq, Clone)]
pub enum TokenType {
    Variable(String),
    Int(i32),
    Plus,
    Minus,
    Multiply,
    Divide,
    Modulo,
    Pow,
    LParen,
    RParen,
    LBrace,
    RBrace,
    Semicolon,
    Equal,
    DoubleEqual,
    GreaterThan,
    LessThan,
    Not,
    And,
    Or,
    TrueKeyword,
    FalseKeyword,
    IfKeyword,
    ElseKeyword,
    WhileKeyword,
    LetKeyword,
    PrintKeyword,
}

#[derive(PartialEq, Clone)]
pub struct Token {
    pub token_type: TokenType,
    line: usize,
    column: usize,
}

impl Display for Token {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:?}", self.token_type)
    }
}

impl Debug for Token {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:?} at line {} column {}", self.token_type, self.line, self.column)
    }
}

pub fn tokenize(path: &str) -> Vec<Token> {
    let mut tokens = Vec::new();
    let mut string = String::new();

    let mut line = 0;
    let mut column = 0;

    for c in fs::read_to_string(path).unwrap().chars() {
        let token_type = match c {
            '+' => Some(TokenType::Plus),
            '-' => Some(TokenType::Minus),
            '*' => Some(TokenType::Multiply),
            '/' => Some(TokenType::Divide),
            '%' => Some(TokenType::Modulo),
            '^' => Some(TokenType::Pow),
            '(' => Some(TokenType::LParen),
            ')' => Some(TokenType::RParen),
            '{' => Some(TokenType::LBrace),
            '}' => Some(TokenType::RBrace),
            ';' => Some(TokenType::Semicolon),
            '=' => Some(TokenType::Equal),
            '>' => Some(TokenType::GreaterThan),
            '<' => Some(TokenType::LessThan),
            '!' => Some(TokenType::Not),
            ' ' | '\r' => None,
            'a'..='z' | 'A'..='Z' | '_' | '0'..='9' => {
                string.push(c);
                continue;
            },
            '\n' => {
                line += 1;
                column = 0;
                continue;
            },
            _ => {
                println!("Unexpected character: {}", c);
                continue;
            },
        };

        if !string.is_empty() {
            if string.chars().all(|c| c.is_numeric()) {
                tokens.push(Token {
                    token_type: TokenType::Int(string.parse().unwrap()),
                    line,
                    column,
                });
            } else {
                let token_type = match string.as_str() {
                    "if" => TokenType::IfKeyword,
                    "else" => TokenType::ElseKeyword,
                    "while" => TokenType::WhileKeyword,
                    "let" => TokenType::LetKeyword,
                    "print" => TokenType::PrintKeyword,
                    "true" => TokenType::TrueKeyword,
                    "false" => TokenType::FalseKeyword,
                    "==" => TokenType::DoubleEqual,
                    "&&" => TokenType::And,
                    "||" => TokenType::Or,
                    _ => TokenType::Variable(string),
                };

                tokens.push(Token {
                    token_type: token_type,
                    line,
                    column,
                });
            }

            string = String::new();
        }

        if let Some(token_type) = token_type {
            tokens.push(Token {
                token_type,
                line,
                column,
            });
        }

        column += 1;
    }

    return tokens;
}