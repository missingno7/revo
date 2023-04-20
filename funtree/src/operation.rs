use crate::expression::Expression;
use rand::prelude::SliceRandom;
use rand::rngs::ThreadRng;
use rand::Rng;
use std::fmt;
use std::mem::swap;

#[derive(Clone, Copy)]
#[repr(u8)]
pub enum OperationType {
    Addition,
    Multiplication,
    Division,
    Power,
}

impl OperationType {
    pub fn random(rng: &mut ThreadRng) -> Self {
        let operations = [
            OperationType::Addition,
            OperationType::Multiplication,
            OperationType::Division,
            OperationType::Power,
        ];
        *operations.choose(rng).unwrap()
    }
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
            OperationType::Division => self.left.evaluate(x) / self.right.evaluate(x),
            OperationType::Power => self.left.evaluate(x).powf(self.right.evaluate(x)),
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
            OperationType::Division => "/",
            OperationType::Power => "^",
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
        let b = left_b + right_b + self.operation_type as u8 as f64;

        (a, b)
    }

    pub fn mutate(&mut self, rng: &mut ThreadRng, mut_prob: f32) {
        if rng.gen_range(0.0..1.0) < mut_prob {
            self.operation_type = OperationType::random(rng);
        }

        if rng.gen_range(0.0..1.0) < mut_prob {
            swap(&mut self.left, &mut self.right);
        }

        self.left.mutate(rng, mut_prob);
        self.right.mutate(rng, mut_prob);
    }

    pub fn append_nodes<'a>(&'a self, nodes: &mut Vec<&'a Expression>) {
        self.left.append_nodes(nodes);
        self.right.append_nodes(nodes);
    }
}

impl fmt::Display for Operation {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.as_string().fmt(f)
    }
}
