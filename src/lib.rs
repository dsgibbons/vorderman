
struct NumbersGame {
    numbers: Vec<u32>,
    target: u32,
}

enum Operation {
    Add,
    Subtract,
    Multiply,
    Divide,
}

enum SolutionComponent {
    Number(u32),
    Operation(Operation),
    PartialSolution(Box<Expression>),
}

type Expression = Vec<SolutionComponent>;

pub fn evaluate(expression: Expression) -> f32 {

}

pub fn solve(numbers_game: NumbersGame) -> Expression {
    Expression::new()
}


pub fn add(left: usize, right: usize) -> usize {
    left + right
}

// construct solutions
// test solutions can be evaluated
// DFS for building solutions towards target

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }
}
