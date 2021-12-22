extern crate revo;
extern crate evo_salesman;

use revo::pop_config::PopulationConfig;
use revo::population::Population;
use revo::evo_individual::EvoIndividual;
use evo_salesman::salesman::{SalesmanIndividualData, SalesmanIndividual};

fn main() {

    let n_cities: u32 = 20;
    let screen_width: u32 = 512;
    let screen_height: u32 = 512;
    let shift_prob: f64 = 0.1;
    let rev_prob: f64 = 0.1;

    let mut rng = rand::thread_rng();

    let pop_config = PopulationConfig::new("pop_config.json");
    let ind_data = SalesmanIndividualData::new(&mut rng, n_cities, screen_width, screen_height,shift_prob,rev_prob);
    let mut pop: Population<SalesmanIndividual, SalesmanIndividualData> = Population::new(pop_config, ind_data);

    for _ in 0..100
    {
        println!("Round {}, best fitness: {}",pop.get_generation(), pop.get_best().get_fitness());
        pop.next_gen();
    }


}
