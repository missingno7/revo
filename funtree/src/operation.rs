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

    pub fn as_string(&self) -> String {
        let op_str = match self.operation_type {
            OperationType::Addition => "+",
            OperationType::Multiplication => "*",
        };
        format!(
            "({} {} {})",
            self.left.as_string(),
            op_str,
            self.right.as_string()
        )
    }

    pub fn get_visuals(&self) -> (f64, f64) {
        let (left_a, left_b) = self.left.get_visuals();
        let (right_a, right_b) = self.right.get_visuals();

        let a = left_a + right_a;
        let b = left_b + right_b + self.operation_type.clone() as u32 as f64;

        (a, b)
    }
}

impl fmt::Display for Operation {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.as_string().fmt(f)
    }
}
