use super::op::{Operand, Operation};
use super::parse;
use std::fmt;
use std::str::FromStr;

pub type Node = Operand<LinkedExpression>;

#[derive(Debug)]
pub struct Edge {
    operation: Operation,
    operand: Box<LinkedExpression>,
}

impl fmt::Display for Edge {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{} {}", self.operation, self.operand)
    }
}

#[derive(Debug)]
pub struct LinkedExpression(pub Node, pub Option<Edge>);

impl fmt::Display for LinkedExpression {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match &self.1 {
            Some(e) => write!(f, "{} {}", self.0, e),
            None => write!(f, "{}", self.0),
        }
    }
}

impl FromStr for LinkedExpression {
    type Err = parse::ParseExpressionError;

    fn from_str(input: &str) -> Result<LinkedExpression, Self::Err> {
        let input = input.trim();

        let mut next_index = 0;

        let next_operand = parse::find_next_operand(&input[next_index..]).unwrap();
        next_index += next_operand.end_char_index + 1;

        let edge: Option<Edge>;
        if next_index < input.len() - 1 {
            let next_operation = parse::find_next_operation(&input[next_index..]).unwrap();
            next_index += next_operation.end_char_index + 1;
            if next_index == input.len() {
                return Err(parse::ParseExpressionError::NotEnoughOperations);
            };
            edge = Some(Edge {
                operation: next_operation.object,
                operand: Box::new(LinkedExpression::from_str(&input[next_index..]).unwrap()),
            });
        } else {
            edge = None;
        }

        Ok(LinkedExpression(next_operand.object, edge))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn linked_expression_to_string() {
        let expr = LinkedExpression(
            Node::Number(1),
            Some(Edge {
                operation: Operation::Add,
                operand: Box::new(LinkedExpression(
                    Node::Number(2),
                    Some(Edge {
                        operation: Operation::Subtract,
                        operand: Box::new(LinkedExpression(Node::Number(3), None)),
                    }),
                )),
            }),
        );

        assert_eq!(expr.to_string(), "1 + 2 - 3");
    }

    #[test]
    fn nested_linked_expression_to_string() {
        let inner_expr = LinkedExpression(
            Node::Number(2),
            Some(Edge {
                operation: Operation::Subtract,
                operand: Box::new(LinkedExpression(Node::Number(3), None)),
            }),
        );

        let outer_expr = LinkedExpression(
            Node::Number(1),
            Some(Edge {
                operation: Operation::Add,
                operand: Box::new(LinkedExpression(
                    Node::Expression(Box::new(inner_expr)),
                    Some(Edge {
                        operation: Operation::Multiply,
                        operand: Box::new(LinkedExpression(
                            Node::Number(4),
                            Some(Edge {
                                operation: Operation::Divide,
                                operand: Box::new(LinkedExpression(Node::Number(5), None)),
                            }),
                        )),
                    }),
                )),
            }),
        );

        assert_eq!(outer_expr.to_string(), "1 + (2 - 3) * 4 / 5");
    }

    macro_rules! str_to_linked_expr_tests {
        ($($name:ident: $value:expr,)*) => {
        $(
            #[test]
            fn $name() {
                let (input, expected) = $value;
                assert_eq!(LinkedExpression::from_str(input).unwrap().to_string(), expected);
            }
        )*
        }
    }

    str_to_linked_expr_tests! {
        str_to_link_0: ("1", "1"),
        str_to_link_1: ("1 + 2", "1 + 2"),
        str_to_link_2: ("(1)", "(1)"),
        str_to_link_3: ("(1*2 ) + (3 - (4/ 5)) + 6 /((7) - (8 * (  9 +10 )))   ", "(1 * 2) + (3 - (4 / 5)) + 6 / ((7) - (8 * (9 + 10)))"),
        str_to_link_4: ("((      4 + 5     ))", "((4 + 5))"),
    }
}
