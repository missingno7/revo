extern crate revo;
extern crate rand;
extern crate rustc_serialize;

use revo::evo_individual::EvoIndividual;
use revo::population::Population;
use revo::basic_individual::{BasicIndividual, BasicIndividualData};
use revo::pop_config::PopulationConfig;



fn main() {

    let pop_config = PopulationConfig::new("pop_config.json");
    let ind_data = BasicIndividualData::new();
    let mut pop: Population<BasicIndividual, BasicIndividualData> = Population::new(pop_config, ind_data);

    for _ in 0..10
    {
    println!("Round {}, best fitness: {}",pop.get_generation(), pop.get_best().get_fitness());
    pop.next_gen();
    }

}
