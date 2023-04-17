extern crate rand;
extern crate revo;

use example::basic_individual::{BasicIndividual, BasicIndividualData};
use revo::config::Config;
use revo::evo_individual::EvoIndividual;
use revo::population::Population;

fn main() {
    let config_path = "config.json";
    let num_rounds = 30;

    let config = Config::new(config_path);
    let ind_data = BasicIndividualData::default();
    let mut pop: Population<BasicIndividual, BasicIndividualData> =
        Population::new(&config, ind_data);

    for _ in 0..num_rounds {
        let pop_best = pop.get_best();

        println!(
            "Round {}, best fitness: {}",
            pop.get_generation(),
            pop_best.get_fitness()
        );

        if config.visualise {
            pop.visualise(format!("pop_{}.png", pop.get_generation()).as_str());
        }

        pop.next_gen();
    }
}
