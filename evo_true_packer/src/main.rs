extern crate evo_true_packer;
extern crate revo;

use evo_true_packer::packer::PackerIndividual;
use evo_true_packer::packer_data::PackerIndividualData;
use revo::config::{Config, DEFAULT_CONFIG_FILENAME};
use revo::evo_individual::{EvoIndividual, Visualise};
use revo::population::Population;
use std::fs;

fn main() {
    // Prepare output directory and remove old files
    let output_dir = "./out";
    let _ = fs::remove_dir_all(output_dir);
    fs::create_dir(output_dir).unwrap();

    let config = Config::new(DEFAULT_CONFIG_FILENAME);
    let visualise = config.may_get_bool("visualise").unwrap().unwrap_or(false);

    let mut pop: Population<PackerIndividual, PackerIndividualData> = Population::new(&config);

    let mut all_best_ind = pop.get_at(0, 0).clone();

    loop {
        let best_ind = pop.get_best();

        if best_ind.get_fitness() > all_best_ind.get_fitness() {
            all_best_ind = best_ind.clone();

            // Compute density of this best individual
            let (placements, w, h) = all_best_ind.compute_layout(pop.get_individual_data());
            let density = PackerIndividual::compute_density(&placements, w, h);

            println!("Round {}, density: {:.2}%", pop.get_generation(), density);

            // Save image of best solution
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

        // Next generation
        pop.next_gen();
    }
}
