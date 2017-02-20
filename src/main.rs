extern crate rpn_parser;

use std::env;

fn main() {
    let args: Vec<String> = env::args().skip(1).collect();

    if args.len() == 0 {
        println!("usage: rpn-parser <expression>");
        return;
    }

    let line = args.join(" ");
    println!("{}", rpn_parser::evaluate(&line).unwrap());
}