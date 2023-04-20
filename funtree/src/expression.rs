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

    pub fn new_leaf(value: f64, leaf_type: LeafType, minus: bool) -> Self {
        Expression {
            minus,
            expr: Expr::Leaf(Leaf::new_leaf(value, leaf_type)),
        }
    }

    // Generate a random expression
    pub fn new_randomised(rng: &mut ThreadRng, max_depth: u16) -> Self {
        let minus = rng.gen_bool(0.5);

        if max_depth == 0 || rng.gen_bool(0.5) {
            // Generate a leaf node with a random value and type
            let value = rng.gen_range(-10.0..10.0);
            let leaf_type = LeafType::random(rng);

            Self::new_leaf(value, leaf_type, minus)
        } else {
            // Generate an operation node with two random child expressions and a random operation type
            let left = Self::new_randomised(rng, max_depth - 1);
            let right = Self::new_randomised(rng, max_depth - 1);

            let operation_type = OperationType::random(rng);

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
        // Change the sign of the expression
        if rng.gen_range(0.0..1.0) < mut_prob {
            self.minus = !self.minus;
        }

        // Replace the expression with a random expression
        if rng.gen_range(0.0..1.0) < mut_prob {
            self.expr = Self::new_randomised(rng, 3).expr;
        }

        // Call mutate on the child expressions
        match &mut self.expr {
            Expr::Leaf(leaf) => leaf.mutate(rng, mut_prob),
            Expr::Op(op) => op.mutate(rng, mut_prob),
        }
    }

    pub fn choose_random_node(&self, rng: &mut ThreadRng) -> &Expression {
        // Get all nodes in the expression
        let nodes = self.get_nodes();

        // Return a random node
        nodes[rng.gen_range(0..nodes.len())]
    }

    // Append all nodes in the expression to the given vector
    pub fn append_nodes<'a>(&'a self, nodes: &mut Vec<&'a Expression>) {
        nodes.push(self);
        match &self.expr {
            Expr::Leaf(_) => (),
            Expr::Op(op) => op.append_nodes(nodes),
        }
    }

    pub fn get_nodes(&self) -> Vec<&Expression> {
        let mut nodes = Vec::new();
        self.append_nodes(&mut nodes);
        nodes
    }

    pub fn copy_from(&mut self, other: &Expression) {
        self.minus = other.minus;
        self.expr = other.expr.clone();
    }
}

impl fmt::Display for Expression {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.as_string())
    }
}

impl Default for Expression {
    fn default() -> Self {
        Self::new_leaf(0.0, LeafType::Constant, false)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_nodes() {
        let leaf_1 = Expression::new_leaf(1.0, LeafType::Constant, true);
        let leaf_2 = Expression::new_leaf(2.0, LeafType::Constant, false);
        let add_1 = Expression::new_operation(leaf_1, leaf_2, OperationType::Addition, true);

        // Check the expression is correct
        assert_eq!(add_1.as_string(), "-(-1.00 + 2.00)");

        // Get all nodes in the expression
        let nodes = add_1.get_nodes();
        assert_eq!(nodes.len(), 3);
        assert_eq!(nodes[0].as_string(), "-(-1.00 + 2.00)");
        assert_eq!(nodes[1].as_string(), "-1.00");
        assert_eq!(nodes[2].as_string(), "2.00");
    }
}
