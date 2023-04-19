use crate::leaf::{Leaf, LeafType};
use crate::operation::{Operation, OperationType};
use std::fmt;

use rand::Rng;
// Define a math expression enum

#[derive(Clone)]
enum Expr {
    Leaf(Leaf),
    Op(Operation),
}

#[derive(Clone)]
pub struct Expression {
    minus: bool,
    expr: Expr,
}

impl Expression {
    // Evaluate the expression and return the result

    pub fn evaluate(&self, x: f64) -> f64 {
        let val = match &self.expr {
            Expr::Leaf(leaf) => leaf.evaluate(x),
            Expr::Op(op) => op.evaluate(x),
        };

        if self.minus {
            -val
        } else {
            val
        }
    }

    pub fn new_operation(
        left: Expression,
        right: Expression,
        operation_type: OperationType,
        minus: bool,
    ) -> Self {
        Expression {
            minus,
            expr: Expr::Op(Operation::new_operation(left, right, operation_type)),
        }
    }

    pub fn new_leaf(value: f64, leaf_type: LeafType) -> Self {
        Expression {
            minus: false,
            expr: Expr::Leaf(Leaf::new_leaf(value, leaf_type)),
        }
    }

    // Generate a random expression
    pub fn random(rng: &mut impl Rng) -> Self {
        if rng.gen_bool(0.5) {
            // Generate a leaf node with a random value and type
            let value = rng.gen_range(-10.0..10.0);
            let leaf_type = if rng.gen_bool(0.5) {
                LeafType::Constant
            } else {
                LeafType::Variable
            };
            Self::new_leaf(value, leaf_type)
        } else {
            // Generate an operation node with two random child expressions and a random operation type
            let left = Self::random(rng);
            let right = Self::random(rng);
            let operation_type = match rng.gen_range(0..1) {
                0 => OperationType::Addition,
                _ => OperationType::Multiplication,
            };
            let minus = rng.gen_bool(0.5);
            Self::new_operation(left, right, operation_type, minus)
        }
    }
}

impl fmt::Display for Expression {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if self.minus {
            write!(f, "-")?;
        }

        match &self.expr {
            Expr::Leaf(leaf) => write!(f, "{}", leaf),
            Expr::Op(op) => write!(f, "{}", op),
        }
    }
}
