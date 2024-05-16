use num::rational::Ratio;

mod expr;

#[derive(Debug)]
struct NumbersRound {
    numbers: Vec<u32>,
    target: Ratio<u32>,
}

// construct solutions
// random generate expressions for testing
// test solutions can be evaluated
// DFS for building solutions towards target
