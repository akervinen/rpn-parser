pub fn evaluate(expr: &str) -> Result<f64, String> {
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
