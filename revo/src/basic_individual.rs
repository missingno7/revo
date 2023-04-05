use crate::evo_individual::EvoIndividual;
use rand::prelude::ThreadRng;
use rand::Rng;

pub struct BasicIndividualData {
    value: f64,
}

impl Default for BasicIndividualData {
    fn default() -> Self {
        BasicIndividualData { value: 0.0 }
    }
}

pub struct BasicIndividual {
    fitness: f64,
    value: f64,
}

impl EvoIndividual<BasicIndividualData> for BasicIndividual {
    fn new(ind_data: &BasicIndividualData) -> Self {
        BasicIndividual {
            fitness: 0.0,
            value: ind_data.value,
        }
    }

    fn new_randomised(ind_data: &BasicIndividualData, rng: &mut ThreadRng) -> Self {
        BasicIndividual {
            fitness: 0.0,
            value: ind_data.value + rng.gen_range(0.0..10.0),
        }
    }

    fn copy_to(&self, ind: &mut Self) {
        ind.fitness = self.fitness;
        ind.value = self.value
    }

    fn clone(&self) -> Self {
        BasicIndividual {
            fitness: self.fitness,
            value: self.value,
        }
    }

    fn mutate(
        &mut self,
        _ind_data: &BasicIndividualData,
        rng: &mut ThreadRng,
        mut_prob: f32,
        mut_amount: f32,
    ) {
        if rng.gen_range(0.0..1.0) < mut_prob {
            self.value += rng.gen_range(-mut_amount as f64..mut_amount as f64);
        }
    }

    fn crossover_to(
        &self,
        another_ind: &BasicIndividual,
        dest_int: &mut BasicIndividual,
        _ind_data: &BasicIndividualData,
        rng: &mut ThreadRng,
    ) {
        let ratio = rng.gen_range(0.0..1.0);

        dest_int.value = self.value * ratio + another_ind.value * (1.0 - ratio);
    }

    fn count_fitness(&mut self, _ind_data: &BasicIndividualData) {
        self.fitness = self.value;
    }
    fn get_fitness(&self) -> f64 {
        self.fitness
    }

    fn get_visuals(&self, _ind_data: &BasicIndividualData) -> (f64, f64) {
        (self.value, self.value)
    }
}
