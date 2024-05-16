mod expr;

#[derive(Debug)]
struct NumbersRound {
    numbers: Vec<u32>,
    target: u32,
}

// construct solutions
// random generate expressions for testing
// test solutions can be evaluated
// DFS for building solutions towards target
