# revo - Rust 2D Cellular Evolution Library

This library provides functionality for creating and evolving 2D cellular automata populations with custom individuals.

The library contains a Population struct that represents a population of individuals, as well as methods for evolving
and visualizing the population.

## Example of the implementation

- Example of full implementation using the revo library can be found in the example folder.

If you want to implement your own individual, you need to implement the Individual trait for your individual struct. The
trait contains methods for mutating and crossing over the individual.

```rust
use example::basic_individual::{BasicIndividual, BasicIndividualData};
use revo::pop_config::PopulationConfig;
use revo::population::Population;
use revo::evo_individual::EvoIndividual;

fn main() {
    // Load the population configuration
    let pop_config = PopulationConfig::new("pop_config.json");
    
    // Prepare the individual data - The data is passed to the individual when it is created
    // Individual data contains things that are not specific to the individual, but are needed for the evolution
    let ind_data = BasicIndividualData::default();
    
    // Create the population
    let mut pop: Population<BasicIndividual, BasicIndividualData> = Population::new(&pop_config, ind_data);

    // Evolve the population 
    // This will apply the evolution rules to the population and create a new generation
    pop.next_gen();

    // Get the best individual from the population
    let pop_best = pop.get_best();

    // Visualize the population
    pop.visualise(format!("pop_{}.png", pop.get_generation()).as_str());

    // Print the best individual from the population
    dbg!(pop_best)
}
```

### Population configuration

PopulationConfig struct contains the configuration for the population. The configuration can be loaded from a json file
using the `from_file` method. The configuration file contains the following parameters:

#### Example of a configuration file:

If we want to use this configuration, we can create a json file named `pop_config.json` and put the following content in it:
```json
{
  "pop_width": 200,
  "pop_height": 200,
  "mut_prob": 0.02,
  "mut_amount": 10.0,
  "crossover_prob": 0.1,
  "visualise": false
}
```


