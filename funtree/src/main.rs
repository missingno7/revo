use funtree::funtree_data::FuntreeIndividualData;
use funtree::funtree_individual::FuntreeIndividual;

use revo::config::{Config, DEFAULT_CONFIG_FILENAME};
use revo::evo_individual::{EvoIndividual, Visualise};
use revo::evo_population::EvoPopulation;
use revo::population::Population;
use std::fs;

fn main() {
    // Prepare output directory and remove old files if they exist
    let output_dir = "./out";
    let _ = fs::remove_dir_all(output_dir);
    fs::create_dir(output_dir).unwrap();

    // Load the population config and create the individual data
    let config = Config::new(DEFAULT_CONFIG_FILENAME);
    let visualise = config.get_bool("visualise").unwrap().unwrap_or(false);

    // Create the population
    let mut pop: Population<FuntreeIndividual, FuntreeIndividualData> = Population::new(&config);

    // Get the best individual
    let mut all_best_ind = pop.get_best().clone();
    println!(
        "Best individual: {}",
        all_best_ind.simplify().to_string(pop.get_individual_data())
    );

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

            println!(
                "Best individual: {}",
                all_best_ind.simplify().to_string(pop.get_individual_data())
            );
            all_best_ind
                .visualise(pop.get_individual_data())
                .save(format!("{}/best_{}.png", output_dir, pop.get_generation()))
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
