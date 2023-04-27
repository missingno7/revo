use rand::{rngs::ThreadRng, Rng};
use rand_distr::{Distribution, Normal};

#[derive(Clone, Copy, PartialEq, Eq)]
#[repr(u8)]
pub enum LeafType {
    Constant,
    Variable,
}

impl LeafType {
    pub fn random(rng: &mut ThreadRng) -> Self {
        if rng.gen_bool(0.5) {
            LeafType::Constant
        } else {
            LeafType::Variable
        }
    }
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

    pub fn to_string(&self, minus: bool) -> String {
        match self.leaf_type {
            LeafType::Constant => format!("{:.2}", if minus { -self.value } else { self.value }),
            LeafType::Variable => {
                if minus {
                    "-x".to_string()
                } else {
                    "x".to_string()
                }
            }
        }
    }

    pub fn get_constant(&self) -> Result<f64, String> {
        if self.leaf_type == LeafType::Constant {
            Ok(self.value)
        } else {
            Err("Not a constant".to_string())
        }
    }

    pub fn get_visuals(&self) -> (f64, f64) {
        let a = self.value;
        let b = self.leaf_type as u8 as f64;

        (a, b)
    }

    pub fn mutate(&mut self, rng: &mut ThreadRng, mut_prob: f32, mut_amount: f32) {
        if rng.gen_range(0.0..1.0) < mut_prob {
            self.leaf_type = LeafType::random(rng);
        }

        if rng.gen_range(0.0..1.0) < mut_prob {
            let normal = Normal::new(0.0, mut_amount as f64).unwrap();
            self.value += normal.sample(rng);
        }
    }

    pub fn get_leaf_type(&self) -> LeafType {
        self.leaf_type
    }

    pub fn is_constant(&self) -> bool {
        self.leaf_type == LeafType::Constant
    }

    pub fn is_variable(&self) -> bool {
        self.leaf_type == LeafType::Variable
    }
}
