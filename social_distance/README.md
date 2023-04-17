# Implementation of social distancing problem using the revo library
This implementation solves the social distancing problem.


## Configuration
The configuration for the implementation is stored in the `pop.json` file.
This file contains the parameters for the evolutionary algorithm, such as the population size, the mutation rate, and the number of generations.

The visualise parameter in the configuration file controls whether or not visualisations of the population at each generation is generated.

### Example of a configuration file:
```json
{
  "pop_width": 128,
  "pop_height": 128,
  "mut_prob": 0.1,
  "mut_amount": 5.0,
  "crossover_prob": 0.2,
  "visualise" : true
}
```


## Running the implementation
To run the implementation, run the following command in the root directory of the project:
```bash
cargo run --release
```

- Best individual of each generation are stored in the `out` directory as a png files with the name `best_{generation}.png`.
- When visualisation is enabled, the population is visualised at each generation and stored in the `out` directory as a png file with the name `pop_{generation}.png`.
