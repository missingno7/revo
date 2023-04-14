extern crate evo_salesman;
extern crate revo;

use evo_salesman::salesman::SalesmanIndividual;
use evo_salesman::salesman_data::SalesmanIndividualData;
use revo::evo_individual::{EvoIndividual, Visualise};
use revo::pop_config::PopulationConfig;
use revo::population::Population;
use std::fs;

fn main() {
    let mut rng = rand::thread_rng();

    // Prepare output directory and remove old files if they exist
    let output_dir = "./out";
    let _ = fs::remove_dir_all(output_dir);
    fs::create_dir(output_dir).unwrap();

    // Load the population config and create the individual data
    let pop_config = PopulationConfig::new("pop_config.json");
    let ind_data = SalesmanIndividualData::from_config(&mut rng, &pop_config);

    // Create the population
    let mut pop: Population<SalesmanIndividual, SalesmanIndividualData> =
        Population::new(&pop_config, ind_data.clone());

    // Get the best individual
    let mut all_best_ind = pop.get_best();

    // Run the evolution
    loop {
        let best_ind = pop.get_best();

        if best_ind.get_fitness() > all_best_ind.get_fitness() {
            all_best_ind = best_ind;

            println!(
                "Round {}, best fitness: {}",
                pop.get_generation(),
                all_best_ind.get_fitness()
            );
            all_best_ind.visualise(
                format!("{}/best_{}.png", output_dir, pop.get_generation()).as_str(),
                &ind_data,
            );
        }

        if pop_config.visualise {
            pop.visualise(format!("{}/pop_{}.png", output_dir, pop.get_generation()).as_str());
        }

        // Advance to the next generation
        pop.next_gen();
    }
}
