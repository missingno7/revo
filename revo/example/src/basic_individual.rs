use rand::prelude::ThreadRng;
use rand::Rng;
use revo::evo_individual::EvoIndividual;

pub struct BasicIndividualData {
    value: f64,
}

impl Default for BasicIndividualData {
    fn default() -> Self {
        BasicIndividualData { value: 0.0 }
    }
}

#[derive(Clone)]
pub struct BasicIndividual {
    fitness: f64,
    foo: f64,
    bar: f64,
}

impl EvoIndividual<BasicIndividualData> for BasicIndividual {
    fn new(ind_data: &BasicIndividualData) -> Self {
        BasicIndividual {
            fitness: 0.0,
            foo: ind_data.value,
            bar: ind_data.value,
        }
    }

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

    fn crossover_to(
        &self,
        another_ind: &BasicIndividual,
        dest_int: &mut BasicIndividual,
        _ind_data: &BasicIndividualData,
        rng: &mut ThreadRng,
    ) {
        let ratio = rng.gen_range(0.0..1.0);

        dest_int.foo = self.foo * ratio + another_ind.foo * (1.0 - ratio);
        dest_int.bar = self.bar * ratio + another_ind.bar * (1.0 - ratio);
    }

    fn copy_to(&self, ind: &mut Self) {
        ind.fitness = self.fitness;
        ind.foo = self.foo;
        ind.bar = self.bar;
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
