use super::evo_individual::EvoIndividual;
use crate::revo::pop_config::PopulationConfig;
use rand::Rng;


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

impl<Individual: EvoIndividual<IndividualData>, IndividualData> Population<Individual, IndividualData> {
    // Another associated function, taking two arguments:
    pub fn new(pop_config: PopulationConfig, ind_data: IndividualData) -> Population<Individual, IndividualData> {
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
        let mut rng = rand::thread_rng();
        for i in 0..self.curr_gen_inds.len() {
            if rng.gen_range(0.0..1.0) < self.crossover_prob
            {
                // Do crossover
                let (first_ind, second_ind) = self.dual_tournament_l5(i);
                self.curr_gen_inds[first_ind].crossover_to(&self.curr_gen_inds[second_ind], &mut self.next_gen_inds[i], &self.ind_data, &mut rng)
            } else {
                // Just mutate
                self.curr_gen_inds[self.tournament_l5(i)].copy_to(&mut self.next_gen_inds[i]);
                self.next_gen_inds[i].mutate(&self.ind_data, &mut rng, self.mut_prob, self.mut_amount);
            }
            self.next_gen_inds[i].count_fitness(&self.ind_data);
        }

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

    pub fn get_generation(&self) -> usize
    {
        self.i_generation
    }

    // Private methods

    fn tournament_l5(&self, i: usize) -> usize
    {
        let x: usize = i % self.pop_width;
        let y: usize = i / self.pop_width;

        let mut best_i = i;

        let row_start_index = y * self.pop_width;
        let l_i = row_start_index + ((x + self.pop_width - 1) % self.pop_width);
        let r_i = row_start_index + ((x + 1) % self.pop_width);

        let column_start_index = x % self.pop_width;
        let u_i =
            ((y + self.pop_height - 1) % self.pop_height) * self.pop_width + column_start_index;
        let d_i = ((y + 1) % self.pop_height) * self.pop_width + column_start_index;

        // Left
        if self.curr_gen_inds[l_i].get_fitness() > self.curr_gen_inds[best_i].get_fitness() {
            best_i = l_i;
        }
        // Right
        if self.curr_gen_inds[r_i].get_fitness() > self.curr_gen_inds[best_i].get_fitness() {
            best_i = r_i;
        }

        // Up
        if self.curr_gen_inds[u_i].get_fitness() > self.curr_gen_inds[best_i].get_fitness() {
            best_i = u_i;
        }

        // Down
        if self.curr_gen_inds[d_i].get_fitness() > self.curr_gen_inds[best_i].get_fitness() {
            best_i = d_i;
        }

        best_i
    }


    fn dual_tournament_l5(&self, i: usize) -> (usize, usize)
    {
        let x: usize = i % self.pop_width;
        let y: usize = i / self.pop_width;

        let row_start_index = y * self.pop_width;
        let l_i = row_start_index + ((x + self.pop_width - 1) % self.pop_width);
        let r_i = row_start_index + ((x + 1) % self.pop_width);

        let column_start_index = x % self.pop_width;
        let u_i =
            ((y + self.pop_height - 1) % self.pop_height) * self.pop_width + column_start_index;
        let d_i = ((y + 1) % self.pop_height) * self.pop_width + column_start_index;


        let mut best_i = i;
        let mut second_best_i = l_i;

        if self.curr_gen_inds[second_best_i].get_fitness() > self.curr_gen_inds[best_i].get_fitness()
        {
            std::mem::swap(&mut best_i, &mut second_best_i);
        }


        if self.curr_gen_inds[r_i].get_fitness() > self.curr_gen_inds[best_i].get_fitness()
        {
            best_i = r_i;
        } else if self.curr_gen_inds[r_i].get_fitness() > self.curr_gen_inds[second_best_i].get_fitness()
        {
            second_best_i = r_i;
        }


        if self.curr_gen_inds[u_i].get_fitness() > self.curr_gen_inds[best_i].get_fitness()
        {
            best_i = u_i;
        } else if self.curr_gen_inds[u_i].get_fitness() > self.curr_gen_inds[second_best_i].get_fitness()
        {
            second_best_i = u_i;
        }


        if self.curr_gen_inds[d_i].get_fitness() > self.curr_gen_inds[best_i].get_fitness()
        {
            best_i = d_i;
        } else if self.curr_gen_inds[d_i].get_fitness() > self.curr_gen_inds[second_best_i].get_fitness()
        {
            second_best_i = d_i;
        }


        (best_i, second_best_i)
    }
}
