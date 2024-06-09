use super::expr::{Expression, Operation, PostfixExpression, Token};
use super::round::NumbersRound;
use num::rational::Ratio;

#[derive(Clone, Debug, PartialEq, Eq)]
struct State {
    expression: Expression,
    stack: Vec<Ratio<usize>>,
    remaining: Vec<usize>,
    target: Ratio<usize>,
    solutions: Vec<PostfixExpression>,
    history: Vec<Ratio<usize>>,
    n_iterations: usize,
}

impl State {
    fn new(config: NumbersRound) -> State {
        State {
            expression: Expression::new(),
            stack: Vec::<Ratio<usize>>::new(),
            remaining: config.numbers,
            solutions: Vec::<PostfixExpression>::new(),
            target: Ratio::<usize>::from_integer(config.target.try_into().unwrap()),
            history: Vec::<Ratio<usize>>::new(),
            n_iterations: 0,
        }
    }

    fn get_options(&self) -> Vec<Token> {
        // Populate options to append to end of current partial solution
        let mut options = Vec::<Token>::new();

        // Remaining numbers
        for r in self.remaining.iter() {
            options.push(Token::Number(*r));
        }

        // All operations if at least two numbers on the stack
        if self.stack.len() >= 2 {
            let first_num = &self.stack[self.stack.len() - 2];
            let second_num = self.stack.last().unwrap();

            if first_num >= second_num {
                options.push(Token::Operation(Operation::Add));
                options.push(Token::Operation(Operation::Subtract));
                options.push(Token::Operation(Operation::Multiply));
            }

            options.push(Token::Operation(Operation::Divide));
        }

        options
    }

    fn find_solutions(&self) -> State {
        let mut return_state = self.clone();

        println!("{}", return_state.expression);

        return_state.n_iterations += 1;

        if self.stack.len() == 1 && *self.stack.first().unwrap() == self.target {
            return_state
                .solutions
                .push(PostfixExpression(self.expression.clone()));
            return return_state; // early exit for now
        }

        if self.remaining.len() == 0 && self.stack.len() == 1 {
            return return_state;
        }

        if self.stack.len() > 0 && *self.stack.last().unwrap().numer() == 0 {
            return return_state;
        }

        let options = self.get_options();
        for token in options {
            let mut next_state = return_state.compute_next(token);
            next_state = next_state.find_solutions();
            return_state = next_state.revert();
            if return_state.solutions.len() > 0 {
                break;
            }
        }

        return_state
    }

    fn compute_next(&self, token: Token) -> State {
        let mut next_state = self.clone();

        match token {
            Token::Number(n) => {
                next_state
                    .stack
                    .push(Ratio::<usize>::from_integer((n).try_into().unwrap()));

                next_state.remaining = {
                    let mut new_vec = Vec::new();
                    let mut found = false;
                    for elem in next_state.remaining.iter() {
                        if *elem == n && !found {
                            found = true;
                            continue;
                        }
                        new_vec.push(*elem);
                    }
                    new_vec
                };
            }
            Token::Operation(op) => {
                let last_num = next_state.stack.pop().unwrap();
                let first_num = next_state.stack.pop().unwrap();

                let result = match op {
                    Operation::Add => first_num + last_num,
                    Operation::Subtract => first_num - last_num,
                    Operation::Multiply => first_num * last_num,
                    Operation::Divide => first_num / last_num,
                };

                next_state.stack.push(result);

                // History pushed in reverse order so they come off the stack later in correct order
                next_state.history.push(last_num);
                next_state.history.push(first_num);
            }
            Token::Parenthesis(_) => panic!("Unexpected parenthesis token found."),
        }

        next_state.expression.0.push(token);

        next_state
    }

    fn revert(&self) -> State {
        let mut reverted = self.clone();

        reverted.stack.pop();
        let last_token = reverted.expression.0.pop().unwrap();

        match last_token {
            Token::Number(n) => {
                reverted.remaining.push(n);
            }
            Token::Operation(_) => {
                reverted.stack.push(reverted.history.pop().unwrap());
                reverted.stack.push(reverted.history.pop().unwrap());
            }
            Token::Parenthesis(_) => panic!("Unexpected parenthesis token found."),
        }

        reverted
    }
}

pub fn search(config: NumbersRound) -> Option<PostfixExpression> {
    let init_state = State::new(config);
    let end_state = init_state.find_solutions();
    end_state.solutions.first().cloned()
}

#[cfg(test)]
mod tests {
    use super::*;
    use test_case::test_case;

    #[test_case(3, vec![1, 2] ; "0")]
    #[test_case(321, vec![1, 2, 3, 5, 10, 100] ; "1")]
    #[test_case(322, vec![2, 2, 3, 5, 10, 100] ; "2")]
    #[test_case(120, vec![8, 3, 7, 2, 5, 4] ; "3")]
    #[test_case(615, vec![25, 3, 7, 2, 5, 4] ; "4")]
    #[test_case(813, vec![1, 10, 25, 50, 75, 100] ; "5")]
    #[test_case(952, vec![3, 6, 25, 50, 75, 100] ; "6")]
    fn valid_numbers_round(target: usize, numbers: Vec<usize>) {
        let solution = search(NumbersRound {
            numbers: numbers,
            target,
        });

        assert_eq!(
            solution.unwrap().evaluate().unwrap(),
            Ratio::<isize>::from_integer(target.try_into().unwrap())
        );
    }

    #[test_case(30, vec![1, 2] ; "0")]
    #[test_case(3000, vec![2, 3, 5, 10] ; "1")]
    #[test_case(30000, vec![2, 3, 5, 8, 10] ; "2")]
    #[test_case(300000, vec![2, 3, 5, 8, 9, 10] ; "3")]
    fn impossible_numbers_round(target: usize, numbers: Vec<usize>) {
        let solution = search(NumbersRound {
            numbers: numbers,
            target,
        });

        assert!(solution.is_none(),);
    }
}
