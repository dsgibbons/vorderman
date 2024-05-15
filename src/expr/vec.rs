use super::op::{Operand, Operation};
use super::parse;
use std::fmt;
use std::str::FromStr;

#[derive(Debug)]
pub struct VecExpression {
    pub operands: Vec<Operand<VecExpression>>,
    pub operations: Vec<Operation>,
}

#[derive(Debug)]
enum VecExpressionError {
    NoOperands,
    TooManyOperations,
    TooFewOperations,
    ParseError,
}

impl VecExpression {
    pub fn new() -> VecExpression {
        VecExpression {
            operands: Vec::<Operand<VecExpression>>::new(),
            operations: Vec::<Operation>::new(),
        }
    }
}

impl fmt::Display for VecExpression {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        // TODO: Validate expression
        let mut characters = Vec::new();
        for (n, e) in self.operands.iter().zip(self.operations.iter()) {
            characters.push(n.to_string());
            characters.push(e.to_string());
        }
        characters.push(self.operands.last().unwrap().to_string());

        write!(f, "{}", characters.join(" "))
    }
}

impl FromStr for VecExpression {
    type Err = parse::ParseExpressionError;

    fn from_str(input: &str) -> Result<VecExpression, Self::Err> {
        let input = input.trim();

        let mut next_index = 0;
        let mut next_operand: parse::NextObjectFromStringStart<Operand<VecExpression>>;
        let mut next_operation: parse::NextObjectFromStringStart<Operation>;
        let mut result = VecExpression::new();

        while next_index < input.len() {
            next_operand = parse::find_next_operand(&input[next_index..]).unwrap();

            result.operands.push(next_operand.object);
            next_index += next_operand.end_char_index + 1;

            if next_index >= input.len() - 1 {
                break;
            }

            next_operation = parse::find_next_operation(&input[next_index..]).unwrap();

            result.operations.push(next_operation.object);
            next_index += next_operation.end_char_index + 1;

            if next_index == input.len() {
                return Err(parse::ParseExpressionError::NotEnoughOperations);
            };
        }

        Ok(result)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn vec_expression_to_string() {
        let operands = vec![
            Operand::Number(1),
            Operand::Number(2),
            Operand::Number(3),
            Operand::Number(4),
            Operand::Number(5),
        ];
        let operations = vec![
            Operation::Add,
            Operation::Subtract,
            Operation::Multiply,
            Operation::Divide,
        ];
        let expr = VecExpression {
            operands,
            operations,
        };

        assert_eq!(expr.to_string(), "1 + 2 - 3 * 4 / 5");
    }

    #[test]
    fn nested_vec_expression_to_string() {
        let inner_nodes = vec![Operand::Number(2), Operand::Number(3)];
        let inner_edges = vec![Operation::Subtract];
        let inner_expr = VecExpression {
            operands: inner_nodes,
            operations: inner_edges,
        };
        let outer_nodes = vec![
            Operand::Number(1),
            Operand::Expression(Box::new(inner_expr)),
            Operand::Number(4),
            Operand::Number(5),
        ];
        let outer_edges = vec![Operation::Add, Operation::Multiply, Operation::Divide];
        let expr = VecExpression {
            operands: outer_nodes,
            operations: outer_edges,
        };

        assert_eq!(expr.to_string(), "1 + (2 - 3) * 4 / 5");
    }

    macro_rules! str_to_vec_expr_tests {
        ($($name:ident: $value:expr,)*) => {
        $(
            #[test]
            fn $name() {
                let (input, expected) = $value;
                assert_eq!(VecExpression::from_str(input).unwrap().to_string(), expected);
            }
        )*
        }
    }

    str_to_vec_expr_tests! {
        str_to_vec_0: ("1", "1"),
        str_to_vec_1: ("1 + 2", "1 + 2"),
        str_to_vec_2: ("(1)", "(1)"),
        str_to_vec_3: ("(1*2 ) + (3 - (4/ 5)) + 6 /((7) - (8 * (  9 +10 )))   ", "(1 * 2) + (3 - (4 / 5)) + 6 / ((7) - (8 * (9 + 10)))"),
        str_to_vec_4: ("((      4 + 5     ))", "((4 + 5))"),
    }
}
