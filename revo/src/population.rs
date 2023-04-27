use super::evo_individual::EvoIndividual;
use crate::config::Config;
use crate::evo_individual::EvoIndividualData;
use crate::utils::{IndexedLabData, LabData};
use image::RgbImage;
use lab::Lab;
use rand::rngs::ThreadRng;
use rand::{thread_rng, Rng};
use rayon::prelude::*;
use std::str::FromStr;

const DEFAULT_POP_WIDTH: usize = 128;
const DEFAULT_POP_HEIGHT: usize = 128;
const DEFAULT_MUT_PROB: f32 = 0.1;
const DEFAULT_MUT_AMOUNT: f32 = 1.0;
const DEFAULT_CROSSOVER_PROB: f32 = 0.1;
const DEFAULT_SELECTION_STRATEGY_TYPE: SelectionStrategyType = SelectionStrategyType::Tournament;

#[derive(Clone)]
pub enum SelectionStrategyType {
    Tournament,
    Roulette,
}

impl FromStr for SelectionStrategyType {
    type Err = String;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "tournament" => Ok(SelectionStrategyType::Tournament),
            "roulette" => Ok(SelectionStrategyType::Roulette),
            _ => Err(format!("{} is not a valid selection type", s)),
        }
    }
}

pub struct Population<Individual, IndividualData> {
    // Current and next generation of individuals
    inds: Vec<Individual>,

    // Population size
    pop_width: usize,
    pop_height: usize,

    // Probability parameters
    mut_prob: f32,
    mut_amount: f32,
    crossover_prob: f32,
    selection_strategy_type: SelectionStrategyType,

    // Current generation number
    i_generation: usize,

    // Data for individuals
    ind_data: IndividualData,
}

impl<Individual: EvoIndividual<IndividualData>, IndividualData: EvoIndividualData>
    Population<Individual, IndividualData>
where
    Individual: EvoIndividual<IndividualData> + Send + Sync + Clone,
    IndividualData: EvoIndividualData,
{
    pub fn get_at(&self, x: usize, y: usize) -> &Individual {
        &self.inds[y * self.pop_width + x]
    }

    pub fn get_width(&self) -> usize {
        self.pop_width
    }

    pub fn get_height(&self) -> usize {
        self.pop_height
    }

    // Function returns the number of the current generation
    pub fn get_generation(&self) -> usize {
        self.i_generation
    }

    // Function creates a new population with randomised individuals and counts their fitness
    pub fn new(config: &Config) -> Population<Individual, IndividualData> {
        let pop_width = config
            .get_uint("pop_width")
            .unwrap()
            .unwrap_or(DEFAULT_POP_WIDTH);
        let pop_height = config
            .get_uint("pop_height")
            .unwrap()
            .unwrap_or(DEFAULT_POP_HEIGHT);

        let ind_data = IndividualData::from_config(config);
        let size = pop_width * pop_height;
        let mut inds: Vec<Individual> = Vec::with_capacity(size);

        // Initialise population with randomised individuals and count their fitness in parallel
        inds.par_extend(
            (0..size)
                .into_par_iter()
                .map(|_| Self::_new_random_individual(&ind_data)),
        );

        Population {
            inds,
            pop_width,
            pop_height,
            mut_prob: config
                .get_float("mut_prob")
                .unwrap()
                .unwrap_or(DEFAULT_MUT_PROB),
            mut_amount: config
                .get_float("mut_amount")
                .unwrap()
                .unwrap_or(DEFAULT_MUT_AMOUNT),
            crossover_prob: config
                .get_float("crossover_prob")
                .unwrap()
                .unwrap_or(DEFAULT_CROSSOVER_PROB),
            selection_strategy_type: config
                .get_val("selection_strategy")
                .unwrap()
                .unwrap_or(DEFAULT_SELECTION_STRATEGY_TYPE),
            i_generation: 0,
            ind_data,
        }
    }

    // Function moves the population to the next generation
    // It does selection, crossover/mutation and counts fitness for each individual
    pub fn next_gen(&mut self) {
        let pop_size = self.inds.len();

        // Create a new vector for the next generation
        let mut next_gen_inds: Vec<Individual> = Vec::with_capacity(pop_size);

        // Do selection and crossover/mutation in parallel for each individual
        next_gen_inds.par_extend((0..pop_size).into_par_iter().map(|i| {
            let mut rng = thread_rng();

            // Select 5 individuals
            let indices = Self::_l5_selection(i, self.pop_width, self.pop_height);

            // Decide whether to do crossover or mutation
            let mut res = if rng.gen_range(0.0..1.0) < self.crossover_prob {
                // Do crossover

                // Select two individuals
                let (first_ind, second_ind) = Self::_dual_tournament(&indices, &self.inds);

                self.inds[first_ind].crossover(&self.inds[second_ind], &self.ind_data, &mut rng)
            } else {
                // Do mutation

                // Select one individual based on the selection type
                let selected_ind_index = match self.selection_strategy_type {
                    SelectionStrategyType::Roulette => {
                        Self::_roulette_selection(&mut rng, &indices, &self.inds)
                    }
                    SelectionStrategyType::Tournament => {
                        Self::_single_tournament(&indices, &self.inds)
                    }
                };

                let mut res = self.inds[selected_ind_index].clone();
                res.mutate(&self.ind_data, &mut rng, self.mut_prob, self.mut_amount);
                res
            };

            // Count fitness of the new individual and return it
            res.count_fitness(&self.ind_data);
            res
        }));

        // Swap the current generation with the next generation and increment the generation counter
        std::mem::swap(&mut self.inds, &mut next_gen_inds);
        self.i_generation += 1;
    }

    // Function returns the best individual in the current generation
    pub fn get_best(&self) -> &Individual {
        let mut best_ind = &self.inds[0];

        for i in 1..self.inds.len() {
            if self.inds[i].get_fitness() > best_ind.get_fitness() {
                best_ind = &self.inds[i];
            }
        }

        best_ind
    }

    // Function creates a visualization of the current generation in the form of an PNG image
    // It maps the fitness (L) and visual attributes (A, B) of each individual
    pub fn visualise(&self) -> RgbImage {
        let mut lab_data = self._prepare_pop_lab_data();

        lab_data = Self::_normalize_lab_data_rank_based(lab_data);

        self._write_lab_data_to_image(&lab_data)
    }

    // Function returns the data for individuals
    pub fn get_individual_data(&self) -> &IndividualData {
        &self.ind_data
    }

    // Private functions

    // Function returns the indices of 5 neighbours of i in a + shape
    fn _l5_selection(i: usize, pop_width: usize, pop_height: usize) -> Vec<usize> {
        // Get x and y coordinates of i
        let x: usize = i % pop_width;
        let y: usize = i / pop_width;

        // Compute indices of neighbors, using wrapping for out-of-bounds indices
        let left_neighbour = if x > 0 { i - 1 } else { i + pop_width - 1 };
        let right_neighbour = if x + 1 < pop_width {
            i + 1
        } else {
            i + 1 - pop_width
        };
        let top_neighbour = if y > 0 {
            i - pop_width
        } else {
            i + pop_width * (pop_height - 1)
        };
        let bottom_neighbour = if y + 1 < pop_height {
            i + pop_width
        } else {
            i - pop_width * (pop_height - 1)
        };

        // Return indices of 5 neighbours in a + shape with i in the middle
        vec![
            i,
            left_neighbour,
            right_neighbour,
            top_neighbour,
            bottom_neighbour,
        ]
    }

    fn _normalize_component(
        data: &mut [IndexedLabData],
        mut get_component: impl FnMut(&LabData) -> f64,
        mut set_component: impl FnMut(&mut LabData, f64),
        min_val: f64,
        max_val: f64,
    ) {
        let len = data.len();
        let eps = 1e-9;

        data.sort_by(|a, b| {
            get_component(&a.data)
                .partial_cmp(&get_component(&b.data))
                .unwrap()
        });
        let mut last_val = get_component(&data.last().unwrap().data);
        let mut last_val_normalised = 0.0;

        for (i, value) in data.iter_mut().enumerate() {
            let current_val = get_component(&value.data);

            if (current_val - last_val).abs() < eps {
                set_component(&mut value.data, last_val_normalised);
            } else {
                last_val = current_val;
                let normalised_val = ((i as f64) * (max_val - min_val)) / len as f64 + min_val;
                set_component(&mut value.data, normalised_val);
                last_val_normalised = normalised_val;
            }
        }
    }

    // Private methods

    // Function normalizes the L, A and B values of the population using the rank-based method
    // This method doesn't preserve the order of the values
    fn _normalize_lab_data_rank_based(mut lab_data: Vec<IndexedLabData>) -> Vec<IndexedLabData> {
        Self::_normalize_component(
            &mut lab_data,
            |lab_data| lab_data.l,
            |lab_data, val| lab_data.l = val,
            10.0,
            90.0,
        );
        Self::_normalize_component(
            &mut lab_data,
            |lab_data| lab_data.a,
            |lab_data, val| lab_data.a = val,
            -128.0,
            128.0,
        );
        Self::_normalize_component(
            &mut lab_data,
            |lab_data| lab_data.b,
            |lab_data, val| lab_data.b = val,
            -128.0,
            128.0,
        );

        lab_data
    }

    // Function writes the L, A and B values of the population to an RgbImage object
    fn _write_lab_data_to_image(&self, lab_data: &[IndexedLabData]) -> RgbImage {
        let mut img = RgbImage::new(self.pop_width as u32, self.pop_height as u32);

        for lab in lab_data {
            let x = lab.index % self.pop_width;
            let y = lab.index / self.pop_width;

            let rgb = Lab {
                l: lab.data.l as f32,
                a: lab.data.a as f32,
                b: lab.data.b as f32,
            }
            .to_rgb();
            img.put_pixel(x as u32, y as u32, image::Rgb(rgb));
        }
        img
    }

    // Function creates a new individual with randomised values and counts its fitness
    fn _new_random_individual(ind_data: &IndividualData) -> Individual {
        let mut rng = rand::thread_rng();
        let mut curr_gen_ind = Individual::new_randomised(ind_data, &mut rng);
        curr_gen_ind.count_fitness(ind_data);
        curr_gen_ind
    }

    /// Private methods

    // Function returns the index of the best individual in the tournament
    fn _single_tournament(indices: &[usize], inds: &[Individual]) -> usize {
        let mut best_i = indices[0];

        for &index in indices.iter().skip(1) {
            if inds[index].get_fitness() > inds[best_i].get_fitness() {
                best_i = index;
            }
        }

        best_i
    }

    fn _roulette_selection(rng: &mut ThreadRng, indices: &[usize], inds: &[Individual]) -> usize {
        // Get min fitness
        let mut min_fitness = inds[indices[0]].get_fitness();
        for &index in indices.iter() {
            let fitness = inds[index].get_fitness();
            if fitness < min_fitness {
                min_fitness = fitness;
            }
        }

        // Calculate the sum of fitnesses
        let mut fitness_sum = 0.0;
        for &index in indices.iter() {
            // subtract the min fitness to avoid negative values
            fitness_sum += inds[index].get_fitness() - min_fitness;
        }

        // Calculate the probabilities of each individual
        let mut probabilities = Vec::with_capacity(indices.len());
        for &index in indices.iter() {
            // subtract the min fitness to avoid negative values
            let prob = (inds[index].get_fitness() - min_fitness) / fitness_sum;
            probabilities.push(prob);
        }

        // Select an individual based on the probabilities
        let rand_val = rng.gen_range(0.0..1.0);
        let mut sum = 0.0;
        for i in 0..indices.len() {
            sum += probabilities[i];
            if sum > rand_val {
                return indices[i];
            }
        }

        // If the loop didn't return, return the last index
        *indices.last().unwrap()
    }

    // Function selects two individuals using roulette selection
    fn _dual_rulette(
        rng: &mut ThreadRng,
        indices: &[usize],
        inds: &[Individual],
    ) -> (usize, usize) {
        // Select the first individual
        let first = Self::_roulette_selection(rng, indices, inds);

        // Remove the first index from the indices vector to avoid selecting the same individual twice
        let mut indices2 = Vec::with_capacity(indices.len() - 1);
        for item in indices {
            if item != &first {
                indices2.push(*item);
            }
        }

        // Select the second individual
        let second = Self::_roulette_selection(rng, &indices2, inds);

        (first, second)
    }

    // Function returns the indices of the two best individuals in the tournament
    fn _dual_tournament(indices: &[usize], inds: &[Individual]) -> (usize, usize) {
        let mut best_i = indices[0];
        let mut second_best_i = indices[1];

        for &index in indices.iter().skip(1) {
            if inds[index].get_fitness() > inds[best_i].get_fitness() {
                second_best_i = best_i;
                best_i = index;
            } else if inds[index].get_fitness() > inds[second_best_i].get_fitness() {
                second_best_i = index;
            }
        }

        (best_i, second_best_i)
    }

    // Function gets the L, A and B values of the current generation
    fn _prepare_pop_lab_data(&self) -> Vec<IndexedLabData> {
        let len = self.inds.len();
        let mut lab_data: Vec<IndexedLabData> = Vec::with_capacity(len);
        for (i, ind) in self.inds.iter().enumerate() {
            let l = ind.get_fitness();
            let (a, b) = ind.get_visuals(&self.ind_data);
            lab_data.push(IndexedLabData::new(l, a, b, i));
        }
        lab_data
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::testing::{MockIndividual, MockIndividualData};
    use crate::utils::LabData;

    pub type TestPopulation = Population<MockIndividual, MockIndividualData>;

    #[test]
    fn test_l5_selection() {
        let pop_width = 5;
        let pop_height = 5;

        // indices goes like [middle, left, right, up, down]
        // Test top-left corner
        let i = 0;
        let neighbors = TestPopulation::_l5_selection(i, pop_width, pop_height);
        assert_eq!(neighbors, vec![0, 4, 1, 20, 5]);

        // Test top-right corner
        let i = 4;
        let neighbors = TestPopulation::_l5_selection(i, pop_width, pop_height);
        assert_eq!(neighbors, vec![4, 3, 0, 24, 9]);

        // Test bottom-left corner
        let i = 20;
        let neighbors = TestPopulation::_l5_selection(i, pop_width, pop_height);
        assert_eq!(neighbors, vec![20, 24, 21, 15, 0]);

        // Test bottom-right corner
        let i = 24;
        let neighbors = TestPopulation::_l5_selection(i, pop_width, pop_height);
        assert_eq!(neighbors, vec![24, 23, 20, 19, 4]);

        // Test middle element
        let i = 12;
        let neighbors = TestPopulation::_l5_selection(i, pop_width, pop_height);
        assert_eq!(neighbors, vec![12, 11, 13, 7, 17]);

        // Test bottom-middle element
        let i = 22;
        let neighbors = TestPopulation::_l5_selection(i, pop_width, pop_height);
        assert_eq!(neighbors, vec![22, 21, 23, 17, 2]);
    }

    #[test]
    fn test_single_tournament() {
        let mut vec_ind = Vec::new();
        for i in 0..6 {
            vec_ind.push(MockIndividual {
                fitness: i as f64,
                visuals: (0.0, 0.0),
                value: 0.0,
            });
        }

        let res = TestPopulation::_single_tournament(&vec![0, 3, 2, 1], &mut vec_ind);
        assert_eq!(res, 3);

        let res = TestPopulation::_single_tournament(&vec![3, 0, 2, 4], &mut vec_ind);
        assert_eq!(res, 4);
    }

    #[test]
    fn test_dual_tournament() {
        let mut vec_ind = Vec::new();
        for i in 0..6 {
            vec_ind.push(MockIndividual {
                fitness: i as f64,
                visuals: (0.0, 0.0),
                value: 0.0,
            });
        }

        let res = TestPopulation::_dual_tournament(&vec![0, 3, 2, 1], &mut vec_ind);
        assert_eq!(res, (3, 2));

        let res = TestPopulation::_dual_tournament(&vec![3, 0, 2, 4], &mut vec_ind);
        assert_eq!(res, (4, 3));
    }

    #[test]
    fn test_population() {
        let config = Config::from_str("{\"pop_width\": 3,  \"pop_height\": 3, \"mut_prob\":1.0, \"crossover_prob\":0.0, \"selection_strategy_type\":\"tournament\"  }").unwrap()
        ;

        let mut pop = Population::new(&config);

        // Fill the population with mock individuals
        let mut vec_ind = Vec::new();
        for i in 0..pop.inds.len() {
            vec_ind.push(MockIndividual {
                fitness: i as f64,
                visuals: (i as f64, i as f64),
                value: i as f64,
            });
        }
        pop.inds = vec_ind.clone();

        // Test get_best
        let res = pop.get_best();
        // Pop should return the best individual - the one with the highest fitness value (last in the vector)
        assert_eq!(res.value, vec_ind[pop.inds.len() - 1].value);

        // Test get_at
        assert_eq!(pop.get_at(1, 2).value, 7.0);
        assert_eq!(pop.get_at(2, 0).value, 2.0);

        // Test next_gen
        assert_eq!(pop.get_generation(), 0);
        pop.next_gen();
        assert_eq!(pop.inds[0].value, 7.0);
        assert_eq!(pop.inds[1].value, 8.0);
        assert_eq!(pop.inds[2].value, 9.0);

        assert_eq!(pop.inds[3].value, 7.0);
        assert_eq!(pop.inds[4].value, 8.0);
        assert_eq!(pop.inds[5].value, 9.0);

        assert_eq!(pop.inds[6].value, 9.0);
        assert_eq!(pop.inds[7].value, 9.0);
        assert_eq!(pop.inds[8].value, 9.0);

        assert_eq!(pop.get_generation(), 1);

        // Test _prepare_pop_lab_data
        let lab_data = pop._prepare_pop_lab_data();
        assert_eq!(lab_data.len(), 9);
        assert_eq!(
            lab_data[0].data,
            LabData {
                l: 7.0,
                a: 6.0,
                b: 6.0,
            }
        );
        assert_eq!(
            lab_data[1].data,
            LabData {
                l: 8.0,
                a: 7.0,
                b: 7.0,
            }
        );
        assert_eq!(
            lab_data[2].data,
            LabData {
                l: 9.0,
                a: 8.0,
                b: 8.0,
            }
        );

        assert_eq!(
            lab_data[3].data,
            LabData {
                l: 7.0,
                a: 6.0,
                b: 6.0,
            }
        );
        assert_eq!(
            lab_data[4].data,
            LabData {
                l: 8.0,
                a: 7.0,
                b: 7.0,
            }
        );
        assert_eq!(
            lab_data[5].data,
            LabData {
                l: 9.0,
                a: 8.0,
                b: 8.0,
            }
        );

        assert_eq!(
            lab_data[6].data,
            LabData {
                l: 9.0,
                a: 8.0,
                b: 8.0,
            }
        );
        assert_eq!(
            lab_data[7].data,
            LabData {
                l: 9.0,
                a: 8.0,
                b: 8.0,
            }
        );
        assert_eq!(
            lab_data[8].data,
            LabData {
                l: 9.0,
                a: 8.0,
                b: 8.0,
            }
        );
    }
}
