use std::fmt;
use std::str::FromStr;
use std::vec::Vec;

const BASE: u32 = 10;

#[derive(Debug, PartialEq)]
pub enum LexError {
    InvalidCharacter(char),
    InvalidCharacterAtIndex(usize, char),
}

#[derive(Debug, PartialEq)]
pub enum Operation {
    Add,
    Subtract,
    Multiply,
    Divide,
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

impl TryFrom<char> for Operation {
    type Error = LexError;

    fn try_from(input: char) -> Result<Operation, Self::Error> {
        match input {
            '+' => Ok(Operation::Add),
            '-' => Ok(Operation::Subtract),
            '*' => Ok(Operation::Multiply),
            '/' => Ok(Operation::Divide),
            c => Err(LexError::InvalidCharacter(c)),
        }
    }
}

#[derive(Debug, PartialEq)]
enum Parenthesis {
    Open,
    Close,
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

impl TryFrom<char> for Parenthesis {
    type Error = LexError;

    fn try_from(input: char) -> Result<Parenthesis, Self::Error> {
        match input {
            '(' => Ok(Parenthesis::Open),
            ')' => Ok(Parenthesis::Close),
            c => Err(LexError::InvalidCharacter(c)),
        }
    }
}

#[derive(Debug, PartialEq)]
pub enum Token {
    Number(u32),
    Operation(Operation),
    Parenthesis(Parenthesis),
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

#[derive(Debug, PartialEq)]
pub struct Expression(pub Vec<Token>);

impl fmt::Display for Expression {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut characters = Vec::new();
        for c in self.0.iter() {
            characters.push(c.to_string());
        }

        write!(f, "{}", characters.join(" "))
    }
}

impl FromStr for Expression {
    type Err = LexError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut tokens = Vec::<Token>::new();
        let mut num_buffer = Vec::<char>::new();
        for (i, c) in s.trim().char_indices() {
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
                '(' | ')' => Some(Token::Parenthesis(Parenthesis::try_from(c).unwrap())),
                '+' | '-' | '*' | '/' => Some(Token::Operation(Operation::try_from(c).unwrap())),
                _ => return Err(LexError::InvalidCharacterAtIndex(i, c)),
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
        Ok(Expression(tokens))
    }
}

#[derive(Debug, PartialEq)]
pub enum Fix {
    Pre,
    In,
    Post,
}

#[derive(Debug, PartialEq)]
pub struct FixExpression {
    pub expression: Expression,
    pub fix: Fix,
}

#[derive(Debug, PartialEq)]
pub enum FixExpressionError {
    InvalidFixExpression,
}

fn validate_prefix(expression: &Expression) -> bool {
    let mut op_count = 0;
    let mut num_count = 0;

    for token in &expression.0 {
        match token {
            Token::Parenthesis(_) => return false,
            Token::Operation(_) => {
                op_count += 1;
            }
            Token::Number(_) => {
                num_count += 1;
                if expression.0.len() == 1 {
                    return true;
                } else if op_count == 0 || num_count > op_count + 1 {
                    return false;
                }
            }
        }
    }
    if op_count != num_count - 1 {
        false
    } else {
        true
    }
}

fn validate_infix(expression: &Expression) -> bool {
    panic!("Not implemented yet")
}

fn validate_postfix(expression: &Expression) -> bool {
    let mut op_count = 0;
    let mut num_count = 0;

    for token in &expression.0 {
        match token {
            Token::Parenthesis(_) => return false,
            Token::Operation(_) => {
                op_count += 1;
                if num_count <= op_count {
                    return false;
                }
            }
            Token::Number(_) => {
                num_count += 1;
            }
        }
    }
    if op_count != num_count - 1 {
        false
    } else {
        true
    }
}

impl FixExpression {
    fn validate(&self) -> bool {
        match self.fix {
            Fix::Pre => validate_prefix(&self.expression),
            Fix::In => validate_infix(&self.expression),
            Fix::Post => validate_postfix(&self.expression),
        }
    }

    pub fn as_prefix(&self) -> Result<&FixExpression, FixExpressionError> {
        if self.fix == Fix::Pre && self.validate() {
            return Ok(self);
        }

        panic!("Not implemented yet");

        Err(FixExpressionError::InvalidFixExpression)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use test_case::test_case;

    #[test_case("1+2", vec![Token::Number(1), Token::Operation(Operation::Add), Token::Number(2)]; "simple addition")]
    #[test_case("12 + 34", vec![Token::Number(12), Token::Operation(Operation::Add), Token::Number(34)]; "double digit addition")]
    #[test_case("1  *(2 -3) ", vec![Token::Number(1), Token::Operation(Operation::Multiply), Token::Parenthesis(Parenthesis::Open), Token::Number(2), Token::Operation(Operation::Subtract), Token::Number(3), Token::Parenthesis(Parenthesis::Close)]; "nested operation with unusual spacing")]
    #[test_case("12 *(345/ 6789)", vec![Token::Number(12), Token::Operation(Operation::Multiply), Token::Parenthesis(Parenthesis::Open), Token::Number(345), Token::Operation(Operation::Divide), Token::Number(6789), Token::Parenthesis(Parenthesis::Close)] ; "another nested operation with unusual spacing")]
    #[test_case("1 23  345 +  + ", vec![Token::Number(1), Token::Number(23), Token::Number(345), Token::Operation(Operation::Add), Token::Operation(Operation::Add)]; "example postfix expression")]
    fn expr_from_str_tests(input: &str, tokens: Vec<Token>) {
        assert_eq!(Expression::from_str(input).unwrap(), Expression(tokens));
    }

    #[test]
    fn bad_lex_char() {
        let input = "(1+ 2/ 3** a 51 x)";
        assert_eq!(
            Expression::from_str(input),
            Err(LexError::InvalidCharacterAtIndex(11, 'a'))
        )
    }

    #[test_case("1+2", "1 + 2"; "simple addition")]
    #[test_case("12 + 34", "12 + 34"  ; "double digit addition")]
    #[test_case("1  *(2 -3) ", "1 * ( 2 - 3 )"  ; "nested operation with unusual spacing")]
    #[test_case("12 *(345/ 6789)  ", "12 * ( 345 / 6789 )"  ; "another nested operation with unusual spacing")]
    #[test_case("1 23  345 + + ", "1 23 345 + +"  ; "example postfix expression")]
    fn expr_to_str_tests(input: &str, expected: &str) {
        assert_eq!(
            Expression::from_str(input).unwrap().to_string(),
            expected.to_string()
        );
    }

    #[test_case("1", true; "single number")]
    #[test_case("+ 1 2", true; "simple addition")]
    #[test_case("+ 12 34", true; "double digit addition")]
    #[test_case("* 1 - 2 3", true; "nested operation")]
    #[test_case("* 12 / 345 6789", true; "another nested operation")]
    #[test_case("1 23 345 + +", false; "postfix expression")]
    #[test_case("1 * 2 - 3", false; "simple infix expression")]
    #[test_case("1 * (2 - 3)", false; "nested infix expression")]
    fn validate_prefix_tests(input: &str, expected: bool) {
        assert_eq!(
            FixExpression {
                expression: Expression::from_str(input).unwrap(),
                fix: Fix::Pre
            }
            .validate(),
            expected,
        );
    }

    #[test_case("1", true; "single number")]
    #[test_case("1 2 +", true; "simple addition")]
    #[test_case("12 34 +", true; "double digit addition")]
    #[test_case("1 2 3 - *", true; "nested operation")]
    #[test_case("12 345 6789 / *", true; "another nested operation")]
    #[test_case("+ + 1 23 345", false; "postfix expression")]
    #[test_case("1 * 2 - 3", false; "simple infix expression")]
    #[test_case("1 * (2 - 3)", false; "nested infix expression")]
    fn validate_postfix_tests(input: &str, expected: bool) {
        assert_eq!(
            FixExpression {
                expression: Expression::from_str(input).unwrap(),
                fix: Fix::Post
            }
            .validate(),
            expected,
        );
    }
}
