use super::expr::{Expression, Fix, FixExpression, Operation, Token};
use num::rational::Ratio;
use std::collections::{BinaryHeap, HashSet, VecDeque};
use std::iter::{repeat, zip};

struct NumbersRound {
    numbers: HashSet<usize>,
    target: usize,
}

#[derive(Clone, Debug)]
struct PartialSolution {
    expression: Expression,
    stack: Vec<Ratio<usize>>,
    remaining: HashSet<usize>,
}

impl PartialSolution {
    fn new(numbers: &HashSet<usize>) -> PartialSolution {
        let mut tokens = Vec::<Token>::new();
        let mut stack = Vec::<Ratio<usize>>::new();
        PartialSolution {
            expression: Expression(tokens),
            stack,
            remaining: numbers.clone(),
        }
    }
}

#[derive(PartialEq, Eq, Debug)]
struct Item<T: PartialEq> {
    priority: usize,
    value: T,
}

impl<T: Eq> PartialOrd for Item<T> {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl<T: Eq> Ord for Item<T> {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.priority.cmp(&other.priority)
    }
}

trait PriorityQueue {
    type V: Eq;

    fn insert(&mut self, item: Item<Self::V>) -> ();

    fn pop(&mut self) -> Item<Self::V>;

    fn peek(&self) -> &Item<Self::V>;
}

type Stack<T> = Vec<Item<T>>;
type Queue<T> = VecDeque<Item<T>>;

impl<T: Eq> PriorityQueue for Stack<T> {
    type V = T;

    fn insert(&mut self, item: Item<Self::V>) -> () {
        self.push(item);
    }

    fn pop(&mut self) -> Item<Self::V> {
        self.pop().unwrap()
    }

    fn peek(&self) -> &Item<Self::V> {
        self.last().unwrap()
    }
}

impl<T: Eq> PriorityQueue for Queue<T> {
    type V = T;

    fn insert(&mut self, item: Item<Self::V>) -> () {
        self.push_front(item);
    }

    fn pop(&mut self) -> Item<Self::V> {
        self.pop_back().unwrap()
    }

    fn peek(&self) -> &Item<Self::V> {
        self.back().unwrap()
    }
}

impl<T: Eq> PriorityQueue for BinaryHeap<Item<T>> {
    type V = T;

    fn insert(&mut self, item: Item<Self::V>) -> () {
        self.push(item);
    }

    fn pop(&mut self) -> Item<Self::V> {
        self.pop().unwrap()
    }

    fn peek(&self) -> &Item<Self::V> {
        self.peek().unwrap()
    }
}

const OPERATIONS: [Operation; 4] = [
    Operation::Add,
    Operation::Subtract,
    Operation::Multiply,
    Operation::Divide,
];

fn branch(partial: PartialSolution, target: usize) -> Option<Vec<PartialSolution>> {
    if partial.remaining.len() == 0 {
        return None;
    }

    let mut options = Vec::<Token>::new();

    for r in partial.remaining.iter() {
        options.push(Token::Number(*r));
    }

    if partial.expression.0.len() > 2 {
        for op in OPERATIONS.iter() {
            options.push(Token::Operation(*op))
        }
    }

    let mut new_partials: Vec<_> = repeat(partial).take(options.len()).collect();

    // TODO: stack compute and update remaining before pushing partial solution
    for (p, o) in zip(new_partials, options) {
        match o {
            Token::Number(n) => new_partials,
        }

        new_partials.push(o);
    }

    Some(new_partials)
}

fn search(config: NumbersRound) -> Option<FixExpression> {
    let target = Ratio::<usize>::from_integer(config.target.try_into().unwrap());
    let partial_solution = PartialSolution::new(&config.numbers);

    while let Some(partial_solutions) = branch(partial_solution, config.target) {
        for partial in partial_solutions.iter() {
            if partial.stack.len() == 1 && *partial.stack.first().unwrap() == target {
                return Some(FixExpression {
                    expression: partial.expression,
                    fix: Fix::Post,
                });
            }
        }
    }

    None
}
