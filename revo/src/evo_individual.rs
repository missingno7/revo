use crate::config::Config;
use image::RgbImage;
use rand::rngs::ThreadRng;

pub trait EvoIndividualData: Send + Sync {
    fn from_config(config: &Config) -> Self;
}

pub trait EvoIndividual<IndividualData>: Send + Sync + Clone {
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
    fn count_fitness(&self, ind_data: &IndividualData) -> f64;

    // Get the A and B values of the individual for visualisation
    fn get_visuals(&self, ind_data: &IndividualData) -> (f64, f64);
}

pub trait Visualise<IndividualData> {
    fn visualise(&self, ind_data: &IndividualData) -> RgbImage;
}
