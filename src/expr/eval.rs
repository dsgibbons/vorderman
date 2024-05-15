use num::rational::Ratio;

use super::linked::{Edge, LinkedExpression, Node};
use super::op::{Operand, Operation};
use super::vec::VecExpression;

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
        // TODO: Validate expression
        Ok(Ratio::from_integer(0))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn null_vec_expression() {
        let mut expr = VecExpression::new();
        expr.operands.push(Operand::Number(0));
        assert_eq!(expr.evaluate().unwrap(), Ratio::from_integer(0));
    }

    #[test]
    fn null_linked_expression() {
        let expr = LinkedExpression(Node::Number(0), None);
        assert_eq!(expr.evaluate().unwrap(), Ratio::from_integer(0));
    }
}
