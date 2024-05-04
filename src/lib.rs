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
    operand: NestedExpression,
}

enum Node {
    Number(u32),
    Expression(NestedExpression),
}

struct NestedExpression(Box<Node>, Option<Box<Edge>>);

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
// to str method - test that to and from is reversible
// test solutions can be evaluated
// DFS for building solutions towards target

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn null_flat_expression() {
        let mut expression = FlatExpression::new();
        expression.numbers.push(0);
        assert_eq!(expression.evaluate().unwrap(), 0.0);
    }

    #[test]
    fn null_nested_expression() {
        let expression = NestedExpression(Box::new(Node::Number(0)), None);
        assert_eq!(expression.evaluate().unwrap(), 0.0);
    }
}
