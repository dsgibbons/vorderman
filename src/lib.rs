use num::rational::Ratio;
use std::fmt;
use std::str::FromStr;

use regex::Regex;

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

fn clean_string(s: &str) -> String {
    let re = Regex::new(r"[^0-9+\-*\/x()]").unwrap();
    re.replace_all(s, "").to_string()
}

// first char must be numerical or (
// last char must be numerical or )

// get numerical indices
// get opening paren indices
// get closing paren indices
// get operation indices

// check validity
// build expression

fn find_next_node<T>(s: &str) -> Result<NextObjectFromStringStart<Node<T>>, ParseExpressionError>
where
    T: std::str::FromStr,
    <T as std::str::FromStr>::Err: std::fmt::Debug,
{
    let first_char = s.chars().nth(0).unwrap();

    if first_char == '(' {
        let mut paren_count = 1;
        for (i, c) in s.char_indices() {
            if c == ')' {
                paren_count -= 1;
            } else if c == '(' {
                paren_count += 1;
            }
            if paren_count == 0 {
                return Ok(NextObjectFromStringStart {
                    object: Node::Expression(Box::new(T::from_str(&s[..i]).unwrap())),
                    n_chars: i,
                });
            }
        }
    } else if !first_char.is_digit(10) && first_char != ' ' {
        return Err(ParseExpressionError::InvalidCharacter(first_char, 0));
    }

    for (i, c) in s.char_indices() {
        if !c.is_digit(10) {
            return Ok(NextObjectFromStringStart {
                object: Node::Number(s[..i].parse().unwrap()),
                n_chars: i,
            });
        }
    }

    Ok(NextObjectFromStringStart {
        object: Node::Number(s.parse().unwrap()),
        n_chars: s.len(),
    })
}

fn find_next_operation(
    s: &str,
) -> Result<NextObjectFromStringStart<Operation>, ParseExpressionError> {
    Ok(NextObjectFromStringStart {
        object: Operation::Add,
        n_chars: 1,
    })
}

struct NextObjectFromStringStart<T> {
    object: T,
    n_chars: usize,
}

#[derive(Debug)]
enum ParseExpressionError {
    NotEnoughNodes,
    TooManyNodes,
    NotEnoughOperations,
    TooManyOperations,
    InvalidCharacter(char, usize),
}

// fn get_strings_in_outer_parentheses(s: &str)

impl FromStr for VecExpression {
    type Err = crate::ParseExpressionError;

    fn from_str(input: &str) -> Result<VecExpression, Self::Err> {
        let mut next_index = 0;
        let next_node: NextObjectFromStringStart<Node<VecExpression>>;
        let next_operation: NextObjectFromStringStart<Operation>;

        let mut result = VecExpression::new();

        next_node = find_next_node(&input[next_index..]).unwrap();
        result.nodes.push(next_node.object);
        next_index += next_node.n_chars;

        if next_index != input.len() {
            next_operation = find_next_operation(&input[next_index..]).unwrap();
            result.edges.push(next_operation.object);
            next_index += next_operation.n_chars;

            if next_index == input.len() {
                return Err(ParseExpressionError::NotEnoughOperations);
            };
        };

        Ok(result)
    }
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
    fn clean_string() {
        assert_eq!(
            super::clean_string("1 .. s 2 x 4* (23+ 9) 1 2 -12+(0 )&ujq  "),
            "12x4*(23+9)12-12+(0)"
        );
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

    #[test]
    fn vec_expr_from_and_to_str() {
        assert_eq!(VecExpression::from_str("123").unwrap().to_string(), "123")
    }
}
