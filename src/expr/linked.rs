use super::op::{Operand, Operation};
use std::fmt;

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
}
