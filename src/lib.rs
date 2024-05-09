use num::rational::Ratio;

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

struct Edge {
    operation: Operation,
    operand: Box<LinkedExpression>,
}

enum Node<T> {
    Number(u32),
    Expression(Box<T>),
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

enum VecExpressionError {
    NoNodes,
    TooManyEdges,
    TooFewEdges,
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
// from str method
// to str method - test that to and from is reversible
// random generate expressions for testing
// test solutions can be evaluated
// DFS for building solutions towards target

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn null_vec_expression() {
        let mut expression = VecExpression::new();
        expression.nodes.push(Node::Number(0));
        assert_eq!(expression.evaluate().unwrap(), Ratio::from_integer(0));
    }

    #[test]
    fn null_nested_expression() {
        let expression = LinkedExpression(Node::Number(0), None);
        assert_eq!(expression.evaluate().unwrap(), Ratio::from_integer(0));
    }
}
