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

    pub fn as_string(&self) -> String {
        match self.leaf_type {
            LeafType::Constant => format!("{:.2}", self.value),
            LeafType::Variable => "x".to_string(),
        }
    }

    pub fn get_visuals(&self) -> (f64, f64) {
        let a = self.value;
        let b = self.leaf_type.clone() as u32 as f64;

        (a, b)
    }
}

impl fmt::Display for Leaf {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.as_string().fmt(f)
    }
}
