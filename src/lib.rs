#[macro_use]
extern crate lazy_static;

use std::collections::HashMap;
use std::f64::consts;

enum Token {
    Number(f64),
    Identifier(String),
}

enum Operator {
    Unary(fn(f64) -> f64),
    Binary(fn(f64, f64) -> f64),
}

// Operator functions

fn op_add(v1: f64, v2: f64) -> f64 {
    v1 + v2
}
fn op_sub(v1: f64, v2: f64) -> f64 {
    v1 - v2
}
fn op_mul(v1: f64, v2: f64) -> f64 {
    v1 * v2
}
fn op_div(v1: f64, v2: f64) -> f64 {
    v1 / v2
}
fn op_mod(v1: f64, v2: f64) -> f64 {
    v1 % v2
}
fn op_sin(v1: f64) -> f64 {
    v1.sin()
}

lazy_static! {
    // Define constants
    static ref CONSTS: HashMap<String, f64> = {
        let mut m = HashMap::new();
        m.insert("pi".into(), consts::PI);
        m
    };

    // Define operators (and functions)
    static ref OPERATORS: HashMap<String, Operator> = {
        let mut m = HashMap::new();
        m.insert("+".into(), Operator::Binary(op_add));
        m.insert("-".into(), Operator::Binary(op_sub));
        m.insert("−".into(), Operator::Binary(op_sub));
        m.insert("×".into(), Operator::Binary(op_mul));
        m.insert("x".into(), Operator::Binary(op_mul));
        m.insert("*".into(), Operator::Binary(op_mul));
        m.insert("/".into(), Operator::Binary(op_div));
        m.insert("%".into(), Operator::Binary(op_mod));
        m.insert("sin".into(), Operator::Unary(op_sin));
        m
    };
}

fn parse(expr: &str) -> Vec<Token> {
    expr.split_whitespace()
        .map(|part| {
            // All non-numbers are tokenized as identifiers
            match part.parse::<f64>() {
                Ok(num) => Token::Number(num),
                Err(_) => Token::Identifier(part.into()),
            }
        })
        .into_iter()
        .collect()
}

fn execute(tokens: Vec<Token>) -> Result<f64, String> {
    use Token::*;

    let mut stack = Vec::<f64>::new();

    for token in tokens {
        match token {
            Number(val) => {
                stack.push(val);
            }
            Identifier(ref op) => {
                // First, check constants, then functions
                
                match CONSTS.get(op) {
                    Some(&val) => {
                        stack.push(val);
                    }
                    None => {
                        match OPERATORS.get(op) {
                            Some(&Operator::Binary(cb)) => {
                                if stack.len() < 2 {
                                    return Err("not enough operands, expected 2".into());
                                }
                                let val2 = stack.pop().unwrap();
                                let val1 = stack.pop().unwrap();

                                stack.push(cb(val1, val2));
                            }
                            Some(&Operator::Unary(cb)) => {
                                if stack.len() < 1 {
                                    return Err("not enough operands, expected 1".into());
                                }
                                let val1 = stack.pop().unwrap();

                                stack.push(cb(val1));
                            }
                            None => {
                                return Err(format!("invalid operator {}", op));
                            }
                        }
                    }
                }
            }
        }
    }

    match stack.len() {
        0 => Err("empty expression".into()),
        1 => Ok(stack.pop().unwrap()),
        _ => Err("too many operands".into())
    }
}

pub fn evaluate(expr: &str) -> Result<f64, String> {
    execute(parse(expr))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn eval_one_positive_number() {
        assert_eq!(evaluate("1").unwrap(), 1.0);
        assert_eq!(evaluate("543").unwrap(), 543.0);
    }

    #[test]
    fn eval_one_negative_number() {
        assert_eq!(evaluate("-1").unwrap(), -1.0);
        assert_eq!(evaluate("-543").unwrap(), -543.0);
    }

    #[test]
    fn eval_one_decimal_number() {
        assert_eq!(evaluate("0.5").unwrap(), 0.5);
        assert_eq!(evaluate("-0.5").unwrap(), -0.5);
    }

    #[test]
    fn eval_err_too_many_operands() {
        assert!(evaluate("1 2").is_err());
        assert!(evaluate("1 2 3 +").is_err());
    }

    #[test]
    fn eval_err_not_enough_operands() {
        assert!(evaluate("1 +").is_err());
    }

    #[test]
    fn eval_err_invalid_operator() {
        assert!(evaluate("1 2 foo").is_err());
    }

    #[test]
    fn eval_add() {
        assert_eq!(evaluate("1 2 +").unwrap(), 3.0);
    }

    #[test]
    fn eval_add_multiple() {
        assert_eq!(evaluate("1 2 + 3 +").unwrap(), 6.0);
    }

    #[test]
    fn eval_subtract() {
        assert_eq!(evaluate("1 2 -").unwrap(), -1.0);
    }

    #[test]
    fn eval_subtract_multiple() {
        assert_eq!(evaluate("1 2 - 3 -").unwrap(), -4.0);
    }

    #[test]
    fn eval_multiply() {
        assert_eq!(evaluate("1 2 x").unwrap(), 2.0);
    }

    #[test]
    fn eval_multiply_multiple() {
        assert_eq!(evaluate("1 2 x 3 x").unwrap(), 6.0);
    }

    #[test]
    fn eval_divide() {
        assert_eq!(evaluate("1 2 /").unwrap(), 0.5);
    }

    #[test]
    fn eval_divide_multiple() {
        assert_eq!(evaluate("1 2 / 2 /").unwrap(), 0.25);
    }

    #[test]
    fn eval_mixed_operators() {
        assert_eq!(evaluate("1 2 + 3 x 4 -").unwrap(), 5.0);
    }

    #[test]
    fn eval_text_operator() {
        assert_eq!(evaluate("pi 2 / sin").unwrap(), 1.0);
    }
}
