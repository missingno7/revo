# Evolutionary algorithms playground

## Description

- This repository contains a window application that allows you to play with evolutionary algorithms. 
- The playground is written in Rust and uses the [revo](revo) library for the evolutionary algorithms. 
- The application is currently able to visualise the population and the selected or best individual.
- Individual is selected by clicking on population display.
- Population can be manually evolved by pressing the `+1 gen`, `+10 gen`, or `+100 gen` buttons

The application is able to solve following problems:
- the Travelling Salesman Problem (salesman)
- Arbitrary function approximation (funtree) 
- Problem I call social distancing problem (social_distance)


## Running the application

To run the application, run the following command in the root directory of the project:

```bash
cargo run --release
```

## Configuration

The configuration for the implementation is stored in the `config.json5` file.
This file contains the parameters for the evolutionary algorithm, such as the population size, the mutation rate, and
the number of generations.

```json5
{
 // General population parameters
 "pop_width": 50,
 "pop_height": 50,
 "mut_prob": 0.02,
 "mut_amount": 10.0,
 "crossover_prob": 0.1,
 "selection_strategy": "tournament", // "tournament", "roulette"

 // Playground parameters
 "example": "funtree", // "salesman", "social_distance", "funtree"
 "display_width": 1200,
 "display_height":  1200,

 // Salesman parameters
 "n_cities": 100,
 "screen_width": 600,
 "screen_height":  600,
 "shift_prob": 0.4,
 "rev_prob": 0.4,
 "init_type": "greedy", // "naive", "noise", "insertion", "greedy"

 // Social distance parameters
 "n_points": 50,
 "required_distance": 20,

 // Funtree parameters
 "plot_width": 600,
 "plot_height":  600,
 "max_depth": 5,
 "values": "-4:4, -3:3, -2:2, -1:1, 0:0, 1:1, 2:2, 3:3, 4:4" // "x1:y1,x2:y2,..."
}
```

