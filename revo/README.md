# revo - Rust 2D Cellular Evolution Library

This library provides functionality for creating and evolving 2D cellular automata populations with custom individuals.

The library contains a Population struct that represents a population of individuals, as well as methods for evolving
and visualizing the population.

# Quick Start

- Example of full implementation using the revo library can be found in the [example](example) folder.

```rust
use example::basic_individual::{BasicIndividual, BasicIndividualData};
use revo::config::Config;
use revo::population::Population;
use revo::evo_individual::Visualise;

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
    if visualise
    {
        pop.visualise().save(format!("{}/pop_{}.png", output_dir, pop.get_generation())).unwrap();
    }

    // Print the string representation of best individual from the population - if the individual implements the Display trait
    println!("{}", pop_best);
        
    // If individual implements the Visualise trait, you can visualise it. To do this, you need to provide the individual data
    // Individual data contains things that are not specific to the individual, but are needed for the evolution
    // These can contain stuff like coordinates of cities in the Travelling Salesman Problem, or the target values in the Math Function Approximation Problem
    let ind_data: &BasicIndividualData = pop.get_individual_data();
    // visualise returns an RgbImage that can be saved to a file or displayed
    pop_best.visualise(ind_data).save(format!("{}/ind_{}.png", output_dir, pop.get_generation())).unwrap();
}
```

## Evolution Process

In Revo, the evolution process consists of the following steps:

1. Create a new population of individuals with randomized values using `new_randomised`.
2. Evaluate the fitness of each individual in the population using the `count_fitness` method.
3. Select the best individuals from the current population for reproduction, using either the tournament or roulette
   selection strategy using `get_fitness`.
4. Create new individuals through `crossover` and `clone` with `mutate` from the selected individuals in the new
   population.
5. Evaluate the fitness of the new individuals using the `count_fitness` method.
6. Swap the old population with the new population.

- Replace the weakest individuals in the population with the new individuals by performing steps 3-6 until the desired
  number of generations is reached by calling `next_gen()` on the population.
- The individuals are selected from L5 neighbourhood of the current individual. Which means that the individual itself
  and the 4 individuals around it are selected for potential reproduction. When the neighbourhood is out of bounds,
  the neighbourhood wraps around to the other side of the population.

# The Population struct

The population struct is used for storing the individuals and performing the evolution process. It provides methods for
creating, evaluating, and visualising the population.

Population struct contains the following public methods:

`new(config: &Config) -> Population`: Create a new population from the given configuration.

`next_gen(&mut self)`: Evolve the population by creating a new generation.

`get_best(&self) -> &Individual`: Get the best individual from the population.

`visualise(&self) -> RgbImage`: Visualize the population to a `RgbImage`.

`get_at(&self, x: usize, y: usize) -> &Individual`: Get the individual at the given coordinates.

`get_width(&self) -> usize`: Get the width of the population.

`get_height(&self) -> usize`: Get the height of the population.

`get_generation(&self) -> usize`: Get the number of current generation of the population.

`get_individual_data(&self) -> &IndividualData`: Get the individual data from the population. IndividualData are created during `new` by calling `IndividualData::from_config(config)`.

# Implementing Your Own Individual

To use Revo, you need to implement the `EvoIndividual` trait for your own individual. This trait provides methods for
creating, mutating, and evaluating individuals in the population.

Here are the methods that need to be implemented:

```rust
use rand::rngs::ThreadRng;

pub trait EvoIndividual<IndividualData>: Send + Sync {
    // Create a new individual with randomised values
    fn new_randomised(ind_data: &IndividualData, rng: &mut ThreadRng) -> Self;

    // Copy only genome of the individual to another individual
    fn copy_to(&self, ind: &mut Self);

    // Mutate the genome of the individual
    fn mutate(
        &mut self,
        ind_data: &IndividualData,
        rng: &mut ThreadRng,
        mut_prob: f32,
        mut_amount: f32,
    );

    // Return new Individual with the genome that is a crossover of two individuals
    fn crossover(&self, another_ind: &Self, ind_data: &IndividualData, rng: &mut ThreadRng) -> Self;

    // Count the fitness of the individual
    fn count_fitness(&mut self, ind_data: &IndividualData);

    // Get the fitness of the individual
    fn get_fitness(&self) -> f64;

    // These values are some kind of semantic hash of the individual used for comparing individuals in visualisation of the population
    fn get_visuals(&self, ind_data: &IndividualData) -> (f64, f64);
}
```

`IndividualData` is the type of the data needed for creating and evaluating individuals. This can be any type that you
define.

`new_randomised(ind_data: &IndividualData, rng: &mut ThreadRng) -> Self` method creates a new individual with randomized
values.

`mutate(&mut self, ind_data: &IndividualData, rng: &mut ThreadRng, mut_prob: f32, mut_amount: f32)` method mutates the
genome of the individual by a given probability and amount.

`crossover(&self, another_ind: &Self, ind_data: &IndividualData, rng: &mut ThreadRng) -> Self` method returns new
individual created by crossover of the self individual and another individual.

`count_fitness(&self, ind_data: &IndividualData)` method counts the fitness of the individual and stores it in the
individual.

`get_fitness(&self) -> f64` method returns the fitness of the individual stored in the individual.

`get_visuals(&self, ind_data: &IndividualData) -> (f64, f64)` method returns the A and B values of the individual for
generating color value for visualisation of the
population. It is used in the `visualise` method of the population.

The Visualise trait is optional and provides a method for visualizing the individual. Here's the method that needs to be
implemented:

```rust
pub trait Visualise<IndividualData> {
    fn visualise(&self, ind_data: &IndividualData) -> RgbImage;
}
```

`visualise(&IndividualData) -> RgbImage` method returns an image of the individual for visualisation.
In case of travelling salesman problem this method returns an image of the path that the salesman takes. It requires the
individual data to get the coordinates of the cities and genom of the individual to get the order of the cities.

### Configuration

Config struct contains the json wrapper and methods for retrieving values from the json file. The config can be loaded
from a json file
using the `from_file` method.

#### Methods for retrieving values from the json file:

Each `may_get_*` method returns `Result<Option<T>, String>` where `T` is the type of the value that is being retrieved.
Method takes a string as an argument, which is the key of the value in the json file.

`may_get_int(key: &str) -> Result<Option<T>, String>`: Retrieve an integer value from the json file up to `i64`.

`may_get_uint(key: &str) -> Result<Option<T>, String>`: Retrieve an unsigned integer value from the json file up to `u64`.
Fails if the value is negative.

`may_get_float(key: &str) -> Result<Option<T>, String>`: Retrieve a float value from the json file up to `f64`.

`may_get_bool(key: &str) -> Result<Option<T>, bool>`: Retrieve a boolean value from the json file.

`may_get_val(key: &str) -> Result<Option<T>, String>`: Retrieve any type of value that implements `FromStr` from the json
file. It can be for example used to retrieve a values of enum types.

`may_get_enum(key: &str) -> Result<Option<T>, String>`: Retrieve a value of an enum type from the json file. It works similarly to `may_get_val` but it also checks if the value is a valid variant of the enum and display possible variants in the error message.

There are also `get_*` methods that are similar to `may_get_*` methods but they return the `Result<T, String>` instead of `Result<Option<T>, String>`. They fail if the value is not present in the json file.

#### Example of a configuration file:

If we want to use this configuration, we can create a json file named `config.json5` and put the following content in it:

```json5
{
  // I use json5 because it supports comments, but json format is also supported.
  "pop_width": 200,
  "pop_height": 200,
  "mut_prob": 0.02,
  "mut_amount": 10.0,
  "crossover_prob": 0.1,
  "visualise": false,
  "selection_strategy": "tournament", // "roulette", "tournament"
  
  // Any other values can be added to the json file, they will be ignored if they are not used in the code.
}
```

- pop_width and pop_height are the width and height of the population. Which means that population will have 40000
  individuals in total.
- mut_prob is the probability if new individual will be mutated.
- mut_amount is the amount of mutation when new individual is mutated.
- crossover_prob is the probability of new individual being created by crossover of two individuals.
- visualise is a boolean value that determines if the population will be visualised.
- selection_strategy is a string value that determines which selection strategy will be used. Possible values are
  "roulette" and "tournament". If the value is not present in the json file, the tournament selection strategy will be used.
