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

fn main() {
   // Load the configuration from the config.json file
   let config = Config::new("config.json");
   let output_dir = "./";

   // Prepare the individual data - The data is passed to the individual when it is created
   // Individual data contains things that are not specific to the individual, but are needed for the evolution
   // These can contain things like coordinates of cities in the Travelling Salesman Problem, or the target values in the Math Function Approximation Problem
   let ind_data = BasicIndividualData::default();

   // Create the population
   let mut pop: Population<BasicIndividual, BasicIndividualData> = Population::new(&config, ind_data);

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

   // Print the best individual from the population - if the individual implements the Display trait
   println!("{}", pop_best);
}
```

### Configuration

Config struct contains the json wrapper and methods for retrieving values from the json file. The config can be loaded from a json file
using the `from_file` method.

#### Methods for retrieving values from the json file:

Each `get_*` method returns `Result<Option<T>, String>` where T is the type of the value that is being retrieved.

`get_int()` - Retrieve an integer value from the json file up to i64.

`get_uint()` - Retrieve an unsigned integer value from the json file up to u64. Fails if the value is negative.

`get_float()` - Retrieve a float value from the json file up to f64.

`get_bool()` - Retrieve a boolean value from the json file.

`get_val()` - Retrieve any type of value that implements `FromStr` from the json file. It can be for example used to retrieve a values of enum types.

#### Example of a configuration file:

If we want to use this configuration, we can create a json file named `config.json` and put the following content in it:

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

## Evolution Process

In Revo, the evolution process consists of the following steps:

1. Create a new population of individuals with randomized values using `new()` and `new_randomised()`.
2. Evaluate the fitness of each individual in the population using the `count_fitness()` method.
3. Select the best individuals from the current population for reproduction, using either the tournament or roulette selection
   strategy using `get_fitness()`.
4. Create new individuals through `crossover_to()` and `copy_to()` with `mutate()` from the selected individuals in the new population.
5. Evaluate the fitness of the new individuals using the `count_fitness()` method.
6. Swap the old population with the new population.
- Replace the weakest individuals in the population with the new individuals by performing steps 3-6 until the desired number of generations is reached by calling `next_gen()` on the population.

During the evolution process, the following methods are called on each individual:

`new()` or `new_randomised()`: Create a new individual.
 
`mutate()`: Mutate the individual's genome.

`copy_to()`: Copy the individual's genome to another individual.

`crossover_to()`: Combine the individual's genome with another individual's genome.

`count_fitness()`: Evaluate the fitness of the individual.

`get_fitness()`: Retrieve the fitness value of the individual.

If `pop.visualise()` is called, the following method is called on each individual:

`get_visuals()`: Retrieve the A and B values of the individual for visualisation.

To run the evolution process you need to create a new population and call the `next_gen()` method on it.

If you want to get the best individual from the current generation of the population, call the `get_best()` method on
the population.

# Implementing Your Own Individual

To use Revo, you need to implement the `EvoIndividual` trait for your own individual. This trait provides methods for
creating, mutating, and evaluating individuals in the population.

Here are the methods that need to be implemented:

```rust
use rand::rngs::ThreadRng;

pub trait EvoIndividual<IndividualData>: Send + Sync {
    // Create a new individual with default values
    fn new(ind_data: &IndividualData) -> Self;

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

    // Crossover the genome of the individual with another individual and store the result in dest_int
    fn crossover_to(
        &self,
        another_ind: &Self,
        dest_int: &mut Self,
        ind_data: &IndividualData,
        rng: &mut ThreadRng,
    );

    // Count the fitness of the individual
    fn count_fitness(&mut self, ind_data: &IndividualData);

    // Get the fitness of the individual
    fn get_fitness(&self) -> f64;

    // Get the A and B values of the individual for visualisation
    fn get_visuals(&self, ind_data: &IndividualData) -> (f64, f64);
}
```

`IndividualData` is the type of the data needed for creating and evaluating individuals. This can be any type that you
define.

`new()` method creates a new individual with default values.

`new_randomised()` method creates a new individual with randomized values.

`copy_to()` method copies the genome of the self individual to another individual.

`mutate()` method mutates the genome of the individual.

`crossover_to()` method combines the genome of the self individual with another individual and copies the result to the
destination individual.

`count_fitness()` method counts the fitness of the individual and stores it in the individual.

`get_fitness()` method returns the fitness of the individual stored in the individual.

`get_visuals()` method returns the A and B values of the individual for generating color value for visualisation of the
genom.

The Visualise trait is optional and provides a method for visualizing the individual. Here's the method that needs to be
implemented:

```rust
pub trait Visualise<IndividualData> {
    fn visualise(&self, ind_data: &IndividualData) -> RgbImage;
}
```

`visualise()` method returns an image of the individual for visualisation. E.g. the path of the travelling salesman
problem.

