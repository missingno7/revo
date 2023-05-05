use crate::config::Config;
use crate::evo_individual::{EvoIndividual, EvoIndividualData};
use rand::rngs::ThreadRng;

#[derive(Clone)]
pub struct MockIndividualData {}

impl EvoIndividualData for MockIndividualData {
    fn from_config(_config: &Config) -> Self {
        MockIndividualData {}
    }
}

#[derive(Clone)]
pub struct MockIndividual {
    pub visuals: (f64, f64),
    pub value: f64,
}

impl EvoIndividual<MockIndividualData> for MockIndividual {
    fn new_randomised(_ind_data: &MockIndividualData, _rng: &mut ThreadRng) -> Self {
        MockIndividual {
            visuals: (0.0, 0.0),
            value: 0.0,
        }
    }

    fn mutate(
        &mut self,
        _ind_data: &MockIndividualData,
        _rng: &mut ThreadRng,
        _mut_prob: f32,
        _mut_amount: f32,
    ) {
        self.value += 1.0;
    }

    fn crossover(
        &self,
        another_ind: &Self,
        _ind_data: &MockIndividualData,
        _rng: &mut ThreadRng,
    ) -> MockIndividual {
        MockIndividual {
            visuals: (0.0, 0.0),
            value: (self.value + another_ind.value) / 2.0,
        }
    }

    fn count_fitness(&self, _ind_data: &MockIndividualData) -> f64 {
        self.value
    }

    fn get_visuals(&self, _ind_data: &MockIndividualData) -> (f64, f64) {
        self.visuals
    }
}
