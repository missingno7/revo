use funtree::funtree_data::FuntreeIndividualData;
use funtree::funtree_individual::FuntreeIndividual;

use revo::config::Config;
use revo::evo_individual::EvoIndividual;
use revo::population::Population;
use std::fs;

fn main() {
    // Prepare output directory and remove old files if they exist
    let output_dir = "./out";
    let _ = fs::remove_dir_all(output_dir);
    fs::create_dir(output_dir).unwrap();

    // Load the population config and create the individual data
    let config = Config::new("config.json");
    let ind_data = FuntreeIndividualData::from_config(&config);
    let visualise = config.get_bool("visualise", Some(false)).unwrap();

    // Create the population
    let mut pop: Population<FuntreeIndividual, FuntreeIndividualData> =
        Population::new(&config, ind_data.clone());

    // Get the best individual
    let mut all_best_ind = pop.get_best().clone();
    println!("Best individual: {}", all_best_ind.as_string(&ind_data));

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

            println!("Best individual: {}", all_best_ind.as_string(&ind_data));
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
