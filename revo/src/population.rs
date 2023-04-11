use super::evo_individual::EvoIndividual;
use crate::pop_config::PopulationConfig;
use crate::utils::IndexedLabData;
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
                let indices = Self::_l5_selection(i, self.pop_width, self.pop_height);

                // Decide whether to do crossover or mutation
                if rng.gen_range(0.0..1.0) < self.crossover_prob {
                    // Do crossover
                    let (first_ind, second_ind) =
                        Self::_dual_tournament(&indices, &self.curr_gen_inds);
                    self.curr_gen_inds[first_ind].crossover_to(
                        &self.curr_gen_inds[second_ind],
                        res,
                        &self.ind_data,
                        &mut rng,
                    )
                } else {
                    // Do mutation
                    self.curr_gen_inds[Self::_single_tournament(&indices, &self.curr_gen_inds)]
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

    pub fn get_at(&self, x: usize, y: usize) -> Individual {
        self.curr_gen_inds[y * self.pop_width + x].clone()
    }

    // Function returns the number of the current generation
    pub fn get_generation(&self) -> usize {
        self.i_generation
    }

    // Function creates a visualization of the current generation in the form of an PNG image
    // It maps the fitness (L) and visual attributes (A, B) of each individual
    pub fn visualise(&self, filename: &str) {
        let mut lab_data = self._prepare_pop_lab_data();

        lab_data = self._normalize_lab_data_rank_based(lab_data);

        let image = self._write_lab_data_to_image(&lab_data);

        image.save(filename).unwrap();
    }

    /// Private methods

    // Function returns the indices of 5 neighbours of i in a + shape
    fn _l5_selection(i: usize, pop_width: usize, pop_height: usize) -> Vec<usize> {
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
    fn _single_tournament(indices: &[usize], curr_gen_inds: &[Individual]) -> usize {
        let mut best_i = indices[0];

        for &index in indices.iter().skip(1) {
            if curr_gen_inds[index].get_fitness() > curr_gen_inds[best_i].get_fitness() {
                best_i = index;
            }
        }

        best_i
    }

    // Function returns the indices of the two best individuals in the tournament
    fn _dual_tournament(indices: &[usize], curr_gen_inds: &[Individual]) -> (usize, usize) {
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

    // Function gets the L, A and B values of the current generation
    fn _prepare_pop_lab_data(&self) -> Vec<IndexedLabData> {
        let len = self.curr_gen_inds.len();
        let mut lab_data: Vec<IndexedLabData> = Vec::with_capacity(len);
        for (i, ind) in self.curr_gen_inds.iter().enumerate() {
            let l = ind.get_fitness();
            let (a, b) = ind.get_visuals(&self.ind_data);
            lab_data.push(IndexedLabData::new(l, a, b, i));
        }
        lab_data
    }

    // Function normalizes the L, A and B values of the population using the rank-based method
    // This method doesn't preserve the order of the values
    fn _normalize_lab_data_rank_based(
        &self,
        mut lab_data: Vec<IndexedLabData>,
    ) -> Vec<IndexedLabData> {
        let len = lab_data.len();

        // Sort and normalize L
        lab_data.sort_by(|a, b| a.data.l.partial_cmp(&b.data.l).unwrap());
        for (i, value) in lab_data.iter_mut().enumerate() {
            value.data.l = ((i as f64) * 100.0) / len as f64;
        }
        // Sort and normalize A
        lab_data.sort_by(|a, b| a.data.a.partial_cmp(&b.data.a).unwrap());
        for (i, value) in lab_data.iter_mut().enumerate() {
            value.data.a = ((i as f64) * 256.0) / len as f64 - 128.0;
        }
        // Sort and normalize B
        lab_data.sort_by(|a, b| a.data.b.partial_cmp(&b.data.b).unwrap());
        for (i, value) in lab_data.iter_mut().enumerate() {
            value.data.b = ((i as f64) * 256.0) / len as f64 - 128.0;
        }

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
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::utils::LabData;
    use rand::rngs::ThreadRng;
    use rustc_serialize::json::Json;

    #[derive(Clone)]
    struct MockIndividualData {}

    #[derive(Clone)]
    struct MockIndividual {
        fitness: f64,
        visuals: (f64, f64),
        value: f64,
    }

    impl EvoIndividual<MockIndividualData> for MockIndividual {
        fn new(_ind_data: &MockIndividualData) -> Self {
            MockIndividual {
                fitness: 0.0,
                visuals: (0.0, 0.0),
                value: 0.0,
            }
        }

        fn new_randomised(_ind_data: &MockIndividualData, _rng: &mut ThreadRng) -> Self {
            MockIndividual {
                fitness: 0.0,
                visuals: (0.0, 0.0),
                value: 0.0,
            }
        }

        fn copy_to(&self, ind: &mut Self) {
            ind.value = self.value;
            ind.fitness = self.fitness;
            ind.visuals = self.visuals;
        }

        fn mutate(
            &mut self,
            _ind_data: &MockIndividualData,
            _rng: &mut ThreadRng,
            _mut_prob: f32,
            _mut_amount: f32,
        ) {
            self.value += 1.0;
        }

        fn crossover_to(
            &self,
            _another_ind: &Self,
            dest_ind: &mut Self,
            _ind_data: &MockIndividualData,
            _rng: &mut ThreadRng,
        ) {
            dest_ind.value = (self.value + dest_ind.value) / 2.0;
        }

        fn count_fitness(&mut self, _ind_data: &MockIndividualData) {
            self.fitness = self.value;
        }

        fn get_fitness(&self) -> f64 {
            self.fitness
        }

        fn get_visuals(&self, _ind_data: &MockIndividualData) -> (f64, f64) {
            self.visuals
        }
    }

    type TestPopulation = Population<MockIndividual, MockIndividualData>;

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
        assert_eq!(res, (2, 3));

        let res = TestPopulation::_dual_tournament(&vec![3, 0, 2, 4], &mut vec_ind);
        assert_eq!(res, (3, 4));
    }

    #[test]
    fn test_population() {
        let pop_config = PopulationConfig {
            pop_width: 3,
            pop_height: 3,
            mut_prob: 1.0,
            mut_amount: 2.0,
            crossover_prob: 0.0,
            visualise: false,
            json: Json::Null,
        };

        let mut pop = Population::new(&pop_config, MockIndividualData {});

        // Fill the population with mock individuals
        let mut vec_ind = Vec::new();
        for i in 0..pop.curr_gen_inds.len() {
            vec_ind.push(MockIndividual {
                fitness: i as f64,
                visuals: (i as f64, i as f64),
                value: i as f64,
            });
        }
        pop.curr_gen_inds = vec_ind.clone();

        // Test get_best
        let res = pop.get_best();
        // Pop should return the best individual - the one with the highest fitness value (last in the vector)
        assert_eq!(res.value, vec_ind[pop.curr_gen_inds.len() - 1].value);

        // Test get_at
        assert_eq!(pop.get_at(1, 2).value, 7.0);
        assert_eq!(pop.get_at(2, 0).value, 2.0);

        // Test next_gen
        assert_eq!(pop.get_generation(), 0);
        pop.next_gen();
        assert_eq!(pop.curr_gen_inds[0].value, 7.0);
        assert_eq!(pop.curr_gen_inds[1].value, 8.0);
        assert_eq!(pop.curr_gen_inds[2].value, 9.0);

        assert_eq!(pop.curr_gen_inds[3].value, 7.0);
        assert_eq!(pop.curr_gen_inds[4].value, 8.0);
        assert_eq!(pop.curr_gen_inds[5].value, 9.0);

        assert_eq!(pop.curr_gen_inds[6].value, 9.0);
        assert_eq!(pop.curr_gen_inds[7].value, 9.0);
        assert_eq!(pop.curr_gen_inds[8].value, 9.0);

        assert_eq!(pop.get_generation(), 1);

        // Test _prepare_pop_lab_data
        let lab_data = pop._prepare_pop_lab_data();
        assert_eq!(lab_data.len(), 9);
        assert_eq!(
            lab_data[0].data,
            LabData {
                l: 7.0,
                a: 6.0,
                b: 6.0
            }
        );
        assert_eq!(
            lab_data[1].data,
            LabData {
                l: 8.0,
                a: 7.0,
                b: 7.0
            }
        );
        assert_eq!(
            lab_data[2].data,
            LabData {
                l: 9.0,
                a: 8.0,
                b: 8.0
            }
        );

        assert_eq!(
            lab_data[3].data,
            LabData {
                l: 7.0,
                a: 6.0,
                b: 6.0
            }
        );
        assert_eq!(
            lab_data[4].data,
            LabData {
                l: 8.0,
                a: 7.0,
                b: 7.0
            }
        );
        assert_eq!(
            lab_data[5].data,
            LabData {
                l: 9.0,
                a: 8.0,
                b: 8.0
            }
        );

        assert_eq!(
            lab_data[6].data,
            LabData {
                l: 9.0,
                a: 8.0,
                b: 8.0
            }
        );
        assert_eq!(
            lab_data[7].data,
            LabData {
                l: 9.0,
                a: 8.0,
                b: 8.0
            }
        );
        assert_eq!(
            lab_data[8].data,
            LabData {
                l: 9.0,
                a: 8.0,
                b: 8.0
            }
        );
    }
}
