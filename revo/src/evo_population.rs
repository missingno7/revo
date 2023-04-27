use crate::config::Config;
use crate::evo_individual::{EvoIndividual, EvoIndividualData};
use image::RgbImage;

pub trait EvoPopulation<Individual, IndividualData>
where
    Individual: EvoIndividual<IndividualData> + Send + Sync + Clone,
    IndividualData: EvoIndividualData,
{
    fn get_at(&self, x: usize, y: usize) -> &Individual;

    fn get_width(&self) -> usize;

    fn get_height(&self) -> usize;

    // Function returns the number of the current generation
    fn get_generation(&self) -> usize;

    // Function creates a new population with randomised individuals and counts their fitness
    fn new(config: &Config) -> Self;

    // Function moves the population to the next generation
    // It does selection, crossover/mutation and counts fitness for each individual
    fn next_gen(&mut self);

    // Function returns the best individual in the current generation
    fn get_best(&self) -> &Individual;

    // Function creates a visualization of the current generation in the form of an PNG image
    // It maps the fitness (L) and visual attributes (A, B) of each individual
    fn visualise(&self) -> RgbImage;

    // Function returns the data for individuals
    fn get_individual_data(&self) -> &IndividualData;
}
