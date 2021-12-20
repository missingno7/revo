extern crate revo;
extern crate rand;

use revo::population::evo_individual::EvoIndividual;
use revo::population::population::Population;
use rand::Rng;
use rand::prelude::ThreadRng;

struct BasicIndividualData
{

}


struct BasicIndividual {
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

    fn mutate(&mut self, ind_data: &BasicIndividualData, rng: &mut ThreadRng) {
        self.value += rng.gen_range(-1.0..1.0);

        //println!("{}", self.value);


    }

    fn count_fitness(&mut self, ind_data: &BasicIndividualData) {
        self.fitness = self.value;
    }
    fn get_fitness(&self) -> f64 {
        return self.fitness;
    }
}

fn main() {

    let ind_data = BasicIndividualData{};
    let mut pop: Population<BasicIndividual, BasicIndividualData> = Population::new(1000, 1000, ind_data);

    for _ in 0..1000
    {
    println!("Round {}, best fitness: {}",pop.get_generation(), pop.get_best().get_fitness());
    pop.next_gen();
    }

}
