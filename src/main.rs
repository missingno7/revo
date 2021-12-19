extern crate revo;

use revo::population::population::{Population};
use revo::population::evo_individual::EvoIndividual;

struct BasicIndividual
{
    fitness: f64
}

impl EvoIndividual for BasicIndividual
{
    fn new() -> Self {
        Self::new_randomised()
    }

    fn new_randomised() -> Self {
        BasicIndividual
        {
            fitness: 0.0
        }
    }

    fn copy_to(&self, ind: &mut Self)
    {
        ind.fitness = self.fitness;
    }

    fn clone(&self) -> Self {
        let mut new_ind = Self::new();
        self.copy_to(&mut new_ind);
        new_ind
    }


    fn count_fitness(&mut self) {
        self.fitness = 1.0;
    }

    fn get_fitness(&self) -> f64 {
        return self.fitness;
    }
}


fn main() {


    let mut pop:Population<BasicIndividual> = Population::new(5, 5);

    println!("{}",  pop.get_best().get_fitness());
    pop.next_gen();

    println!("{}",  pop.get_best().get_fitness());


}
