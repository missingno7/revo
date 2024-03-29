use image::RgbImage;
use rand::prelude::ThreadRng;
use rand::Rng;
use revo::config::Config;
use revo::evo_individual::{EvoIndividual, EvoIndividualData, Visualise};
use std::fmt;

pub struct BasicIndividualData {
    value: f64,
}

const DEFAULT_VALUE: f64 = 0.0;

impl EvoIndividualData for BasicIndividualData {
    fn from_config(config: &Config) -> Self {
        BasicIndividualData {
            value: config
                .may_get_float("value")
                .unwrap()
                .unwrap_or(DEFAULT_VALUE),
        }
    }
}

#[derive(Clone)]
pub struct BasicIndividual {
    fitness: f64,
    foo: f64,
    bar: f64,
}

impl EvoIndividual<BasicIndividualData> for BasicIndividual {
    fn new_randomised(ind_data: &BasicIndividualData, rng: &mut ThreadRng) -> Self {
        BasicIndividual {
            fitness: 0.0,
            foo: ind_data.value + rng.gen_range(0.0..10.0),
            bar: ind_data.value + rng.gen_range(0.0..10.0),
        }
    }

    fn mutate(
        &mut self,
        _ind_data: &BasicIndividualData,
        rng: &mut ThreadRng,
        mut_prob: f32,
        mut_amount: f32,
    ) {
        if rng.gen_range(0.0..1.0) < mut_prob {
            self.foo += rng.gen_range(-mut_amount as f64..mut_amount as f64);
        }

        if rng.gen_range(0.0..1.0) < mut_prob {
            self.bar += rng.gen_range(-mut_amount as f64..mut_amount as f64);
        }
    }

    fn crossover(
        &self,
        another_ind: &BasicIndividual,
        _ind_data: &BasicIndividualData,
        rng: &mut ThreadRng,
    ) -> BasicIndividual {
        let ratio = rng.gen_range(0.0..1.0);

        BasicIndividual {
            fitness: 0.0,
            foo: self.foo * ratio + another_ind.foo * (1.0 - ratio),
            bar: self.bar * ratio + another_ind.bar * (1.0 - ratio),
        }
    }

    fn count_fitness(&mut self, _ind_data: &BasicIndividualData) {
        self.fitness = (self.foo - self.bar).abs();
    }
    fn get_fitness(&self) -> f64 {
        self.fitness
    }

    fn get_visuals(&self, _ind_data: &BasicIndividualData) -> (f64, f64) {
        (self.foo, self.bar)
    }
}

impl fmt::Display for BasicIndividual {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "foo: {}, bar: {}", self.foo, self.bar)
    }
}

impl<IndividualData> Visualise<IndividualData> for BasicIndividual {
    fn visualise(&self, _: &IndividualData) -> RgbImage {
        RgbImage::new(1, 1)
    }
}
