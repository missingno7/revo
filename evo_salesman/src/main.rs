extern crate evo_salesman;
extern crate revo;

use evo_salesman::salesman::SalesmanIndividual;
use evo_salesman::salesman_data::SalesmanIndividualData;
use revo::config::{Config, DEFAULT_CONFIG_FILENAME};
use revo::evo_individual::{EvoIndividual, Visualise};
use revo::population::Population;
use std::fs;

fn main() {
    // Prepare output directory and remove old files if they exist
    let output_dir = "./out";
    let _ = fs::remove_dir_all(output_dir);
    fs::create_dir(output_dir).unwrap();

    // Load the population config and create the individual data
    let config = Config::new(DEFAULT_CONFIG_FILENAME);
    let visualise = config.may_get_bool("visualise").unwrap().unwrap_or(false);

    // Create the population
    let mut pop: Population<SalesmanIndividual, SalesmanIndividualData> = Population::new(&config);

    // Get the best individual
    let mut all_best_ind = pop.get_best().clone();

    // Run the evolution
    loop {
        let best_ind = pop.get_best();

        if best_ind.get_fitness() > all_best_ind.get_fitness() {
            all_best_ind = best_ind.clone();

            println!(
                "Round {}, best fitness: {}",
                pop.get_generation(),
                all_best_ind.get_fitness()
            );
            let image = all_best_ind.visualise(pop.get_individual_data());
            image
                .save(format!("{}/best_{}.png", output_dir, pop.get_generation()).as_str())
                .unwrap();
        }

        if visualise {
            let image = pop.visualise();
            image
                .save(format!("{}/pop_{}.png", output_dir, pop.get_generation()))
                .unwrap();
        }

        // Advance to the next generation
        pop.next_gen();
    }
}
