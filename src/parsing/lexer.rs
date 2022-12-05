use super::Operator;

#[derive(Debug)]
pub enum Token {
    Operator(Operator),
    Identifier(String),
    Number(i64),
    OpenBrace,
    CloseBrace,
    OpenParen,
    CloseParen,
    Quote
}

pub fn lex(str: &String) -> Vec<Token> {
    let iter: Vec<char> = str.chars().collect();
    let mut tokens = Vec::new();
    let mut i = 0;
    while let Some(next_char) = iter.get(i) {
        i += 1;
        match next_char {
            '(' => tokens.push(Token::OpenParen),
            ')' => tokens.push(Token::CloseParen),
            '{' => tokens.push(Token::OpenBrace),
            '}' => tokens.push(Token::CloseBrace),
            '"' => tokens.push(Token::Quote),
            ' ' => continue,
            _ => {
                if next_char.is_ascii_digit() {
                    let mut number = String::new();
                    number.push(*next_char);
                    while let Some(next_char) = iter.get(i) {
                        if next_char.is_ascii_digit() {
                            number.push(*next_char);
                            i += 1;
                        } else {
                            break;
                        }
                    }
                    tokens.push(Token::Number(number.parse().unwrap()));
                } else if next_char.is_ascii_alphabetic() {
                    let mut identifier = String::new();
                    identifier.push(*next_char);
                    while let Some(next_char) = iter.get(i) {
                        if next_char.is_ascii_alphanumeric() {
                            identifier.push(*next_char);
                            i += 1;
                        } else {
                            break;
                        }
                    }
                    tokens.push(Token::Identifier(identifier));
                } else {
                    if let Some(operator) = Operator::from_char(next_char) {
                        tokens.push(Token::Operator(operator));
                    }
                }
            }
        }
    }
    tokens
}