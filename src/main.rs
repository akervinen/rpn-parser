extern crate rpn_parser;

fn main() {
    use rpn_parser::Token;
    use rpn_parser::Token::*;

    let tokens: Vec<Token> = vec![
        Number(5.0),
        Number(1.0),
        Number(2.0),
        Identifier("+".into()),
        Number(4.0),
        Identifier("x".into()),
        Identifier("+".into()),
        Number(3.0),
        Identifier("-".into())
    ];

    println!("= {:?}", rpn_parser::execute(tokens).unwrap());

    let tokens: Vec<Token> = vec![
        Identifier("pi".into()),
        Number(2.0),
        Identifier("/".into()),
        Identifier("sin".into())
    ];

    println!("= {:?}", rpn_parser::execute(tokens).unwrap());
}