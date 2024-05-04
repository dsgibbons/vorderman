struct NumbersRound {
    numbers: Vec<u32>,
    target: u32,
}

enum Operation {
    Add,
    Subtract,
    Multiply,
    Divide,
}

struct Edge {
    operation: Operation,
    operand: Node,
}

enum Node {
    Number(u32, Option<Box<Edge>>),
    Expression(Box<Node>),
}

type NestedExpression = Node;

struct FlatExpression {
    numbers: Vec<u32>,
    operations: Vec<Operation>,
}

impl FlatExpression {
    fn new() -> FlatExpression {
        FlatExpression {
            numbers: Vec::<u32>::new(),
            operations: Vec::<Operation>::new(),
        }
    }
}

enum FlatExpressionError {
    NoNumbers,
    TooManyOperations,
    TooFewOperations,
}

#[derive(Debug)]
enum EvaluationError {
    GenericError,
}

trait Evaluatable {
    fn evaluate(&self) -> Result<f32, EvaluationError>;
}

impl Evaluatable for NestedExpression {
    fn evaluate(&self) -> Result<f32, EvaluationError> {
        Ok(0.0)
    }
}

impl Evaluatable for FlatExpression {
    fn evaluate(&self) -> Result<f32, EvaluationError> {
        Ok(0.0)
    }
}

// construct solutions
// from str method
// test solutions can be evaluated
// DFS for building solutions towards target

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn null_flat_expression() {
        let expression = FlatExpression::new();
        assert_eq!(expression.evaluate().unwrap(), 0.0);
    }

    #[test]
    fn null_nested_expression() {
        let expression = Node::Number(0, None);
        assert_eq!(expression.evaluate().unwrap(), 0.0);
    }
}
