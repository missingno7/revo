use std::fmt;

#[derive(Clone)]
pub enum LeafType {
    Constant,
    Variable,
}

#[derive(Clone)]
pub struct Leaf {
    leaf_type: LeafType,
    value: f64,
}

impl Leaf {
    pub fn evaluate(&self, x: f64) -> f64 {
        match self.leaf_type {
            LeafType::Constant => self.value,
            LeafType::Variable => x,
        }
    }

    pub fn new_leaf(value: f64, leaf_type: LeafType) -> Self {
        Leaf { leaf_type, value }
    }
}

impl fmt::Display for Leaf {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self.leaf_type {
            LeafType::Constant => write!(f, "{:.2}", self.value),
            LeafType::Variable => write!(f, "x"),
        }
    }
}
