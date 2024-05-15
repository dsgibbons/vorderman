use super::op::{Operand, Operation};

const BASE: u32 = 10;

#[derive(Debug)]
pub enum ParseExpressionError {
    NotEnoughOperands,
    TooManyOperands,
    NotEnoughOperations,
    TooManyOperations,
    InvalidCharacter(char, usize),
    AllWhitespace,
}

fn get_first_non_empty_char(s: &str) -> Result<(usize, char), ParseExpressionError> {
    for (i, c) in s.char_indices() {
        if c != ' ' {
            return Ok((i, c));
        }
    }
    Err(ParseExpressionError::AllWhitespace)
}

#[derive(Debug)]
pub struct NextObjectFromStringStart<T> {
    pub object: T,
    pub end_char_index: usize,
}

pub fn find_next_operand<T>(
    s: &str,
) -> Result<NextObjectFromStringStart<Operand<T>>, ParseExpressionError>
where
    T: std::str::FromStr,
    <T as std::str::FromStr>::Err: std::fmt::Debug,
{
    let (offset, first_char) = get_first_non_empty_char(s).unwrap();

    if first_char == '(' {
        let mut paren_count = 1;
        for (i, c) in s[offset + 1..].char_indices() {
            if c == ')' {
                paren_count -= 1;
            } else if c == '(' {
                paren_count += 1;
            }
            if paren_count == 0 {
                return Ok(NextObjectFromStringStart {
                    object: Operand::Expression(Box::new(
                        T::from_str(&s[offset + 1..offset + i + 1]).unwrap(),
                    )),
                    end_char_index: offset + 1 + i,
                });
            }
        }
    } else if !first_char.is_digit(BASE) {
        return Err(ParseExpressionError::InvalidCharacter(first_char, offset));
    }

    for (i, c) in s[offset..].char_indices() {
        if !c.is_digit(BASE) {
            return Ok(NextObjectFromStringStart {
                object: Operand::Number(s[offset..offset + i].parse().unwrap()),
                end_char_index: offset + i - 1,
            });
        }
    }

    Ok(NextObjectFromStringStart {
        object: Operand::Number(s[offset..].parse().unwrap()),
        end_char_index: s.len() - 1,
    })
}

pub fn find_next_operation(
    s: &str,
) -> Result<NextObjectFromStringStart<Operation>, ParseExpressionError> {
    for (i, c) in s.char_indices() {
        if c != ' ' {
            return Ok(NextObjectFromStringStart {
                object: Operation::try_from(c).unwrap(),
                end_char_index: i,
            });
        }
    }
    Err(ParseExpressionError::NotEnoughOperations)
}
