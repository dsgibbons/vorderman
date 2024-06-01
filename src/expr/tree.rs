use super::expr::{Expression, Fix, FixExpression, FixExpressionError, Operation, Token};
use num::rational::Ratio;

pub enum ExpressionTree {
    Lit(usize),
    Add(Box<ExpressionTree>, Box<ExpressionTree>),
    Subtract(Box<ExpressionTree>, Box<ExpressionTree>),
    Multiply(Box<ExpressionTree>, Box<ExpressionTree>),
    Divide(Box<ExpressionTree>, Box<ExpressionTree>),
}

pub struct SizedExpressionTree {
    pub tree: ExpressionTree,
    pub size: usize,
}

fn parse(tokens: &[Token]) -> SizedExpressionTree {
    if let Token::Number(n) = tokens[0] {
        return SizedExpressionTree {
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

    SizedExpressionTree {
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
    pub fn evaluate(&self) -> Ratio<i32> {
        match self {
            ExpressionTree::Lit(n) => Ratio::<i32>::from_integer((*n).try_into().unwrap()),
            ExpressionTree::Add(lhs, rhs) => lhs.evaluate() + rhs.evaluate(),
            ExpressionTree::Subtract(lhs, rhs) => lhs.evaluate() - rhs.evaluate(),
            ExpressionTree::Multiply(lhs, rhs) => lhs.evaluate() * rhs.evaluate(),
            ExpressionTree::Divide(lhs, rhs) => lhs.evaluate() / rhs.evaluate(),
        }
    }

    fn as_tokens(&self) -> Vec<Token> {
        let mut tokens = Vec::<Token>::new();

        let (t, lhs, rhs) = match self {
            ExpressionTree::Lit(n) => {
                tokens.push(Token::Number(*n));
                return tokens;
            }
            ExpressionTree::Add(l, r) => (
                Token::Operation(Operation::Add),
                l.as_tokens(),
                r.as_tokens(),
            ),
            ExpressionTree::Subtract(l, r) => (
                Token::Operation(Operation::Subtract),
                l.as_tokens(),
                r.as_tokens(),
            ),
            ExpressionTree::Multiply(l, r) => (
                Token::Operation(Operation::Multiply),
                l.as_tokens(),
                r.as_tokens(),
            ),
            ExpressionTree::Divide(l, r) => (
                Token::Operation(Operation::Divide),
                l.as_tokens(),
                r.as_tokens(),
            ),
        };

        tokens.push(t);
        tokens.extend(lhs);
        tokens.extend(rhs);

        tokens
    }
}

impl From<ExpressionTree> for FixExpression {
    fn from(value: ExpressionTree) -> Self {
        let tokens = value.as_tokens();

        FixExpression {
            expression: Expression(tokens),
            fix: Fix::Pre,
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

    #[test_case("1"; "single number")]
    #[test_case("+ 1 2"; "simple addition")]
    #[test_case("+ 12 34"; "double digit addition")]
    #[test_case("* 1 - 2 3"; "nested operation")]
    #[test_case("* 12 / 345 6789"; "another nested operation")]
    #[test_case("/ / + 3 + 7 * 2 2 2 2"; "long expression")]
    #[test_case("- * 12 / / + 3 + 7 * 2 2 2 2 / 9 4"; "another long expression")]
    fn e2e_str_tests(input: &str) {
        let prefix_expr = FixExpression {
            expression: Expression::from_str(input).unwrap(),
            fix: Fix::Pre,
        };
        let expr_tree = ExpressionTree::try_from(prefix_expr).unwrap();
        assert_eq!(FixExpression::from(expr_tree).expression.to_string(), input)
    }
}
