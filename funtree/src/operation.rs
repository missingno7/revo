use crate::expression::Expression;
use std::fmt;

#[derive(Clone)]
pub enum OperationType {
    Addition,
    Multiplication,
}

#[derive(Clone)]
pub struct Operation {
    left: Box<Expression>,
    right: Box<Expression>,
    operation_type: OperationType,
}

impl Operation {
    pub fn evaluate(&self, x: f64) -> f64 {
        match self.operation_type {
            OperationType::Addition => self.left.evaluate(x) + self.right.evaluate(x),
            OperationType::Multiplication => self.left.evaluate(x) * self.right.evaluate(x),
        }
    }

    pub fn new_operation(
        left: Expression,
        right: Expression,
        operation_type: OperationType,
    ) -> Self {
        Operation {
            left: Box::new(left),
            right: Box::new(right),
            operation_type,
        }
    }
}

impl fmt::Display for Operation {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let op_str = match self.operation_type {
            OperationType::Addition => "+",
            OperationType::Multiplication => "*",
        };
        write!(f, "({} {} {})", self.left, op_str, self.right,)
    }
}
