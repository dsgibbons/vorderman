use std::fmt;

const BASE: u32 = 10;

type Expression = Vec<Token>;

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

#[derive(Debug, PartialEq)]
enum ParseError {
    InvalidCharacter(usize, char),
}

fn parse(input: &str) -> Result<Expression, ParseError> {
    let mut result = Expression::new();
    let mut num_buffer = Vec::<char>::new();
    for (i, c) in input.trim().char_indices() {
        if c.is_digit(BASE) {
            num_buffer.push(c);
            continue;
        } else if num_buffer.len() > 0 {
            result.push(Token::Number(
                num_buffer.iter().collect::<String>().parse().unwrap(),
            ));
            num_buffer = Vec::<char>::new();
        }

        let token: Result<Option<Token>, ParseError> = match c {
            ' ' => Ok(None),
            '(' => Ok(Some(Token::Parenthesis(Parenthesis::Open))),
            ')' => Ok(Some(Token::Parenthesis(Parenthesis::Close))),
            '+' => Ok(Some(Token::Operation(Operation::Add))),
            '-' => Ok(Some(Token::Operation(Operation::Subtract))),
            '*' => Ok(Some(Token::Operation(Operation::Multiply))),
            '/' => Ok(Some(Token::Operation(Operation::Divide))),
            _ => Err(ParseError::InvalidCharacter(i, c)),
        };

        match token {
            Ok(Some(t)) => result.push(t),
            Ok(None) => continue,
            _ => return Err(ParseError::InvalidCharacter(i, c)),
        }
    }

    if num_buffer.len() > 0 {
        result.push(Token::Number(
            num_buffer.iter().collect::<String>().parse().unwrap(),
        ));
    }
    Ok(result)
}

#[cfg(test)]
mod tests {
    use super::*;

    macro_rules! expr_from_str_tests {
        ($($name:ident: $value:expr,)*) => {
        $(
            #[test]
            fn $name() {
                let (input, expected) = $value;
                assert_eq!(parse(input).unwrap(), expected);
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
    fn bad_parse_char() {
        let input = "(1+ 2/ 3** a 51 x)";
        assert_eq!(parse(input), Err(ParseError::InvalidCharacter(11, 'a')))
    }
}
