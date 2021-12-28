extern crate revo;
extern crate evo_salesman;

use revo::pop_config::PopulationConfig;
use revo::population::Population;
use revo::evo_individual::EvoIndividual;
use evo_salesman::salesman::{SalesmanIndividualData, SalesmanIndividual, SalesmanInitType};

fn main() {
    let n_cities: u32 = 500;
    let screen_width: u32 = 1000;
    let screen_height: u32 = 1000;
    let shift_prob: f64 = 0.4;
    let rev_prob: f64 = 0.4;

    let mut rng = rand::thread_rng();

    let pop_config = PopulationConfig::new("pop_config.json");
    let ind_data = SalesmanIndividualData::new(&mut rng, n_cities, screen_width, screen_height, shift_prob, rev_prob, SalesmanInitType::GreedyJoining);
    let mut pop: Population<SalesmanIndividual, SalesmanIndividualData> = Population::new(pop_config, ind_data.clone());

    let mut all_best_ind = pop.get_best();

    all_best_ind.draw(format!("best_{}.png", pop.get_generation()).as_str(), &ind_data);
    println!("Round {}, best fitness: {}", pop.get_generation(), all_best_ind.get_fitness());

    loop
    {
        let best_ind = pop.get_best();

        if best_ind.get_fitness() > all_best_ind.get_fitness()
        {
            println!("Round {}, best fitness: {}", pop.get_generation(), best_ind.get_fitness());
            all_best_ind = best_ind.clone();
            all_best_ind.draw(format!("best_{}.png", pop.get_generation()).as_str(), &ind_data);
        }

        pop.next_gen();
    }
}
