extern crate revo;

use revo::evo_individual::EvoIndividual;
use revo::pop_config::PopulationConfig;
use revo::population::Population;
use social_distance::social_distance::{DistanceIndividual, DistanceIndividualData};
use std::fs;

fn main() {
    let n_points: usize = 50;
    let required_distance = 20;
    let screen_width: u32 = 500;
    let screen_height: u32 = 500;

    let pop_config = PopulationConfig::new("pop_config.json");
    let output_dir = "./out";

    let ind_data =
        DistanceIndividualData::new(n_points, screen_width, screen_height, required_distance);
    let mut pop: Population<DistanceIndividual, DistanceIndividualData> =
        Population::new(&pop_config, ind_data.clone());

    fs::create_dir(output_dir).unwrap();

    let mut all_best_ind = pop.get_best();
    for _ in 0..1000000 {
        let best_ind = pop.get_best();

        if best_ind.get_fitness() > all_best_ind.get_fitness() {
            all_best_ind = best_ind.clone();
            all_best_ind.draw(
                format!("{}/best_{}.png", output_dir, pop.get_generation()).as_str(),
                &ind_data,
            );
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

        if pop_config.visualise
        {
            pop.visualise(format!("{}/pop_{}.png", output_dir, pop.get_generation()).as_str());
        }


        pop.next_gen();
    }
}
