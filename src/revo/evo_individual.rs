use rand::rngs::ThreadRng;

pub trait EvoIndividual<IndividualData> {
    fn new() -> Self;
    fn new_randomised(ind_data: &IndividualData, rng: &mut ThreadRng) -> Self;

    fn copy_to(&self, ind: &mut Self);
    fn clone(&self) -> Self;
    fn mutate(&mut self, ind_data: &IndividualData, rng: &mut ThreadRng, mut_prob: f32, mut_amount: f32);

    fn count_fitness(&mut self, ind_data: &IndividualData);

    fn get_fitness(&self) -> f64;
}
