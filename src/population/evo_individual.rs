pub trait EvoIndividual {
    fn new() -> Self;
    fn new_randomised() -> Self;

    fn copy_to(&self, ind: &mut Self);
    fn clone(&self) -> Self;
    fn mutate(&mut self);

    fn count_fitness(&mut self);

    fn get_fitness(&self) -> f64;
}
