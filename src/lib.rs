use num::rational::Ratio;
use std::fmt;
use std::str::FromStr;

struct NumbersRound {
    numbers: Vec<u32>,
    target: Ratio<u32>,
}

enum Operation {
    Add,
    Subtract,
    Multiply,
    Divide,
}

enum OperationError {
    InvalidOperation,
}

impl FromStr for Operation {
    type Err = crate::OperationError;

    fn from_str(input: &str) -> Result<Operation, Self::Err> {
        match input {
            "+" => Ok(Operation::Add),
            "-" => Ok(Operation::Subtract),
            "*" | "x" => Ok(Operation::Multiply),
            "/" => Ok(Operation::Divide),
            _ => Err(OperationError::InvalidOperation),
        }
    }
}

impl fmt::Display for Operation {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let character = match self {
            Operation::Add => "+",
            Operation::Subtract => "-",
            Operation::Multiply => "*",
            Operation::Divide => "/",
        };
        write!(f, "{}", character)
    }
}

struct Edge {
    operation: Operation,
    operand: Box<LinkedExpression>,
}

enum Node<T> {
    Number(u32),
    Expression(Box<T>),
}

impl<T: fmt::Display> fmt::Display for Node<T> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Node::Number(n) => write!(f, "{}", n),
            Node::Expression(e) => write!(f, "({})", *e),
        }
    }
}

impl fmt::Display for Edge {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{} {}", self.operation, self.operand)
    }
}

struct LinkedExpression(Node<LinkedExpression>, Option<Edge>);

struct VecExpression {
    nodes: Vec<Node<VecExpression>>,
    edges: Vec<Operation>,
}

impl VecExpression {
    fn new() -> VecExpression {
        VecExpression {
            nodes: Vec::<Node<VecExpression>>::new(),
            edges: Vec::<Operation>::new(),
        }
    }
}

impl fmt::Display for VecExpression {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut characters = Vec::new();
        for (n, e) in self.nodes.iter().zip(self.edges.iter()) {
            characters.push(n.to_string());
            characters.push(e.to_string());
        }
        characters.push(self.nodes.last().unwrap().to_string());

        write!(f, "{}", characters.join(" "))
    }
}

impl fmt::Display for LinkedExpression {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match &self.1 {
            Some(e) => write!(f, "{} {}", self.0, e),
            None => write!(f, "{}", self.0),
        }
    }
}

enum VecExpressionError {
    NoNodes,
    TooManyEdges,
    TooFewEdges,
    ParseError,
}

#[derive(Debug)]
enum EvaluationError {
    GenericError,
}

trait Evaluatable {
    fn evaluate(&self) -> Result<Ratio<u32>, EvaluationError>;
}

impl Evaluatable for LinkedExpression {
    fn evaluate(&self) -> Result<Ratio<u32>, EvaluationError> {
        Ok(Ratio::from_integer(0))
    }
}

impl Evaluatable for VecExpression {
    fn evaluate(&self) -> Result<Ratio<u32>, EvaluationError> {
        Ok(Ratio::from_integer(0))
    }
}

// construct solutions
// write from_str and bidirectional tests
// random generate expressions for testing
// test solutions can be evaluated
// DFS for building solutions towards target

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn null_vec_expression() {
        let mut expr = VecExpression::new();
        expr.nodes.push(Node::Number(0));
        assert_eq!(expr.evaluate().unwrap(), Ratio::from_integer(0));
    }

    #[test]
    fn null_linked_expression() {
        let expr = LinkedExpression(Node::Number(0), None);
        assert_eq!(expr.evaluate().unwrap(), Ratio::from_integer(0));
    }

    #[test]
    fn vec_expression_to_string() {
        let nodes = vec![
            Node::Number(1),
            Node::Number(2),
            Node::Number(3),
            Node::Number(4),
            Node::Number(5),
        ];
        let edges = vec![
            Operation::Add,
            Operation::Subtract,
            Operation::Multiply,
            Operation::Divide,
        ];
        let expr = VecExpression { nodes, edges };

        assert_eq!(expr.to_string(), "1 + 2 - 3 * 4 / 5");
    }

    #[test]
    fn nested_vec_expression_to_string() {
        let inner_nodes = vec![Node::Number(2), Node::Number(3)];
        let inner_edges = vec![Operation::Subtract];
        let inner_expr = VecExpression {
            nodes: inner_nodes,
            edges: inner_edges,
        };
        let outer_nodes = vec![
            Node::Number(1),
            Node::Expression(Box::new(inner_expr)),
            Node::Number(4),
            Node::Number(5),
        ];
        let outer_edges = vec![Operation::Add, Operation::Multiply, Operation::Divide];
        let expr = VecExpression {
            nodes: outer_nodes,
            edges: outer_edges,
        };

        assert_eq!(expr.to_string(), "1 + (2 - 3) * 4 / 5");
    }

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
