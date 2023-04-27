use example::basic_individual::{BasicIndividual, BasicIndividualData};
use revo::config::{Config, DEFAULT_CONFIG_FILENAME};
use revo::evo_individual::Visualise;
use revo::population::Population;

fn main() {
    // Load the configuration from the config.json5 file
    let config = Config::new(DEFAULT_CONFIG_FILENAME);
    let output_dir = "./";

    // Create the population
    let mut pop: Population<BasicIndividual, BasicIndividualData> = Population::new(&config);

    // Evolve the population
    // This will apply the evolution rules to the population and create a new generation
    // You can run next_gen() in a loop to evolve the population
    pop.next_gen();

    // Get the best individual from the population
    let pop_best = pop.get_best();

    // Get visualise flag from the configuration
    let visualise: bool = config.may_get_bool("visualise").unwrap().unwrap_or(false);

    // Visualize the population
    if visualise {
        pop.visualise()
            .save(format!("{}/pop_{}.png", output_dir, pop.get_generation()))
            .unwrap();
    }

    // Print the best individual from the population - if the individual implements the Display trait
    println!("{}", pop_best);

    // If individual implements the Visualise trait, you can visualise it. To do this, you need to provide the individual data
    // Individual data contains things that are not specific to the individual, but are needed for the evolution
    // These can contain stuff like coordinates of cities in the Travelling Salesman Problem, or the target values in the Math Function Approximation Problem
    let ind_data: &BasicIndividualData = pop.get_individual_data();
    // visualise returns an RgbImage that can be saved to a file or displayed
    pop_best
        .visualise(ind_data)
        .save(format!("{}/ind_{}.png", output_dir, pop.get_generation()))
        .unwrap();
}
