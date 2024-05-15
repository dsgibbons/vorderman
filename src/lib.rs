use num::rational::Ratio;

mod expr;

#[derive(Debug)]
struct NumbersRound {
    numbers: Vec<u32>,
    target: Ratio<u32>,
}
