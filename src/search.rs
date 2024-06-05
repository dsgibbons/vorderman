use super::expr::{Expression, Operation, PostfixExpression, Token};
use num::rational::Ratio;
use std::collections::{BinaryHeap, HashSet};

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct NumbersRound {
    pub numbers: HashSet<usize>,
    pub target: usize,
}

#[derive(Clone, Debug, PartialEq, Eq)]
struct PartialSolution {
    expression: Expression,
    stack: Vec<Ratio<usize>>,
    remaining: HashSet<usize>,
}

impl PartialSolution {
    fn new(numbers: &HashSet<usize>) -> PartialSolution {
        let tokens = Vec::<Token>::new();
        let stack = Vec::<Ratio<usize>>::new();
        PartialSolution {
            expression: Expression(tokens),
            stack,
            remaining: numbers.clone(),
        }
    }
}

#[derive(PartialEq, Debug, Eq)]
struct PrioritizedPartialSolution(usize, Box<PartialSolution>);

impl PartialOrd for PrioritizedPartialSolution {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for PrioritizedPartialSolution {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.0.cmp(&other.0)
    }
}

fn get_options(partial: &PartialSolution) -> Vec<Token> {
    // Populate options to append to end of current partial solution
    let mut options = Vec::<Token>::new();

    // Remaining numbers
    for r in partial.remaining.iter() {
        options.push(Token::Number(*r));
    }

    // All operations if at least two numbers on the stack
    if partial.stack.len() >= 2 {
        let first_num = &partial.stack[partial.stack.len() - 2];
        let second_num = partial.stack.last().unwrap();

        if first_num >= second_num {
            options.push(Token::Operation(Operation::Add));
            options.push(Token::Operation(Operation::Subtract));
            options.push(Token::Operation(Operation::Multiply));
        }

        options.push(Token::Operation(Operation::Divide));
    }

    options
}

fn create_new_partial_solution(old: &PartialSolution, token: &Token) -> Box<PartialSolution> {
    let PartialSolution {
        mut expression,
        mut stack,
        mut remaining,
    } = old.clone();

    match token {
        Token::Number(n) => {
            stack.push(Ratio::<usize>::from_integer((*n).try_into().unwrap()));
            remaining.remove(&n);
        }
        Token::Operation(op) => {
            let last_num = stack.pop().unwrap();
            let first_num = stack.pop().unwrap();

            let result = match op {
                Operation::Add => first_num + last_num,
                Operation::Subtract => first_num - last_num,
                Operation::Multiply => first_num * last_num,
                Operation::Divide => first_num / last_num,
            };

            stack.push(result);
        }
        Token::Parenthesis(_) => panic!("Unexpected parenthesis token found."),
    }

    expression.0.push(*token);

    Box::new(PartialSolution {
        expression,
        stack,
        remaining,
    })
}

fn branch(partial_solution: &PartialSolution) -> Option<Vec<Box<PartialSolution>>> {
    if partial_solution.remaining.len() == 0 && partial_solution.stack.len() == 1 {
        return None;
    }

    if partial_solution.stack.len() > 0 && *partial_solution.stack.last().unwrap().numer() == 0 {
        return None;
    }

    let mut new_partial_solutons = Vec::<Box<PartialSolution>>::new();
    for token in get_options(&partial_solution) {
        new_partial_solutons.push(create_new_partial_solution(&partial_solution, &token));
    }
    Some(new_partial_solutons)
}

pub fn search(config: NumbersRound) -> Option<PostfixExpression> {
    let target = Ratio::<usize>::from_integer(config.target.try_into().unwrap());

    let mut priority_queue = BinaryHeap::<PrioritizedPartialSolution>::new();
    priority_queue.push(PrioritizedPartialSolution(
        1,
        Box::new(PartialSolution::new(&config.numbers)),
    ));

    while let Some(PrioritizedPartialSolution(priority, partial_solution)) = priority_queue.pop() {
        if let Some(new_partial_solutions) = branch(&partial_solution) {
            for new_partial_solution in new_partial_solutions.iter() {
                if new_partial_solution.stack.len() == 1
                    && *new_partial_solution.stack.first().unwrap() == target
                {
                    return Some(PostfixExpression(new_partial_solution.expression.clone()));
                }

                priority_queue.push(PrioritizedPartialSolution(1, new_partial_solution.clone()));
                // TODO: how can clone be avoided here?
            }
        }
    }
    None
}

#[cfg(test)]
mod tests {
    use super::*;
    use test_case::test_case;

    #[test_case(3, vec![1, 2] ; "0")]
    #[test_case(321, vec![1, 2, 3, 5, 10, 100] ; "1")]
    #[test_case(120, vec![8, 3, 7, 2, 5, 4] ; "2")]
    #[test_case(615, vec![25, 3, 7, 2, 5, 4] ; "3")]
    #[test_case(813, vec![1, 10, 25, 50, 75, 100] ; "4")]
    #[test_case(952, vec![3, 6, 25, 50, 75, 100] ; "5")]
    fn valid_numbers_round(target: usize, numbers: Vec<usize>) {
        let solution = search(NumbersRound {
            numbers: HashSet::from_iter(numbers),
            target,
        });

        assert_eq!(
            solution.unwrap().evaluate().unwrap(),
            Ratio::<isize>::from_integer(target.try_into().unwrap())
        );
    }

    #[test_case(30, vec![1, 2] ; "0")]
    #[test_case(3000, vec![2, 3, 5, 10] ; "1")]
    fn impossible_numbers_round(target: usize, numbers: Vec<usize>) {
        let solution = search(NumbersRound {
            numbers: HashSet::from_iter(numbers),
            target,
        });

        assert!(solution.is_none(),);
    }
}
