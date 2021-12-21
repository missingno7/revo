use rand::Rng;
use rand::prelude::ThreadRng;
use crate::revo::evo_individual::EvoIndividual;


pub struct BasicIndividualData
{

}


pub struct BasicIndividual {
    fitness: f64,
    value: f64,
}

impl EvoIndividual<BasicIndividualData> for BasicIndividual {
    fn new() -> Self {
        BasicIndividual {
            fitness: 0.0,
            value: 0.0,
        }
    }

    fn new_randomised(ind_data: &BasicIndividualData, rng: &mut ThreadRng) -> Self {
        BasicIndividual {
            fitness: 0.0,
            value: rng.gen_range(0.0..10.0),
        }
    }

    fn copy_to(&self, ind: &mut Self) {
        ind.fitness = self.fitness;
        ind.value = self.value
    }

    fn clone(&self) -> Self {
        let mut new_ind = Self::new();
        self.copy_to(&mut new_ind);
        new_ind
    }

    fn mutate(&mut self, ind_data: &BasicIndividualData, rng: &mut ThreadRng, mut_prob: f32, mut_amount: f32) {
        self.value += rng.gen_range(-mut_amount as f64..mut_amount as f64);

        //println!("{}", self.value);


    }

    fn count_fitness(&mut self, ind_data: &BasicIndividualData) {
        self.fitness = self.value;
    }
    fn get_fitness(&self) -> f64 {
        return self.fitness;
    }
}
