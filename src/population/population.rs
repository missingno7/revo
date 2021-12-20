use super::evo_individual::EvoIndividual;

pub struct Population<Individual> {
    curr_gen: Vec<Individual>,
    next_gen: Vec<Individual>,
    pop_width: usize,
    pop_height: usize,
    i_generation: usize,
}

impl<Individual: EvoIndividual> Population<Individual> {
    // Another associated function, taking two arguments:
    pub fn new(pop_width: usize, pop_height: usize) -> Population<Individual> {
        let size = pop_width * pop_height;
        let mut curr_gen: Vec<Individual> = Vec::with_capacity(size);
        let mut next_gen: Vec<Individual> = Vec::with_capacity(size);

        for _ in 0..size {
            let mut curr_gen_ind = Individual::new_randomised();
            curr_gen_ind.count_fitness();
            curr_gen.push(curr_gen_ind);
            next_gen.push(Individual::new());
        }

        Population {
            curr_gen,
            next_gen,
            pop_width,
            pop_height,
            i_generation: 0,
        }
    }

    pub fn next_gen(&mut self) {
        for i in 0..self.curr_gen.len() {
            self.curr_gen[self.tournament_l5(i)].copy_to(&mut self.next_gen[i]);
            self.next_gen[i].mutate();
        }

        // Advance to next generation
        std::mem::swap(&mut self.curr_gen, &mut self.next_gen);
        self.i_generation += 1;
    }

    pub fn get_best(&self) -> Individual {
        let mut best_ind = &self.curr_gen[0];

        for i in 1..self.curr_gen.len() {
            if self.curr_gen[i].get_fitness() > self.next_gen[i].get_fitness() {
                best_ind = &self.curr_gen[i];
            }
        }

        best_ind.clone()
    }

    fn tournament_l5(&self, i: usize) -> usize {
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


        //println!("i{} l{} r{} u{} d{}",i, l_i, r_i, u_i, d_i);

        // Left
        if self.curr_gen[l_i].get_fitness() > self.curr_gen[best_i].get_fitness() {
            best_i = l_i;
        }
        // Right
        if self.curr_gen[r_i].get_fitness() > self.curr_gen[best_i].get_fitness() {
            best_i = r_i;
        }

        // Up
        if self.curr_gen[u_i].get_fitness() > self.curr_gen[best_i].get_fitness() {
            best_i = u_i;
        }

        // Down
        if self.curr_gen[d_i].get_fitness() > self.curr_gen[best_i].get_fitness() {
            best_i = d_i;
        }

        best_i
    }
}
