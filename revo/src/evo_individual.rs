use rand::rngs::ThreadRng;

pub trait EvoIndividual<IndividualData>: Send + Sync {
    fn new(ind_data: &IndividualData) -> Self;
    fn new_randomised(ind_data: &IndividualData, rng: &mut ThreadRng) -> Self;

    fn copy_to(&self, ind: &mut Self);
    fn clone(&self) -> Self;
    fn mutate(
        &mut self,
        ind_data: &IndividualData,
        rng: &mut ThreadRng,
        mut_prob: f32,
        mut_amount: f32,
    );

    fn crossover_to(
        &self,
        another_ind: &Self,
        dest_int: &mut Self,
        ind_data: &IndividualData,
        rng: &mut ThreadRng,
    );

    fn count_fitness(&mut self, ind_data: &IndividualData);

    fn get_fitness(&self) -> f64;

    fn get_visuals(&self, ind_data: &IndividualData) -> (f64, f64);
}
