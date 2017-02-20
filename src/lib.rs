use std::f64::consts;

#[derive(Debug)]
pub enum Token {
    Operand(f64),
    Operator(String),
}

fn parse(expr: &str) -> Result<Vec<Token>, String> {
    Ok(vec![Token::Operand(0.0)])
}

pub fn execute(tokens: Vec<Token>) -> Result<f64, String> {
    use Token::*;
    
    let mut stack = Vec::<f64>::new();

    for token in tokens {
        match token {
            Operand(val) => {
                println!("push: {}", val);
                stack.push(val);
            },
            Operator(ref op) if op == "+" => {
                let val2 = stack.pop().expect("not enough operands");
                let val1 = stack.pop().expect("not enough operands");

                println!("{} + {}", val1, val2);
                stack.push(val1 + val2);
            },
            Operator(ref op) if op == "-" => {
                let val2 = stack.pop().expect("not enough operands");
                let val1 = stack.pop().expect("not enough operands");

                println!("{} - {}", val1, val2);
                stack.push(val1 - val2);
            },
            Operator(ref op) if op == "x" => {
                let val2 = stack.pop().expect("not enough operands");
                let val1 = stack.pop().expect("not enough operands");

                println!("{} × {}", val1, val2);
                stack.push(val1 * val2);
            },
            Operator(ref op) if op == "/" => {
                let val2 = stack.pop().expect("not enough operands");
                let val1 = stack.pop().expect("not enough operands");

                println!("{} / {}", val1, val2);
                stack.push(val1 / val2);
            },
            Operator(ref op) if op == "sin" => {
                let val = stack.pop().expect("not enough operands");

                println!("sin {}", val);
                stack.push(val.sin());
            },
            Operator(ref op) if op == "pi" => {
                println!("pi");
                stack.push(consts::PI);
            },
            Operator(ref op) => {
                return Err(format!("unimplemented operator {}", op));
            }
        }
    }

    if stack.len() == 1 {
        Ok(stack.pop().unwrap())
    } else {
        Err("too many operands".into())
    }
}

pub fn evaluate(expr: &str) -> Result<f64, String> {
    println!("input: {:?}", expr);

    let tokens = try!(parse(expr));

    println!("tokens:");
    for token in &tokens {
        println!("{:?}", token);
    }

    Ok(0.0)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn eval_one_positive_operand() {
        assert_eq!(evaluate("1").unwrap(), 1.0);
        assert_eq!(evaluate("543").unwrap(), 543.0);
    }

    #[test]
    fn eval_one_negative_operand() {
        assert_eq!(evaluate("-1").unwrap(), -1.0);
        assert_eq!(evaluate("-543").unwrap(), -543.0);
    }

    #[test]
    fn eval_one_decimal_operand() {
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
        assert_eq!(evaluate("1 2 *").unwrap(), 2.0);
    }

    #[test]
    fn eval_multiply_multiple() {
        assert_eq!(evaluate("1 2 * 3 *").unwrap(), 6.0);
    }

    #[test]
    fn eval_divide() {
        assert_eq!(evaluate("1 2 /").unwrap(), 2.0);
    }

    #[test]
    fn eval_divide_multiple() {
        assert_eq!(evaluate("1 2 * 3 *").unwrap(), 6.0);
    }

    #[test]
    fn eval_mixed_operators() {
        assert_eq!(evaluate("1 2 + 3 * 4 -").unwrap(), 5.0);
    }
}
