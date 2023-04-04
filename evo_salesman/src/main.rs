extern crate evo_salesman;
extern crate revo;

use evo_salesman::salesman::{SalesmanIndividual};
use evo_salesman::salesman_data::{SalesmanIndividualData};
use revo::evo_individual::EvoIndividual;
use revo::pop_config::PopulationConfig;
use revo::population::Population;

fn main() {
    let mut rng = rand::thread_rng();

    let pop_config = PopulationConfig::new("pop_config.json");
    let ind_data = SalesmanIndividualData::from_config(
        &mut rng,
        &pop_config,
    );
    let mut pop: Population<SalesmanIndividual, SalesmanIndividualData> =
        Population::new(pop_config, ind_data.clone());

    let mut all_best_ind = pop.get_best();

    all_best_ind.draw(
        format!("best_{}.png", pop.get_generation()).as_str(),
        &ind_data,
    );
    println!(
        "Round {}, best fitness: {}",
        pop.get_generation(),
        all_best_ind.get_fitness()
    );

    loop {
        let best_ind = pop.get_best();

        if best_ind.get_fitness() > all_best_ind.get_fitness() {
            println!(
                "Round {}, best fitness: {}",
                pop.get_generation(),
                best_ind.get_fitness()
            );
            all_best_ind = best_ind.clone();
            all_best_ind.draw(
                format!("best_{}.png", pop.get_generation()).as_str(),
                &ind_data,
            );
        }

        pop.next_gen();
    }
}
