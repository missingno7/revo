extern crate revo;

use revo::population::evo_individual::EvoIndividual;
use revo::population::population::Population;

struct BasicIndividual {
    fitness: f64,
    value: f64,
}

impl EvoIndividual for BasicIndividual {
    fn new() -> Self {
        Self::new_randomised()
    }

    fn new_randomised() -> Self {
        BasicIndividual {
            fitness: 0.0,
            value: 0.0,
        }
    }

    fn copy_to(&self, ind: &mut Self) {
        ind.fitness = self.fitness;
    }

    fn clone(&self) -> Self {
        let mut new_ind = Self::new();
        self.copy_to(&mut new_ind);
        new_ind
    }

    fn mutate(&mut self) {
        self.value += 1.0
    }

    fn count_fitness(&mut self) {
        self.fitness = 1.0;
    }
    fn get_fitness(&self) -> f64 {
        return self.fitness;
    }
}

fn main() {
    let mut pop: Population<BasicIndividual> = Population::new(4, 5);
    pop.next_gen();
}
