extern crate rpn_parser;

fn main() {
    use rpn_parser::Token;
    use rpn_parser::Token::*;

    let tokens: Vec<Token> = vec![
        Operand(5.0),
        Operand(1.0),
        Operand(2.0),
        Operator("+".into()),
        Operand(4.0),
        Operator("x".into()),
        Operator("+".into()),
        Operand(3.0),
        Operator("-".into())
    ];

    println!("= {:?}", rpn_parser::execute(tokens).unwrap());

    let tokens: Vec<Token> = vec![
        Operator("pi".into()),
        Operand(2.0),
        Operator("/".into()),
        Operator("sin".into())
    ];

    println!("= {:?}", rpn_parser::execute(tokens).unwrap());
}