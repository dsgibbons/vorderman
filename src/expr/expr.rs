use std::fmt;
use std::vec::Vec;

const BASE: u32 = 10;

#[derive(Debug, PartialEq)]
struct Expression {
    tokens: Vec<Token>,
}

#[derive(Debug, PartialEq)]
enum Token {
    Number(u32),
    Operation(Operation),
    Parenthesis(Parenthesis),
}

#[derive(Debug, PartialEq)]
pub enum Operation {
    Add,
    Subtract,
    Multiply,
    Divide,
}

#[derive(Debug, PartialEq)]
enum Parenthesis {
    Open,
    Close,
}

impl fmt::Display for Expression {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut characters = Vec::new();
        for c in self.tokens.iter() {
            characters.push(c.to_string());
        }

        write!(f, "{}", characters.join(" "))
    }
}

impl fmt::Display for Token {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Token::Number(n) => write!(f, "{}", n),
            Token::Operation(op) => write!(f, "{}", op),
            Token::Parenthesis(p) => write!(f, "{}", p),
        }
    }
}

impl fmt::Display for Operation {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let c = match self {
            Operation::Add => '+',
            Operation::Subtract => '-',
            Operation::Multiply => '*',
            Operation::Divide => '/',
        };

        write!(f, "{}", c)
    }
}

impl fmt::Display for Parenthesis {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let c = match self {
            Parenthesis::Open => '(',
            Parenthesis::Close => ')',
        };

        write!(f, "{}", c)
    }
}

#[derive(Debug, PartialEq)]
enum LexError {
    InvalidCharacter(usize, char),
}

fn lex(input: &str) -> Result<Expression, LexError> {
    let mut tokens = Vec::<Token>::new();
    let mut num_buffer = Vec::<char>::new();
    for (i, c) in input.trim().char_indices() {
        if c.is_digit(BASE) {
            num_buffer.push(c);
            continue;
        } else if num_buffer.len() > 0 {
            tokens.push(Token::Number(
                num_buffer.iter().collect::<String>().parse().unwrap(),
            ));
            num_buffer = Vec::<char>::new();
        }

        let token: Option<Token> = match c {
            ' ' => None,
            '(' => Some(Token::Parenthesis(Parenthesis::Open)),
            ')' => Some(Token::Parenthesis(Parenthesis::Close)),
            '+' => Some(Token::Operation(Operation::Add)),
            '-' => Some(Token::Operation(Operation::Subtract)),
            '*' => Some(Token::Operation(Operation::Multiply)),
            '/' => Some(Token::Operation(Operation::Divide)),
            _ => return Err(LexError::InvalidCharacter(i, c)),
        };

        match token {
            Some(t) => tokens.push(t),
            None => continue,
        }
    }

    if num_buffer.len() > 0 {
        tokens.push(Token::Number(
            num_buffer.iter().collect::<String>().parse().unwrap(),
        ));
    }
    Ok(Expression { tokens })
}

#[cfg(test)]
mod tests {
    use super::*;

    macro_rules! expr_from_str_tests {
        ($($name:ident: $value:expr,)*) => {
        $(
            #[test]
            fn $name() {
                let (input, tokens) = $value;
                assert_eq!(lex(input).unwrap(), Expression { tokens });
            }
        )*
        }
    }

    expr_from_str_tests! {
        expr_from_str_0: ("1+2", vec![Token::Number(1), Token::Operation(Operation::Add), Token::Number(2)]),
        expr_from_str_1: ("12 + 34", vec![Token::Number(12), Token::Operation(Operation::Add), Token::Number(34)]),
        expr_from_str_2: ("1  *(2 -3) ", vec![Token::Number(1), Token::Operation(Operation::Multiply), Token::Parenthesis(Parenthesis::Open), Token::Number(2), Token::Operation(Operation::Subtract), Token::Number(3), Token::Parenthesis(Parenthesis::Close)]),
        expr_from_str_3: ("12 *(345/ 6789)  ", vec![Token::Number(12), Token::Operation(Operation::Multiply), Token::Parenthesis(Parenthesis::Open), Token::Number(345), Token::Operation(Operation::Divide), Token::Number(6789), Token::Parenthesis(Parenthesis::Close)]),
        expr_from_str_4: ("1 23  345 +  + 6789", vec![Token::Number(1), Token::Number(23), Token::Number(345), Token::Operation(Operation::Add), Token::Operation(Operation::Add), Token::Number(6789)]),
    }

    #[test]
    fn bad_lex_char() {
        let input = "(1+ 2/ 3** a 51 x)";
        assert_eq!(lex(input), Err(LexError::InvalidCharacter(11, 'a')))
    }

    macro_rules! expr_to_str_tests {
        ($($name:ident: $value:expr,)*) => {
        $(
            #[test]
            fn $name() {
                let (input, expected) = $value;
                let as_expr = lex(input).unwrap().to_string();
                assert_eq!(as_expr, expected.to_string());
            }
        )*
        }
    }

    expr_to_str_tests! {
        expr_to_str_0: ("1+2", "1 + 2"),
        expr_to_str_1: ("12 + 34", "12 + 34"),
        expr_to_str_2: ("1  *(2 -3) ", "1 * ( 2 - 3 )"),
        expr_to_str_3: ("12 *(345/ 6789)  ", "12 * ( 345 / 6789 )"),
        expr_to_str_4: ("1 23  345 +  + 6789", "1 23 345 + + 6789"),
    }
}
