use image::RgbImage;
use rand::rngs::ThreadRng;

pub trait EvoIndividual<IndividualData>: Send + Sync {
    // Create a new individual with default values
    fn new(ind_data: &IndividualData) -> Self;

    // Create a new individual with randomised values
    fn new_randomised(ind_data: &IndividualData, rng: &mut ThreadRng) -> Self;

    // Mutate the genome of the individual
    fn mutate(
        &mut self,
        ind_data: &IndividualData,
        rng: &mut ThreadRng,
        mut_prob: f32,
        mut_amount: f32,
    );

    // Return new Individual with the genome that is a crossover of two individuals
    fn crossover(&self, another_ind: &Self, ind_data: &IndividualData, rng: &mut ThreadRng)
        -> Self;

    // Count the fitness of the individual
    fn count_fitness(&mut self, ind_data: &IndividualData);

    // Get the fitness of the individual
    fn get_fitness(&self) -> f64;

    // Get the A and B values of the individual for visualisation
    fn get_visuals(&self, ind_data: &IndividualData) -> (f64, f64);
}

pub trait Visualise<IndividualData> {
    fn visualise(&self, ind_data: &IndividualData) -> RgbImage;
}
