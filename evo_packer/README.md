# Rectangle Packing using the revo library

This implementation solves a simplified 2D rectangle packing problem

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
  "pop_width": 256,
  "pop_height": 256,
  "mut_prob": 0.05,
  "crossover_prob": 0.4,
  "visualise": true,
  "selection_strategy": "tournament", // "roulette", "tournament"

  // Packer parameters
  "n_rects": 30,
  "rect_min": 1,
  "rect_max": 10,
  "swap_prob": 0.1,
  "reverse_prob": 0.05,
  "height_change_prob": 0.1,
  "height_change_amount": 5,
  "screen_width": 512,
  "screen_height": 512,
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
