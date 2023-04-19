use crate::expression::Expression;
use revo::evo_individual::EvoIndividual;

#[derive(Clone)]
pub struct FuntreeIndividual {
    pub fitness: f64,
    genom: Expression,
}

impl FuntreeIndividual {}
