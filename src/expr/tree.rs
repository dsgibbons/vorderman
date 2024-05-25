use super::expr::{Expression, Fix, FixExpression, FixExpressionError, Operation, Token};
use num::rational::Ratio;

enum ExpressionTree {
    Lit(u32),
    Add(Box<ExpressionTree>, Box<ExpressionTree>),
    Subtract(Box<ExpressionTree>, Box<ExpressionTree>),
    Multiply(Box<ExpressionTree>, Box<ExpressionTree>),
    Divide(Box<ExpressionTree>, Box<ExpressionTree>),
}

struct PartialExpressionTree {
    tree: ExpressionTree,
    size: usize,
}

fn parse(tokens: &[Token]) -> PartialExpressionTree {
    if let Token::Number(n) = tokens[0] {
        return PartialExpressionTree {
            tree: ExpressionTree::Lit(n),
            size: 1,
        };
    }

    let lhs = parse(&tokens[1..]);
    let skip = 1 + lhs.size;
    let rhs = parse(&tokens[skip..]);

    let tree = match &tokens[0] {
        Token::Operation(Operation::Add) => {
            ExpressionTree::Add(Box::new(lhs.tree), Box::new(rhs.tree))
        }
        Token::Operation(Operation::Subtract) => {
            ExpressionTree::Subtract(Box::new(lhs.tree), Box::new(rhs.tree))
        }
        Token::Operation(Operation::Multiply) => {
            ExpressionTree::Multiply(Box::new(lhs.tree), Box::new(rhs.tree))
        }
        Token::Operation(Operation::Divide) => {
            ExpressionTree::Divide(Box::new(lhs.tree), Box::new(rhs.tree))
        }
        _ => panic!("Unexpected token encountered"),
    };

    PartialExpressionTree {
        tree,
        size: 1 + lhs.size + rhs.size,
    }
}

impl TryFrom<FixExpression> for ExpressionTree {
    type Error = FixExpressionError;

    fn try_from(input: FixExpression) -> Result<ExpressionTree, Self::Error> {
        match input.as_prefix() {
            Ok(expr) => Ok(parse(&expr.expression.0[..]).tree),
            Err(e) => Err(e),
        }
    }
}

impl ExpressionTree {
    fn evaluate(&self) -> Ratio<i32> {
        match self {
            ExpressionTree::Lit(n) => Ratio::<i32>::from_integer((*n).try_into().unwrap()),
            ExpressionTree::Add(lhs, rhs) => lhs.evaluate() + rhs.evaluate(),
            ExpressionTree::Subtract(lhs, rhs) => lhs.evaluate() - rhs.evaluate(),
            ExpressionTree::Multiply(lhs, rhs) => lhs.evaluate() * rhs.evaluate(),
            ExpressionTree::Divide(lhs, rhs) => lhs.evaluate() / rhs.evaluate(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::str::FromStr;
    use test_case::test_case;

    #[test_case("1", Ratio::<i32>::from_integer(1); "single number")]
    #[test_case("+ 1 2", Ratio::<i32>::from_integer(3); "simple addition")]
    #[test_case("+ 12 34", Ratio::<i32>::from_integer(46); "double digit addition")]
    #[test_case("* 1 - 2 3", Ratio::<i32>::from_integer(-1); "nested operation")]
    #[test_case("* 12 / 345 6789", Ratio::<i32>::new(12 * 345, 6789); "another nested operation")]
    #[test_case("/ / + 3 + 7 * 2 2 2 2", Ratio::<i32>::new(7, 2); "long expression")]
    #[test_case("- * 12 / / + 3 + 7 * 2 2 2 2 / 9 4", Ratio::<i32>::new(159, 4); "another long expression")]
    fn eval_from_str_tests(input: &str, expected: Ratio<i32>) {
        let prefix_expr = FixExpression {
            expression: Expression::from_str(input).unwrap(),
            fix: Fix::Pre,
        };
        let expr_tree = ExpressionTree::try_from(prefix_expr).unwrap();
        assert_eq!(expr_tree.evaluate(), expected)
    }
}
