extern crate rand;
extern crate revo;

use example::basic_individual::{BasicIndividual, BasicIndividualData};
use revo::pop_config::PopulationConfig;
use revo::population::Population;
use revo::evo_individual::EvoIndividual;

fn main() {
    let pop_config = PopulationConfig::new("pop_config.json");
    let ind_data = BasicIndividualData::default();
    let mut pop: Population<BasicIndividual, BasicIndividualData> =
        Population::new(&pop_config, ind_data);

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
