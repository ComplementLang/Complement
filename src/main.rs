extern crate core;

mod parsing;

fn main() {
    let args: Vec<String> = std::env::args().collect();
    let file = std::fs::read_to_string(&args[1]).expect("File not found");
    let tokens = parsing::lexer::lex(&file);
    println!("{:?}", tokens);
    parsing::parser::parse(&tokens);
}
