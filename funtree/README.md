# Arbitrary function approximation (funtree)
## Description
This implementation solves the arbitrary function approximation problem.


## Configuration

The configuration for the implementation is stored in the `config.json5` file.
This file contains the parameters for the evolutionary algorithm, such as the population size, the mutation rate, and
the number of generations.

The visualise parameter in the configuration file controls whether or not visualisations of the population at each
generation is generated.

### Example of a configuration file:

```json5
{
  // Population parameters
  "pop_width": 1000,
  "pop_height": 100,
  "mut_prob": 0.015,
  "mut_amount": 0.1,
  "crossover_prob": 0.1,
  "selection_strategy": "tournament", // "tournament", "roulette"
  "visualise": false,

  // Funtree parameters
  "plot_width": 800,
  "plot_height":  400,
  "max_depth": 8,
  "values": "0:0, 1:2, 2:3, 3:4, 4:5" // "x1:y1,x2:y2,..."
}
```

- Values are given as a comma separated list of `x:y` pairs, where `x` is the input value and `y` is the expected output value.

## Running the implementation

To run the implementation, run the following command in the root directory of the project:

```bash
cargo run --release
```

- Plots of function that do the best approximation of each generation (best individuals) are stored in the `out` directory as a png files with the
  name `best_{generation}.png`.
- Best individual is also printed to the console.
- When visualisation is enabled, the population is visualised at each generation and stored in the `out` directory as a
  png file with the name `pop_{generation}.png`.

