# Travelling Salesman Problem implementation using the revo library

This implementation solves the Travelling Salesman Problem.

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
  "pop_width": 200,
  "pop_height": 200,
  "mut_prob": 0.02,
  "crossover_prob": 0.1,
  "visualise": false,
  "selection_strategy": "tournament", // "roulette", "tournament"

  // TSP parameters
  "n_cities": 300,
  "screen_width": 1000,
  "screen_height": 1000,
  "shift_prob": 0.4,
  "rev_prob": 0.4,
  "init_type": "greedy" // "naive", "noise", "insertion", "greedy"
}
```

## Running the implementation

To run the implementation, run the following command in the root directory of the project:

```bash
cargo run --release
```

- Best individual of each generation are stored in the `out` directory as a png files with the
  name `best_{generation}.png`.
- When visualisation is enabled, the population is visualised at each generation and stored in the `out` directory as a
  png file with the name `pop_{generation}.png`.
