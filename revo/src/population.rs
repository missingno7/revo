use super::evo_individual::EvoIndividual;
use crate::pop_config::PopulationConfig;
use rand::Rng;
use rayon::prelude::*;

pub struct Population<Individual, IndividualData> {
    curr_gen_inds: Vec<Individual>,
    next_gen_inds: Vec<Individual>,
    pop_width: usize,
    pop_height: usize,
    mut_prob: f32,
    mut_amount: f32,
    crossover_prob: f32,
    i_generation: usize,

    ind_data: IndividualData,
}

impl<Individual: EvoIndividual<IndividualData> + Send + Sync, IndividualData: Sync>
    Population<Individual, IndividualData>
{
    // Another associated function, taking two arguments:
    pub fn new(
        pop_config: PopulationConfig,
        ind_data: IndividualData,
    ) -> Population<Individual, IndividualData> {
        let size = pop_config.pop_width * pop_config.pop_height;
        let mut curr_gen_inds: Vec<Individual> = Vec::with_capacity(size);
        let mut next_gen_inds: Vec<Individual> = Vec::with_capacity(size);

        let mut rng = rand::thread_rng();
        for _ in 0..size {
            let mut curr_gen_ind = Individual::new_randomised(&ind_data, &mut rng);
            curr_gen_ind.count_fitness(&ind_data);
            curr_gen_inds.push(curr_gen_ind);
            next_gen_inds.push(Individual::new(&ind_data));
        }

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

    pub fn next_gen(&mut self) {
        self.next_gen_inds
            .par_iter_mut()
            .enumerate()
            .take(self.curr_gen_inds.len())
            .for_each(|(i, res)| {
                let mut rng = rand::thread_rng();
                let indices = Self::l5_selection(i, self.pop_width, self.pop_height);

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
                    // Just mutate
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

    pub fn get_best(&self) -> Individual {
        let mut best_ind = &self.curr_gen_inds[0];

        for i in 1..self.curr_gen_inds.len() {
            if self.curr_gen_inds[i].get_fitness() > best_ind.get_fitness() {
                best_ind = &self.curr_gen_inds[i];
            }
        }

        best_ind.clone()
    }

    pub fn get_generation(&self) -> usize {
        self.i_generation
    }

    // Private methods
    fn l5_selection(i: usize, pop_width: usize, pop_height: usize) -> Vec<usize> {
        let x: usize = i % pop_width;
        let y: usize = i / pop_width;

        let row_start_index = y * pop_width;
        let l_i = row_start_index + ((x + pop_width - 1) % pop_width);
        let r_i = row_start_index + ((x + 1) % pop_width);

        let column_start_index = x % pop_width;
        let u_i = ((y + pop_height - 1) % pop_height) * pop_width + column_start_index;
        let d_i = ((y + 1) % pop_height) * pop_width + column_start_index;

        vec![i, l_i, r_i, u_i, d_i]
    }

    fn single_tournament(indices: &[usize], curr_gen_inds: &[Individual]) -> usize {
        let mut best_i = indices[0];

        for &index in indices.iter().skip(1) {
            if curr_gen_inds[index].get_fitness() > curr_gen_inds[best_i].get_fitness() {
                best_i = index;
            }
        }

        best_i
    }

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
