use crate::leaf::{Leaf, LeafType};
use crate::operation::{Operation, OperationType};
use rand::rngs::ThreadRng;
use rand::Rng;
use rand_distr::{Distribution, Normal};
use std::default::Default;
use std::mem::swap;
use std::str::FromStr;

#[derive(Clone)]
pub enum Expr {
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

    pub fn new_leaf(value: f64, leaf_type: LeafType, minus: bool) -> Self {
        Expression {
            minus,
            expr: Expr::Leaf(Leaf::new_leaf(value, leaf_type)),
        }
    }

    pub fn new_constant(value: f64) -> Self {
        Self::new_leaf(value, LeafType::Constant, false)
    }

    pub fn new_variable(minus: bool) -> Self {
        Self::new_leaf(0.0, LeafType::Variable, minus)
    }

    // Generate a random expression
    pub fn new_randomised(rng: &mut ThreadRng, max_depth: u16) -> Self {
        let minus = rng.gen_bool(0.5);

        if max_depth == 0 || rng.gen_bool(0.5) {
            // Generate a leaf node with a random value and type
            let normal = Normal::new(0.0, 1.0).unwrap();
            let value = normal.sample(rng);

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

    pub fn mutate(&mut self, rng: &mut ThreadRng, mut_prob: f32, mut_amount: f32) {
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
            Expr::Leaf(leaf) => leaf.mutate(rng, mut_prob, mut_amount),
            Expr::Op(op) => op.mutate(rng, mut_prob, mut_amount),
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

    pub fn copy_to(&self, other: &mut Expression) {
        other.minus = self.minus;
        other.expr = self.expr.clone();
    }

    pub fn is_constant(&self) -> bool {
        match &self.expr {
            Expr::Leaf(leaf) => leaf.is_constant(),
            Expr::Op(_) => false,
        }
    }

    pub fn is_variable(&self) -> bool {
        match &self.expr {
            Expr::Leaf(leaf) => leaf.is_variable(),
            Expr::Op(_) => false,
        }
    }

    pub fn is_operation(&self) -> bool {
        match &self.expr {
            Expr::Leaf(_) => false,
            Expr::Op(_) => true,
        }
    }

    pub fn is_leaf(&self) -> bool {
        match &self.expr {
            Expr::Leaf(_) => true,
            Expr::Op(_) => false,
        }
    }

    pub fn simplify(&self) -> Expression {
        // ?? means that it can be operation or leaf
        // ? means it's a leaf (constant or variable)
        // _ means it's any operation type
        // +* means it's an addition or multiplication

        // ??
        match &self.expr {
            // If the expression is a leaf, it is already simplified
            // ? => ?
            Expr::Leaf(_) => {
                // Expression is already simplified
                self.clone()
            }
            // (?? _ ??)
            Expr::Op(op) => {
                // Simplify the children
                let mut left = op.get_left().simplify();
                let mut right = op.get_right().simplify();

                // (? _ ?)
                if left.is_leaf() && right.is_leaf() {
                    // a _ b => constant
                    if left.is_constant() && right.is_constant() {
                        // Both children are constants, make a new constant expression
                        return Expression::new_leaf(
                            op.evaluate(0.0),
                            LeafType::Constant,
                            self.minus,
                        );
                    }

                    // x + x
                    if left.is_variable() && right.is_variable() && op.is_addition() {
                        // x + -x = -x + x => 0
                        if left.minus != right.minus {
                            return Self::new_constant(0.0);
                        }
                        // -x + -x => -2*x
                        else if left.minus && right.minus {
                            return Self::new_operation(
                                Self::new_constant(-2.0),
                                Self::new_variable(false),
                                OperationType::Multiplication,
                                self.minus,
                            );
                        }
                        // x + x => 2*x
                        else {
                            return Self::new_operation(
                                Self::new_constant(2.0),
                                Self::new_variable(false),
                                OperationType::Multiplication,
                                self.minus,
                            );
                        }
                    }

                    // x * x
                    if left.is_variable() && right.is_variable() && op.is_power() {
                        // x * -x = -x * x => -x^2
                        if left.minus != right.minus {
                            return Self::new_operation(
                                Self::new_variable(true),
                                Self::new_constant(2.0),
                                OperationType::Power,
                                self.minus,
                            );
                        }
                        // -x * -x = x * x => x^2
                        else {
                            return Self::new_operation(
                                Self::new_variable(false),
                                Self::new_constant(2.0),
                                OperationType::Power,
                                self.minus,
                            );
                        }
                    }

                    // x / x
                    if left.is_variable() && right.is_variable() && op.is_division() {
                        // x / -x = -x / x => -1
                        if left.minus != right.minus {
                            return Self::new_constant(-1.0);
                        }
                        // -x / -x = x / x => 1
                        else {
                            return Self::new_constant(1.0);
                        }
                    }

                    // Constant always on the left in case of commutative operations
                    if left.is_variable() && right.is_constant() && op.is_commutative() {
                        return Expression::new_operation(
                            right,
                            left,
                            op.get_operation_type(),
                            self.minus,
                        );
                    }

                    // Cannot simplify
                    return Expression::new_operation(
                        left,
                        right,
                        op.get_operation_type(),
                        self.minus,
                    );
                }

                // Optimization for commutative operations
                // ?? +* ??
                if op.is_commutative() {
                    // Normalize that operation is always on the left side in case of commutative operations
                    if left.is_leaf() && right.is_operation() {
                        swap(&mut left, &mut right);
                    }

                    // (? _ ?) +* ??
                    if let Expr::Op(ref left_op) = left.expr {
                        let _left_left = left_op.get_left();
                        let _left_right = left_op.get_right();

                        // (? _ ?) +* (? _ ?)
                        if let Expr::Op(ref _right_op) = right.expr {
                        }
                        // (? _ ?) +* ?
                        else {
                        }
                    }
                    // ? _ ?
                }

                // Unhandled cases
                Expression::new_operation(left, right, op.get_operation_type(), self.minus)
            }
        }
    }

    /// # Safety
    ///
    /// This function dereferences a raw pointer obtained by casting `self` as a `*const`
    /// pointer and then casting it to a `*mut` pointer. Calling this function with an invalid
    /// `self` pointer or an already mutably borrowed reference can result in undefined behavior.
    ///
    /// The caller must ensure that there are no other mutable references to the same data,
    /// otherwise this function can violate Rust's aliasing rules.
    #[allow(clippy::mut_from_ref)]
    pub unsafe fn as_mut(&self) -> &mut Expression {
        #[allow(clippy::cast_ref_to_mut)]
        &mut *(self as *const _ as *mut _)
    }
}

impl Default for Expression {
    fn default() -> Self {
        Self::new_leaf(0.0, LeafType::Constant, false)
    }
}

impl ToString for Expression {
    fn to_string(&self) -> String {
        let mut s = String::new();

        match &self.expr {
            Expr::Leaf(leaf) => s.push_str(&leaf.to_string(self.minus)),
            Expr::Op(op) => {
                if self.minus {
                    s.push('-');
                }

                s.push_str(&op.to_string())
            }
        }

        s
    }
}

impl FromStr for Expression {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        // Remove all whitespaces
        let s = s.chars().filter(|c| !c.is_whitespace()).collect::<String>();

        //println!("parsing: {}",s);

        // Most inner
        if !s.contains('(') && !s.contains(')') {
            if let Some(op) = s.find(|c| c == '+' || c == '/' || c == '^' || c == '*') {
                let left_expr = Self::from_str(&s[..op])?;
                let right_expr = Self::from_str(&s[op + 1..])?;
                let operation_type = OperationType::from_str(&s[op..op + 1])?;

                return Ok(Expression::new_operation(
                    left_expr,
                    right_expr,
                    operation_type,
                    false,
                ));
            } else if s == "x" {
                return Ok(Expression::new_variable(false));
            } else if s == "-x" {
                return Ok(Expression::new_variable(true));
            } else {
                return Ok(Expression::new_constant(s.parse::<f64>().unwrap()));
            }
        }

        let left_start = s.find('(').unwrap_or(0);

        let s_left = &s[left_start + 1..];

        let mut depth = 0;
        let mut left_end = left_start;
        for (i, ch) in s_left.chars().enumerate() {
            if ch == ')' && depth == 0 {
                left_end = i;
                break;
            }

            if ch == '(' {
                depth += 1;
            }
            if ch == ')' {
                depth -= 1;
            }
        }

        let s_left = &s_left[..left_end];

        let mut left_expr = Self::from_str(s_left)?;
        if left_start > 0 && &s[left_start - 1..left_start] == "-" {
            left_expr.minus = true;
        }

        // left is actually right - left part is missing () marks
        if let Some(op_mark_i) =
            s[..left_start].find(|c| c == '+' || c == '/' || c == '^' || c == '*')
        {
            let real_left_expr = Self::from_str(&s[..op_mark_i])?;

            let op_type = OperationType::from_str(&s[op_mark_i..op_mark_i + 1])?;

            return Ok(Expression::new_operation(
                real_left_expr,
                left_expr,
                op_type,
                false,
            ));
        }

        let s_remaining = &s[left_end + 2..];

        let op_type: Option<OperationType> =
            match s_remaining.find(|c| c == '+' || c == '/' || c == '^' || c == '*') {
                None => None,
                Some(i) => Some(OperationType::from_str(&s_remaining[i..i + 1])?),
            };

        let mut right_start = s_remaining.find('(').unwrap_or(1);

        if let Some(op_mark_i) = s_remaining.find(|c| c == '+' || c == '/' || c == '^' || c == '*')
        {
            right_start = op_mark_i;
        }

        if right_start + 1 > s_remaining.len() {
            return Ok(left_expr);
        }

        let s_right = &s_remaining[right_start..];
        let mut depth = 0;
        let mut right_end = s_right.len();
        for (i, ch) in s_right.chars().enumerate() {
            if ch == ')' && depth == 0 {
                right_end = i;
                break;
            }

            if ch == '(' {
                depth += 1;
            }
            if ch == ')' {
                depth -= 1;
            }
        }

        let s_right = &s_right[1..right_end];

        let mut right_expr = Self::from_str(s_right)?;
        if right_start > 0 && &s_remaining[right_start - 1..right_start] == "-" {
            right_expr.minus = true;
        }

        if let Some(op_type) = op_type {
            return Ok(Expression::new_operation(
                left_expr, right_expr, op_type, false,
            ));
        }

        Ok(right_expr)
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
        assert_eq!(add_1.to_string(), "-(-1.00 + 2.00)");

        // Get all nodes in the expression
        let nodes = add_1.get_nodes();
        assert_eq!(nodes.len(), 3);
        assert_eq!(nodes[0].to_string(), "-(-1.00 + 2.00)");
        assert_eq!(nodes[1].to_string(), "-1.00");
        assert_eq!(nodes[2].to_string(), "2.00");
    }

    #[test]
    fn test_simplify_constants() {
        let leaf_1 = Expression::new_leaf(1.0, LeafType::Constant, true);
        let leaf_2 = Expression::new_leaf(2.0, LeafType::Constant, false);
        // -(-1.00 + 2.00)
        let add_1 = Expression::new_operation(leaf_1, leaf_2, OperationType::Addition, true);

        let leaf_3 = Expression::new_leaf(3.0, LeafType::Constant, false);
        // -(-(-1.00 + 2.00) * 3.00)
        let multiply_1 =
            Expression::new_operation(add_1, leaf_3, OperationType::Multiplication, true);

        let leaf_4 = Expression::new_leaf(4.0, LeafType::Constant, true);
        // -(-(-(-1.00 + 2.00) * 3.00) + -4.00)
        let add_2 = Expression::new_operation(multiply_1, leaf_4, OperationType::Addition, true);

        // Check the expression is correct
        assert_eq!(add_2.to_string(), "-(-(-(-1.00 + 2.00) * 3.00) + -4.00)");

        // Simplify the expression
        add_2.simplify();

        // Check the simplified expression is correct
        assert_eq!(add_2.simplify().to_string(), "1.00");

        // Add *x to expression
        let leaf_5 = Expression::new_leaf(0.0, LeafType::Variable, true);
        let multiply_2 =
            Expression::new_operation(leaf_5, add_2, OperationType::Multiplication, true);

        // Check the expression is correct
        assert_eq!(
            multiply_2.to_string(),
            "-(-x * -(-(-(-1.00 + 2.00) * 3.00) + -4.00))"
        );

        // Check the simplified expression is correct
        assert_eq!(multiply_2.simplify().to_string(), "-(1.00 * -x)");
    }

    #[test]
    fn from_string() {
        // Testing some edge cases

        let original_string = "(-(x ^ -0.13) * x)";
        let new_string = Expression::from_str(original_string).unwrap().to_string();
        assert_eq!(original_string, new_string);

        let original_string = "((2.20 + 0.13) / x)";
        let new_string = Expression::from_str(original_string).unwrap().to_string();
        assert_eq!(original_string, new_string);

        // Test with 10000 random expressions
        let mut rng = rand::thread_rng();
        for _ in 0..10000 {
            let exp = Expression::new_randomised(&mut rng, 5);

            let original_string = exp.to_string();
            println!("Testing {}", original_string);

            let new_string = Expression::from_str(original_string.as_str())
                .unwrap()
                .to_string();

            assert_eq!(original_string, new_string);
        }
    }
}
