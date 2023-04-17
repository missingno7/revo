extern crate revo;

use revo::evo_individual::{EvoIndividual, Visualise};
use revo::population::Population;
use social_distance::social_distance::{DistanceIndividual, DistanceIndividualData};

use revo::config::Config;
use std::fs;

fn main() {
    let config = Config::new("pop.json");
    let output_dir = "./out";
    let visualise = config.get_bool("visualise", Some(false)).unwrap();

    let ind_data = DistanceIndividualData::from_config(&config);
    let mut pop: Population<DistanceIndividual, DistanceIndividualData> =
        Population::new(&config, ind_data.clone());

    fs::create_dir(output_dir).unwrap();

    let mut all_best_ind = pop.get_best();
    for _ in 0..1000000 {
        let best_ind = pop.get_best();

        if best_ind.get_fitness() > all_best_ind.get_fitness() {
            all_best_ind = best_ind.clone();
            let img = all_best_ind.visualise(&ind_data);
            img.save(format!("{}/best_{}.png", output_dir, pop.get_generation()).as_str())
                .unwrap();

            println!(
                "Round {}, best fitness: {} - New record",
                pop.get_generation(),
                best_ind.get_fitness()
            );
        } else {
            println!(
                "Round {}, best fitness: {}",
                pop.get_generation(),
                best_ind.get_fitness()
            );
        }

        if visualise {
            let img = pop.visualise();
            img.save(format!("{}/pop_{:05}.png", output_dir, pop.get_generation()).as_str())
                .unwrap();
        }

        pop.next_gen();
    }
}
