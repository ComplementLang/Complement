extern crate core;

mod parsing;
mod equation;

fn main() {
    let args: Vec<String> = std::env::args().collect();
    let file = std::fs::read_to_string(&args[1]).expect("File not found");
    let tokens = parsing::lex(&file);
    println!("{:?}", tokens);
    let parsed = parsing::parse(&tokens);
    println!("{:?}", parsed);
    let solved = equation::solve_equation(&parsed.unwrap());
    println!("{:?}", solved);
}
