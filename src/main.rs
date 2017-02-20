extern crate rpn_parser;

fn main() {
    use rpn_parser::Token;

    let tokens: Vec<rpn_parser::Token> = vec![
        Token::Operand(11.0),
        Token::Operand(22.0),
        Token::Operator("+".into())
    ];

    println!("{:?}", rpn_parser::execute(tokens).unwrap());
}