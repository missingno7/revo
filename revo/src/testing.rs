use crate::evo_individual::EvoIndividual;
use rand::rngs::ThreadRng;

#[derive(Clone)]
pub struct MockIndividualData {}

#[derive(Clone)]
pub struct MockIndividual {
    pub fitness: f64,
    pub visuals: (f64, f64),
    pub value: f64,
}

impl EvoIndividual<MockIndividualData> for MockIndividual {
    fn new(_ind_data: &MockIndividualData) -> Self {
        MockIndividual {
            fitness: 0.0,
            visuals: (0.0, 0.0),
            value: 0.0,
        }
    }

    fn new_randomised(_ind_data: &MockIndividualData, _rng: &mut ThreadRng) -> Self {
        MockIndividual {
            fitness: 0.0,
            visuals: (0.0, 0.0),
            value: 0.0,
        }
    }

    fn copy_to(&self, ind: &mut Self) {
        ind.value = self.value;
        ind.fitness = self.fitness;
        ind.visuals = self.visuals;
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

    fn crossover_to(
        &self,
        _another_ind: &Self,
        dest_ind: &mut Self,
        _ind_data: &MockIndividualData,
        _rng: &mut ThreadRng,
    ) {
        dest_ind.value = (self.value + dest_ind.value) / 2.0;
    }

    fn count_fitness(&mut self, _ind_data: &MockIndividualData) -> f64 {
         self.value
    }


    fn get_visuals(&self, _ind_data: &MockIndividualData) -> (f64, f64) {
        self.visuals
    }
}
