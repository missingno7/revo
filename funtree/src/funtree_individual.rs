use crate::expression::Expression;
use crate::funtree_data::FuntreeIndividualData;
use rand::rngs::ThreadRng;
use rand::Rng;
use revo::evo_individual::EvoIndividual;

#[derive(Clone)]
pub struct FuntreeIndividual {
    pub fitness: f64,
    genom: Expression,
}

impl FuntreeIndividual {
    pub fn as_string(&self, ind_data: &FuntreeIndividualData) -> String {
        let mut result = String::new();

        result.push_str(&format!("y = {}\n", self.genom.as_string()));

        // Count mean absolute error
        for val in ind_data.vals.iter() {
            let (x, y) = val.as_tuple();
            let y_pred = self.genom.evaluate(x);
            result.push_str(&format!(
                " x: {}, y: {}, y_pred: {}, error: {}\n",
                x,
                y,
                y_pred,
                (y - y_pred).abs()
            ));
        }

        result.push_str(&format!("Fitness: {}\n", self.fitness));

        result
    }
}

impl EvoIndividual<FuntreeIndividualData> for FuntreeIndividual {
    fn new(_ind_data: &FuntreeIndividualData) -> Self {
        FuntreeIndividual {
            fitness: 0.0,
            genom: Expression::default(),
        }
    }

    fn new_randomised(_ind_data: &FuntreeIndividualData, rng: &mut ThreadRng) -> Self {
        FuntreeIndividual {
            fitness: 0.0,
            genom: Expression::new_randomised(rng, 5),
        }
    }

    fn copy_to(&self, ind: &mut Self) {
        ind.genom = self.genom.clone();
        ind.fitness = self.fitness;
    }

    fn mutate(
        &mut self,
        _ind_data: &FuntreeIndividualData,
        _rng: &mut ThreadRng,
        _mut_prob: f32,
        _mut_amount: f32,
    ) {
        //self.genom.mutate(rng, mut_prob, mut_amount);
    }

    fn crossover_to(
        &self,
        another_ind: &FuntreeIndividual,
        dest_ind: &mut FuntreeIndividual,
        _ind_data: &FuntreeIndividualData,
        rng: &mut ThreadRng,
    ) {
        // Implement crossover

        if rng.gen_bool(0.5) {
            dest_ind.genom = self.genom.clone();
        } else {
            dest_ind.genom = another_ind.genom.clone();
        }
    }

    fn count_fitness(&mut self, ind_data: &FuntreeIndividualData) {
        // Count mean absolute error
        let mut error = 0.0;
        for val in ind_data.vals.iter() {
            let (x, y) = val.as_tuple();
            let y_pred = self.genom.evaluate(x);
            error += (y - y_pred).abs();
        }

        self.fitness = -error;
    }

    fn get_fitness(&self) -> f64 {
        self.fitness
    }

    fn get_visuals(&self, _ind_data: &FuntreeIndividualData) -> (f64, f64) {
        self.genom.get_visuals()
    }
}
