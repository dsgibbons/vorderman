use super::expr::{Expression, Operation, PostfixExpression, Token};
use super::round::NumbersRound;
use num::rational::Ratio;

#[derive(Debug)]
struct State {
    expression: Expression,
    stack: Vec<Ratio<usize>>,
    remaining: Vec<usize>,
    target: Ratio<usize>,
    solutions: Vec<PostfixExpression>,
    history: Vec<Ratio<usize>>,
    n_iterations: usize,
    allow_fractional_intermediate_values: bool,
}

impl State {
    fn new(numbers_round: NumbersRound, allow_fractional_intermediate_values: bool) -> State {
        State {
            expression: Expression::new(),
            stack: Vec::<Ratio<usize>>::new(),
            remaining: numbers_round.numbers,
            solutions: Vec::<PostfixExpression>::new(),
            target: Ratio::<usize>::from_integer(numbers_round.target.try_into().unwrap()),
            history: Vec::<Ratio<usize>>::new(),
            n_iterations: 0,
            allow_fractional_intermediate_values: allow_fractional_intermediate_values,
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

            if self.allow_fractional_intermediate_values
                || (first_num >= second_num && *(first_num % second_num).numer() == 0)
            {
                options.push(Token::Operation(Operation::Divide));
            }
        }

        options
    }

    fn find_solutions(&mut self) {
        // println!("{}", self.expression);

        self.n_iterations += 1;

        if self.stack.len() == 1 && *self.stack.first().unwrap() == self.target {
            self.solutions
                .push(PostfixExpression(self.expression.clone()));
            return; // early exit for now
        }

        if self.remaining.len() == 0 && self.stack.len() == 1 {
            return;
        }

        if self.stack.len() > 0 && *self.stack.last().unwrap().numer() == 0 {
            return;
        }

        let options = self.get_options();
        for token in options {
            self.compute_next(token);
            self.find_solutions();
            self.revert();
            if self.solutions.len() > 0 {
                break;
            }
        }
    }

    fn compute_next(&mut self, token: Token) {
        match token {
            Token::Number(n) => {
                self.stack
                    .push(Ratio::<usize>::from_integer((n).try_into().unwrap()));

                self.remaining = {
                    let mut new_vec = Vec::new();
                    let mut found = false;
                    for elem in self.remaining.iter() {
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
                let last_num = self.stack.pop().unwrap();
                let first_num = self.stack.pop().unwrap();

                let result = match op {
                    Operation::Add => first_num + last_num,
                    Operation::Subtract => first_num - last_num,
                    Operation::Multiply => first_num * last_num,
                    Operation::Divide => first_num / last_num,
                };

                self.stack.push(result);

                // History pushed in reverse order so they come off the stack later in correct order
                self.history.push(last_num);
                self.history.push(first_num);
            }
            Token::Parenthesis(_) => panic!("Unexpected parenthesis token found."),
        }

        self.expression.0.push(token);
    }

    fn revert(&mut self) {
        self.stack.pop();
        let last_token = self.expression.0.pop().unwrap();

        match last_token {
            Token::Number(n) => {
                self.remaining.push(n);
            }
            Token::Operation(_) => {
                self.stack.push(self.history.pop().unwrap());
                self.stack.push(self.history.pop().unwrap());
            }
            Token::Parenthesis(_) => panic!("Unexpected parenthesis token found."),
        }
    }
}

pub fn search(
    numbers_round: NumbersRound,
    allow_fractional_intermediate_values: bool,
) -> Option<PostfixExpression> {
    let mut init_state = State::new(numbers_round, allow_fractional_intermediate_values);
    init_state.find_solutions();
    init_state.solutions.first().cloned()
}

#[cfg(test)]
mod tests {
    use super::*;
    use test_case::test_case;

    #[test_case(3, vec![1, 2])]
    // #[test_case(42, vec![1, 4, 5, 8])]  // example that requires intermediate ratio division: ((1/4) + 5) * 8
    #[test_case(55, vec![6, 5, 1, 25])]
    #[test_case(146, vec![10, 7, 9, 14])]
    #[test_case(321, vec![1, 2, 3, 5, 10, 100])]
    #[test_case(322, vec![2, 2, 3, 5, 10, 100])]
    #[test_case(120, vec![8, 3, 7, 2, 5, 4])]
    #[test_case(615, vec![25, 3, 7, 2, 5, 4])]
    #[test_case(813, vec![1, 10, 25, 50, 75, 100])]
    #[test_case(952, vec![3, 6, 25, 50, 75, 100])]
    fn valid_numbers_round(target: usize, numbers: Vec<usize>) {
        let solution = search(
            NumbersRound {
                numbers: numbers,
                target,
            },
            false,
        );

        assert_eq!(
            solution.unwrap().evaluate().unwrap(),
            Ratio::<isize>::from_integer(target.try_into().unwrap())
        );
    }

    #[test_case(42, vec![1, 4, 5, 8])]
    fn valid_numbers_round_with_intermediate_fractions(target: usize, numbers: Vec<usize>) {
        let solution = search(
            NumbersRound {
                numbers: numbers,
                target,
            },
            true,
        );

        assert_eq!(
            solution.unwrap().evaluate().unwrap(),
            Ratio::<isize>::from_integer(target.try_into().unwrap())
        );
    }

    #[test_case(30, vec![1, 2])]
    #[test_case(3000, vec![2, 3, 5, 10])]
    #[test_case(30000, vec![2, 3, 5, 8, 10])]
    #[test_case(300000, vec![2, 3, 5, 8, 9, 10])]
    fn impossible_numbers_round(target: usize, numbers: Vec<usize>) {
        let solution = search(
            NumbersRound {
                numbers: numbers,
                target,
            },
            false,
        );

        assert!(solution.is_none(),);
    }
}
