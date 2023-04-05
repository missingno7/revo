extern crate rand;
extern crate revo;
extern crate rustc_serialize;

use revo::basic_individual::{BasicIndividual, BasicIndividualData};
use revo::evo_individual::EvoIndividual;
use revo::pop_config::PopulationConfig;
use revo::population::Population;

fn main() {
    let pop_config = PopulationConfig::new("pop_config.json");
    let ind_data = BasicIndividualData::default();
    let mut pop: Population<BasicIndividual, BasicIndividualData> =
        Population::new(pop_config, ind_data);

    for _ in 0..10 {
        let pop_best = pop.get_best();

        println!(
            "Round {}, best fitness: {}",
            pop.get_generation(),
            pop_best.get_fitness()
        );

        if pop_config.visualise {
            pop.visualise(format!("pop_{}.png", pop.get_generation()).as_str());
        }

        pop.next_gen();
    }
}
