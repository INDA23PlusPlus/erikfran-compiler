use std::{hash::Hash, collections::HashMap};

use crate::ast::*;

enum Allocation {
    Register(u8),
    Memory(u8),
}

enum Lifetime {
    Alloc(String),
    Free(String),
}

struct State {
    available_registers: [bool; 16],
    available_memory: [bool; 256],
    variable_map: HashMap<String, Allocation>,
    lifetime_vec: Vec<Lifetime>,
}

pub fn compile(ast: Ast) -> String {
    let mut state = State {
        available_registers: [true; 16],
        available_memory: [true; 256],
        variable_map: HashMap::new(),
        lifetime_vec: Vec::new(),
    };

    block_lifetime(&*ast.ast, &mut state);

    let mut assembly_code = String::new();

    assembly_code.push_str(&block_comp(&*ast.ast));

    assembly_code
}

fn allocate(variable: &str, state: &mut State) {
    for (i, register) in state.available_registers.iter_mut().enumerate() {
        if *register {
            state.available_registers[i] = false;
            state.variable_map.insert(variable.to_string(), Allocation::Register(i as u8));
            return;
        }
    }

    for (i, memory) in state.available_memory.iter_mut().enumerate() {
        if *memory {
            state.available_memory[i] = false;
            state.variable_map.insert(variable.to_string(), Allocation::Memory(i as u8));
            return;
        }
    }

    panic!("Out of memory!");
}

fn register_allocate(variable: &str, state: &mut State) {
    if let Some(allocation) = state.variable_map.get(variable) {
        match allocation {
            Allocation::Register(_) => {},
            Allocation::Memory(address) => {
                for (i, register) in state.available_registers.iter_mut().enumerate() {
                    if *register {
                        state.available_registers[i] = false;
                        state.available_memory[*address as usize] = true;

                        state.variable_map.remove(variable);
                        state.variable_map.insert(variable.to_string(), Allocation::Register(i as u8));
                        return;
                    }
                }

                state.variable_map.remove(variable);
                state.variable_map.insert(variable.to_string(), Allocation::Memory(*address));
            },
        }
    }
    else {
        panic!("Variable {} not found!", variable);
    }
}

fn two_register_allocate(variable1: &str, variable2: &str, state: &mut State) {
    if let  =  {
        
    }
}

fn block_lifetime(block: &Vec<Statement>, state: &mut State) {
    for statement in block {
        match statement {
            Statement::Print { expr } => {
                expr_lifetime(expr, state);
            },
            Statement::Let { variable, expr } => {
                state.lifetime_vec.push(Lifetime::Alloc(variable.clone()));
                allocate(variable.as_str(), state);
                expr_lifetime(expr, state);
            },

        }
    }
}

fn expr_lifetime(expr: &Expression, state: &mut State) {
    match expr {
        Expression::Term(term) => term_lifetime(term, state),
        Expression::Plus { expr, term } => {
            expr_lifetime(expr, state);
            term_lifetime(term, state);
        },
        Expression::Minus { expr, term } => {
            expr_lifetime(expr, state);
            term_lifetime(term, state);
        },
    }
}

fn term_lifetime(term: &Term, state: &mut State) {
    match term {
        Term::Factor(factor) => factor_lifetime(factor, state),
        Term::Divide { term, factor } => {
            term_lifetime(term, state);
            factor_lifetime(factor, state);
        },
        Term::Multiply { term, factor } => {
            term_lifetime(term, state);
            factor_lifetime(factor, state);
        },
    }
}

fn factor_lifetime(factor: &Factor, state: &mut State) {
    match factor {
        Factor::Int(_) => {},
        Factor::Variable(name) => {
            if !state.variable_map.contains_key(name) {
                panic!("Variable {} not found!", name);
            }
        },
        Factor::Parenthesis(expr) => {
            expr_lifetime(expr, state);
        },
    }
}

fn block_comp(block: &Vec<Statement>) -> String {
    let mut out = String::new();

    for statement in block {
        out.push_str(&match statement {
            Statement::Print { expr } => {
                format!("{}setmr {:02X} 0xFF\n", expr_comp(expr), )
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
