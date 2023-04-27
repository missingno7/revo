use example::basic_individual::{BasicIndividual, BasicIndividualData};
use revo::config::Config;
use revo::evo_population::EvoPopulation;
use revo::population::Population;

fn main() {
    // Load the configuration from the config.json file
    let config = Config::new("config.json");
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
    let visualise: bool = config.get_bool("visualise").unwrap().unwrap_or(false);

    // Visualize the population
    if visualise {
        pop.visualise()
            .save(format!("{}/pop_{}.png", output_dir, pop.get_generation()))
            .unwrap();
    }

    // Print the best individual from the population - if the individual implements the Display trait
    println!("{}", pop_best);
}
