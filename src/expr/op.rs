use std::fmt;

#[derive(Debug)]
pub enum Operand<T> {
    Number(u32),
    Expression(Box<T>),
}

impl<T: fmt::Display> fmt::Display for Operand<T> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Operand::Number(n) => write!(f, "{}", n),
            Operand::Expression(e) => write!(f, "({})", *e),
        }
    }
}

#[derive(Debug)]
pub enum Operation {
    Add,
    Subtract,
    Multiply,
    Divide,
}

#[derive(Debug)]
pub enum OperationError {
    InvalidOperation(char),
}

impl TryFrom<char> for Operation {
    type Error = OperationError;

    fn try_from(input: char) -> Result<Operation, Self::Error> {
        match input {
            '+' => Ok(Operation::Add),
            '-' => Ok(Operation::Subtract),
            '*' | 'x' => Ok(Operation::Multiply),
            '/' => Ok(Operation::Divide),
            c => Err(OperationError::InvalidOperation(c)),
        }
    }
}

impl fmt::Display for Operation {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let character = match self {
            Operation::Add => "+",
            Operation::Subtract => "-",
            Operation::Multiply => "*",
            Operation::Divide => "/",
        };
        write!(f, "{}", character)
    }
}
