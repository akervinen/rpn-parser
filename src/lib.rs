#[macro_use]
extern crate lazy_static;

use std::collections::HashMap;
use std::f64::consts;

enum Token {
    Number(f64),
    Identifier(String),
}

#[derive(Debug)]
enum Operator {
    Constant(f64),
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
fn op_powf(v1: f64, v2: f64) -> f64 {
    v1.powf(v2)
}
fn op_sin(v1: f64) -> f64 {
    v1.sin()
}
fn op_ln(v1: f64) -> f64 {
    v1.ln()
}
fn op_log10(v1: f64) -> f64 {
    v1.log10()
}

lazy_static! {
    // Define operators
    static ref OPERATORS: HashMap<String, Operator> = {
        use Operator::*;

        let mut m = HashMap::new();
        m.insert("+".into(), Binary(op_add));
        m.insert("-".into(), Binary(op_sub));
        m.insert("−".into(), Binary(op_sub));
        m.insert("×".into(), Binary(op_mul));
        m.insert("x".into(), Binary(op_mul));
        m.insert("*".into(), Binary(op_mul));
        m.insert("/".into(), Binary(op_div));
        m.insert("%".into(), Binary(op_mod));
        m.insert("^".into(), Binary(op_powf));

        m.insert("sin".into(),  Unary(op_sin));
        m.insert("ln".into(),   Unary(op_ln));
        m.insert("log10".into(),Unary(op_log10));

        m.insert("pi".into(),   Constant(consts::PI));
        m.insert("e".into(),    Constant(consts::E));
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

fn get_operator(op: &str) -> Result<&Operator, String> {
    OPERATORS.get(op)
        .ok_or(format!("invalid operator '{}'", op))
}
// Recursive execution, starting from the last token and going backwards
// Still seems kinda messy
fn exec_index<I>(tokens: &mut I) -> Result<f64, String>
    where I: Iterator<Item = Token> {
    use Token::*;
    use Operator::*;

    let next = tokens.next();
    let tok = match next {
        Some(val) => val,
        None => return Err("not enough operands".into())
    };

    // Bail out early if we have a plain number
    let op = match tok {
        Number(num) => return Ok(num),
        Identifier(ref op) => {
            try!(get_operator(op))
        }
    };

    return match op {
        &Constant(val) => Ok(val),
        &Unary(cb) => {
            Ok(cb(try!(exec_index(tokens))))
        },
        &Binary(cb) => {
            let val2 = try!(exec_index(tokens));
            let val1 = try!(exec_index(tokens));
            Ok(cb(val1, val2))
        }
    }
}

fn execute(tokens: Vec<Token>) -> Result<f64, String> {
    exec_index(&mut tokens.into_iter().rev())
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

    // I'm not sure if this should be an error
    //#[test]
    //fn eval_err_too_many_operands() {
    //    assert!(evaluate("1 2").is_err());
    //    assert!(evaluate("1 2 3 +").is_err());
    //}

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
