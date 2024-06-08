use rand::seq::SliceRandom;
use rand::Rng;

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct NumbersRound {
    pub numbers: Box<[usize]>,
    pub target: usize,
}

#[derive(Debug)]
pub enum NumbersRoundError {
    TooFewSmalls,
    TooManySmalls,
}

impl NumbersRound {
    pub fn new(n_small: u8) -> Result<NumbersRound, NumbersRoundError> {
        let mut rng = rand::thread_rng();

        if n_small < 2 {
            return Err(NumbersRoundError::TooFewSmalls);
        } else if n_small > 6 {
            return Err(NumbersRoundError::TooManySmalls);
        }

        let mut smalls: Vec<usize> = (1..=10).cycle().take(20).collect();
        smalls.shuffle(&mut rng);

        let mut bigs: Vec<usize> = vec![25, 50, 75, 100];
        bigs.shuffle(&mut rng);

        let target = rng.gen_range(1..=999);

        let mut numbers = Vec::new();

        for _ in 0..n_small {
            numbers.push(smalls.pop().unwrap());
        }

        if n_small < 6 {
            for _ in 0..6 - n_small {
                numbers.push(bigs.pop().unwrap());
            }
        }

        Ok(NumbersRound {
            numbers: numbers.into_boxed_slice(),
            target,
        })
    }
}
