extern crate rpn_parser;

fn main() {
    println!("{:?}", rpn_parser::evaluate("5 1 2 + 4 × + 3 −").unwrap());
}