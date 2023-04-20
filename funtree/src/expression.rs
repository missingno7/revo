use crate::leaf::{Leaf, LeafType};
use crate::operation::{Operation, OperationType};
use rand::rngs::ThreadRng;
use rand::Rng;
use std::default::Default;
use std::fmt;

#[derive(Clone)]
pub enum Expr {
    Leaf(Leaf),
    Op(Operation),
}

#[derive(Clone)]
pub struct Expression {
    pub minus: bool,
    pub expr: Expr,
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
    pub fn new_randomised(rng: &mut ThreadRng, max_depth: u16) -> Self {
        if max_depth == 0 || rng.gen_bool(0.5) {
            // Generate a leaf node with a random value and type
            let value = rng.gen_range(-10.0..10.0);
            let leaf_type = LeafType::random(rng);

            Self::new_leaf(value, leaf_type)
        } else {
            // Generate an operation node with two random child expressions and a random operation type
            let left = Self::new_randomised(rng, max_depth - 1);
            let right = Self::new_randomised(rng, max_depth - 1);

            let operation_type = OperationType::random(rng);
            let minus = rng.gen_bool(0.5);
            Self::new_operation(left, right, operation_type, minus)
        }
    }

    pub fn as_string(&self) -> String {
        let mut s = String::new();

        if self.minus {
            s.push('-');
        }

        match &self.expr {
            Expr::Leaf(leaf) => s.push_str(&leaf.as_string()),
            Expr::Op(op) => s.push_str(&op.as_string()),
        }

        s
    }

    pub fn get_visuals(&self) -> (f64, f64) {
        let (mut a, mut b) = match &self.expr {
            Expr::Leaf(leaf) => leaf.get_visuals(),
            Expr::Op(op) => op.get_visuals(),
        };
        if self.minus {
            a = -a;
            b = -b;
        }
        (a, b)
    }

    pub fn mutate(&mut self, rng: &mut ThreadRng, mut_prob: f32) {
        if rng.gen_range(0.0..1.0) < mut_prob {
            self.minus = !self.minus;
        }

        if rng.gen_range(0.0..1.0) < mut_prob {
            self.expr = Self::new_randomised(rng, 3).expr;
        }

        match &mut self.expr {
            Expr::Leaf(leaf) => leaf.mutate(rng, mut_prob),
            Expr::Op(op) => op.mutate(rng, mut_prob),
        }
    }

    pub fn choose_random_node(&self, rng: &mut ThreadRng) -> &Expression {
        if rng.gen_bool(0.5) {
            return self;
        }

        match &self.expr {
            Expr::Leaf(_) => self,
            Expr::Op(op) => op.choose_random_node(rng),
        }
    }
}

impl fmt::Display for Expression {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.as_string())
    }
}

impl Default for Expression {
    fn default() -> Self {
        Self::new_leaf(0.0, LeafType::Constant)
    }
}
