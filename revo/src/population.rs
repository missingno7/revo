use super::evo_individual::EvoIndividual;
use crate::pop_config::PopulationConfig;
use image::RgbImage;
use lab::Lab;
use rand::{thread_rng, Rng};
use rayon::prelude::*;

pub struct Population<Individual, IndividualData> {
    // Current and next generation of individuals
    curr_gen_inds: Vec<Individual>,
    next_gen_inds: Vec<Individual>,

    // Population size
    pop_width: usize,
    pop_height: usize,

    // Probability parameters
    mut_prob: f32,
    mut_amount: f32,
    crossover_prob: f32,

    // Current generation number
    i_generation: usize,

    // Data for individuals
    ind_data: IndividualData,
}

impl<Individual: EvoIndividual<IndividualData> + Send + Sync + Clone, IndividualData: Sync>
    Population<Individual, IndividualData>
{
    // Function creates a new individual with randomised values and counts its fitness
    fn _new_random_individual(ind_data: &IndividualData) -> Individual {
        let mut rng = rand::thread_rng();
        let mut curr_gen_ind = Individual::new_randomised(ind_data, &mut rng);
        curr_gen_ind.count_fitness(ind_data);
        curr_gen_ind
    }

    // Function creates a new population with randomised individuals and counts their fitness
    pub fn new(
        pop_config: &PopulationConfig,
        ind_data: IndividualData,
    ) -> Population<Individual, IndividualData> {
        let size = pop_config.pop_width * pop_config.pop_height;
        let mut curr_gen_inds: Vec<Individual> = Vec::with_capacity(size);
        let mut next_gen_inds: Vec<Individual> = Vec::with_capacity(size);

        // Initialise population with randomised individuals and count their fitness in parallel
        curr_gen_inds.par_extend(
            (0..size)
                .into_par_iter()
                .map(|_| Self::_new_random_individual(&ind_data)),
        );

        // Just fill next_gen with default values
        next_gen_inds.par_extend(
            (0..size)
                .into_par_iter()
                .map(|_| Individual::new(&ind_data)),
        );

        Population {
            curr_gen_inds,
            next_gen_inds,
            pop_width: pop_config.pop_width,
            pop_height: pop_config.pop_height,
            mut_prob: pop_config.mut_prob,
            mut_amount: pop_config.mut_amount,
            crossover_prob: pop_config.crossover_prob,
            i_generation: 0,
            ind_data,
        }
    }

    // Function moves the population to the next generation
    // It does selection, crossover/mutation and counts fitness for each individual
    pub fn next_gen(&mut self) {
        // Do selection and crossover/mutation in parallel for each individual
        self.next_gen_inds
            .par_iter_mut()
            .enumerate()
            .take(self.curr_gen_inds.len())
            .for_each(|(i, res)| {
                let mut rng = thread_rng();

                // Select 5 individuals
                let indices = Self::l5_selection(i, self.pop_width, self.pop_height);

                // Decide whether to do crossover or mutation
                if rng.gen_range(0.0..1.0) < self.crossover_prob {
                    // Do crossover
                    let (first_ind, second_ind) =
                        Self::dual_tournament(&indices, &self.curr_gen_inds);
                    self.curr_gen_inds[first_ind].crossover_to(
                        &self.curr_gen_inds[second_ind],
                        res,
                        &self.ind_data,
                        &mut rng,
                    )
                } else {
                    // Do mutation
                    self.curr_gen_inds[Self::single_tournament(&indices, &self.curr_gen_inds)]
                        .copy_to(res);
                    res.mutate(&self.ind_data, &mut rng, self.mut_prob, self.mut_amount);
                }
                res.count_fitness(&self.ind_data);
            });

        // Advance to next generation
        std::mem::swap(&mut self.curr_gen_inds, &mut self.next_gen_inds);
        self.i_generation += 1;
    }

    // Function returns the best individual in the current generation
    pub fn get_best(&self) -> Individual {
        let mut best_ind = &self.curr_gen_inds[0];

        for i in 1..self.curr_gen_inds.len() {
            if self.curr_gen_inds[i].get_fitness() > best_ind.get_fitness() {
                best_ind = &self.curr_gen_inds[i];
            }
        }

        best_ind.clone()
    }

    // Function returns the number of the current generation
    pub fn get_generation(&self) -> usize {
        self.i_generation
    }

    /// Private methods

    // Function returns the indices of 5 neighbours of i in a + shape
    fn l5_selection(i: usize, pop_width: usize, pop_height: usize) -> Vec<usize> {
        // Get x and y coordinates of i
        let x: usize = i % pop_width;
        let y: usize = i / pop_width;

        // Get indices of left, right, up and down neighbours
        let row_start_index = y * pop_width;
        let left_neighbour = row_start_index + ((x + pop_width - 1) % pop_width);
        let right_neigbour = row_start_index + ((x + 1) % pop_width);

        let column_start_index = x % pop_width;
        let top_neighbour = ((y + pop_height - 1) % pop_height) * pop_width + column_start_index;
        let bottom_neighbour = ((y + 1) % pop_height) * pop_width + column_start_index;

        // Return indices of 5 neighbours in a + shape with i in the middle
        vec![
            i,
            left_neighbour,
            right_neigbour,
            top_neighbour,
            bottom_neighbour,
        ]
    }

    // Function returns the index of the best individual in the tournament
    fn single_tournament(indices: &[usize], curr_gen_inds: &[Individual]) -> usize {
        let mut best_i = indices[0];

        for &index in indices.iter().skip(1) {
            if curr_gen_inds[index].get_fitness() > curr_gen_inds[best_i].get_fitness() {
                best_i = index;
            }
        }

        best_i
    }

    // Function returns the indices of the two best individuals in the tournament
    fn dual_tournament(indices: &[usize], curr_gen_inds: &[Individual]) -> (usize, usize) {
        let mut best_i = indices[0];
        let mut second_best_i = indices[1];

        for &index in indices.iter().skip(1) {
            if curr_gen_inds[index].get_fitness() > curr_gen_inds[best_i].get_fitness() {
                second_best_i = best_i;
                best_i = index;
            } else if curr_gen_inds[index].get_fitness()
                > curr_gen_inds[second_best_i].get_fitness()
            {
                second_best_i = index;
            }
        }

        (best_i, second_best_i)
    }

    // Function creates a visualization of the current generation in the form of an PNG image
    // It maps the fitness (L) and visual attributes (A, B) of each individual
    pub fn visualise(&self, filename: &str) {
        let mut lab_data = self._prepare_pop_lab_data();

        self._normalize_lab_data_rank_based(&mut lab_data);

        let mut img = RgbImage::new(self.pop_width as u32, self.pop_height as u32);
        self._write_lab_data_to_image(&mut img, &lab_data);
        img.save(filename).unwrap();
    }

    // Function gets the L, A and B values of the current generation
    fn _prepare_pop_lab_data(&self) -> Vec<(f64, f64, f64, usize)> {
        let len = self.curr_gen_inds.len();
        let mut lab = Vec::with_capacity(len);
        for (i, ind) in self.curr_gen_inds.iter().enumerate() {
            let l = ind.get_fitness();
            let (a, b) = ind.get_visuals(&self.ind_data);
            lab.push((l, a, b, i));
        }
        lab
    }

    // Function normalizes the L, A and B values of the population using the rank-based method
    // This method doesn't preserve the order of the values
    fn _normalize_lab_data_rank_based(&self, lab: &mut [(f64, f64, f64, usize)]) {
        let len = lab.len();

        // Sort and normalize L
        lab.sort_by(|a, b| a.0.partial_cmp(&b.0).unwrap());
        for (i, value) in lab.iter_mut().enumerate() {
            value.0 = ((i as f64) * 100.0) / len as f64;
        }
        // Sort and normalize A
        lab.sort_by(|a, b| a.1.partial_cmp(&b.1).unwrap());
        for (i, value) in lab.iter_mut().enumerate() {
            value.1 = ((i as f64) * 256.0) / len as f64 - 128.0;
        }
        // Sort and normalize B
        lab.sort_by(|a, b| a.2.partial_cmp(&b.2).unwrap());
        for (i, value) in lab.iter_mut().enumerate() {
            value.2 = ((i as f64) * 256.0) / len as f64 - 128.0;
        }
    }

    // Function writes the L, A and B values of the population to an RgbImage object
    fn _write_lab_data_to_image(&self, img: &mut RgbImage, lab: &[(f64, f64, f64, usize)]) {
        for (l, a, b, i) in lab {
            let x = i % self.pop_width;
            let y = i / self.pop_width;

            let rgb = Lab {
                l: *l as f32,
                a: *a as f32,
                b: *b as f32,
            }
            .to_rgb();
            img.put_pixel(x as u32, y as u32, image::Rgb(rgb));
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::basic_individual::{BasicIndividual, BasicIndividualData};

    type TestPopulation = Population<BasicIndividual, BasicIndividualData>;

    #[test]
    fn test_l5_selection() {
        let pop_width = 5;
        let pop_height = 5;

        // indices goes like [middle, left, right, up, down]
        // Test top-left corner
        let i = 0;
        let neighbors = TestPopulation::l5_selection(i, pop_width, pop_height);
        assert_eq!(neighbors, vec![0, 4, 1, 20, 5]);

        // Test top-right corner
        let i = 4;
        let neighbors = TestPopulation::l5_selection(i, pop_width, pop_height);
        assert_eq!(neighbors, vec![4, 3, 0, 24, 9]);

        // Test bottom-left corner
        let i = 20;
        let neighbors = TestPopulation::l5_selection(i, pop_width, pop_height);
        assert_eq!(neighbors, vec![20, 24, 21, 15, 0]);

        // Test bottom-right corner
        let i = 24;
        let neighbors = TestPopulation::l5_selection(i, pop_width, pop_height);
        assert_eq!(neighbors, vec![24, 23, 20, 19, 4]);

        // Test middle element
        let i = 12;
        let neighbors = TestPopulation::l5_selection(i, pop_width, pop_height);
        assert_eq!(neighbors, vec![12, 11, 13, 7, 17]);

        // Test bottom-middle element
        let i = 22;
        let neighbors = TestPopulation::l5_selection(i, pop_width, pop_height);
        assert_eq!(neighbors, vec![22, 21, 23, 17, 2]);
    }
}
