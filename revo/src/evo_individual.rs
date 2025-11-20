use crate::config::Config;
use image::RgbImage;
use rand::rngs::SmallRng;

pub trait EvoIndividualData: Send + Sync {
    fn from_config(config: &Config) -> Self;
}

pub trait EvoIndividual<Data>: Sized + Clone + Send + Sync {
    fn new_randomised(data: &Data, rng: &mut SmallRng) -> Self;

    /// Default mutate uses `mutate_into`.
    /// Implementors MUST override at least one of `mutate` or `mutate_into`
    /// to avoid infinite recursion.
    fn mutate(&mut self, data: &Data, rng: &mut SmallRng, mut_prob: f32, mut_amount: f32) {
        // Default: delegate to mutate_into via a temporary clone.
        // If `mutate_into` is overridden, this will use the optimized path.
        let tmp = self.clone();
        tmp.mutate_into(self, data, rng, mut_prob, mut_amount);
        *self = tmp;
    }

    /// Default crossover uses `crossover_into`.
    /// Implementors MUST override at least one of `crossover`
    /// or `crossover_into` to avoid infinite recursion.
    fn crossover(&self, other: &Self, data: &Data, rng: &mut SmallRng) -> Self {
        // Default: delegate to `crossover_into` on a cloned target.
        let mut out = self.clone();
        self.crossover_into(other, &mut out, data, rng);
        out
    }

    fn count_fitness(&mut self, data: &Data);
    fn get_fitness(&self) -> f64;
    fn get_visuals(&self, data: &Data) -> (f64, f64);

    /// Default clone + mutate that can be overridden
    fn mutate_into(
        &self,
        target: &mut Self,
        data: &Data,
        rng: &mut SmallRng,
        mut_prob: f32,
        mut_amount: f32,
    ) {
        *target = self.clone();
        target.mutate(data, rng, mut_prob, mut_amount);
    }

    /// Default crossover into that can be overridden
    fn crossover_into(&self, other: &Self, target: &mut Self, data: &Data, rng: &mut SmallRng) {
        *target = self.crossover(other, data, rng);
    }
}

pub trait Visualise<IndividualData> {
    fn visualise(&self, ind_data: &IndividualData) -> RgbImage;
}
