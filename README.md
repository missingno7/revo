# revo - Rust 2D Cellular Evolution Library

Revo is a Rust 2D cellular evolution library designed to provide a simple framework for creating and evolving populations of 2D cellular automata. It provides an implementation of common genetic algorithm components, such as selection, mutation, and crossover, and is highly customizable to accommodate various applications.

## Quick Start
Implementation of the Travelling Salesman Problem is located in the [evo_salesman folder](https://github.com/missingno7/revo/tree/master/evo_salesman).

The example implementation of the revo library is located in the [revo folder](https://github.com/missingno7/revo/tree/master/revo). 

## Implementing Your Own Individual

To use Revo, you need to implement the `EvoIndividual` trait for your own individual. This trait provides methods for creating, mutating, and evaluating individuals in the population.

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

`IndividualData` is the type of the data needed for creating and evaluating individuals. This can be any type that you define.

`new()` method creates a new individual with default values. new_randomised() method creates a new individual with randomized values.

`copy_to()` method copies the genome of the individual to another individual.

`mutate()` method mutates the genome of the individual.

`crossover_to()` method crosses over the genome of the individual with another individual.

`count_fitness()` method counts the fitness of the individual.

`get_fitness()` method returns the fitness of the individual.

`get_visuals()` method returns the A and B values of the individual for visualisation.

The Visualise trait is optional and provides a method for visualizing the individual. Here's the method that needs to be implemented:


```rust
pub trait Visualise<IndividualData> {
    fn visualise(&self, output_filename: &str, ind_data: &IndividualData);
}
```

`output_filename` is the name of the file to save the visualization to.


## Evolution Process
In Revo, the evolution process consists of the following steps:

1. Create a new population of individuals with randomized values.
2. Evaluate the fitness of each individual in the population using the count_fitness() method.
3. Select the best individuals from the population for reproduction, using either the tournament or roulette selection strategy.
4. Create new individuals through crossover and mutation of the selected individuals.
5. Evaluate the fitness of the new individuals using the count_fitness() method.
6. Replace the weakest individuals in the population with the new individuals.
Repeat steps 3-6 until the desired number of generations is reached by calling next_gen() on the population.
During the evolution process, the following methods are called on each individual:

- `new()` or `new_randomised()`: Create a new individual.
- `mutate()`: Mutate the individual's genome.
- `copy_to()`: Copy the individual's genome to another individual.
- `crossover_to()`: Combine the individual's genome with another individual's genome.
- `count_fitness()`: Evaluate the fitness of the individual.
- `get_fitness()`: Retrieve the fitness value of the individual.

If `pop.visualise()` is called, the following method is called on each individual: 
- `get_visuals()`: Retrieve the A and B values of the individual for visualisation.

To run the evolution process you need to create a new population and call the `next_gen()` method on it.

If you want to get the best individual from the current generation of the population, call the `get_best()` method on the population.


## Contributing
Contributions are welcome! If you find a bug or have a feature request, please open an issue on GitHub.



