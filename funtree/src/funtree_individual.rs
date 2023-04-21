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
    fn new_randomised(_ind_data: &FuntreeIndividualData, rng: &mut ThreadRng) -> Self {
        FuntreeIndividual {
            fitness: 0.0,
            genom: Expression::new_randomised(rng, 5),
        }
    }

    fn mutate(
        &mut self,
        _ind_data: &FuntreeIndividualData,
        rng: &mut ThreadRng,
        mut_prob: f32,
        mut_amount: f32,
    ) {
        self.genom.mutate(rng, mut_prob, mut_amount);
    }

    fn crossover(
        &self,
        another_ind: &FuntreeIndividual,
        _ind_data: &FuntreeIndividualData,
        rng: &mut ThreadRng,
    ) -> FuntreeIndividual {
        // Select random source genom and copy the other not selected genom to destination genom
        let (source_genom, dest_ind) = if rng.gen_bool(0.5) {
            (&another_ind, self.clone())
        } else {
            (&self, another_ind.clone())
        };

        // Choose random node in source and destination genom
        let source_genom_it = source_genom.genom.choose_random_node(rng);
        let dest_genom_it = dest_ind.genom.choose_random_node(rng);

        // Copy data from selected source node to selected destination node
        unsafe { source_genom_it.copy_to(dest_genom_it.as_mut()) }

        dest_ind
    }

    fn count_fitness(&mut self, ind_data: &FuntreeIndividualData) {
        // Count mean absolute error
        let mut error = 0.0;
        for val in ind_data.vals.iter() {
            let (x, y) = val.as_tuple();
            let y_pred = self.genom.evaluate(x);

            // Handling cases like division by zero
            if y_pred.is_nan() {
                self.fitness = -f64::INFINITY;
                return;
            }

            // Sum of squared errors
            let err = y_pred - y;
            error += err * err;
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

#[cfg(test)]
mod tests {
    use super::*;
    use crate::leaf::LeafType;
    use crate::operation::OperationType;

    #[test]
    fn crossover() {
        // Create individuals
        let left_add_1 = Expression::new_leaf(1.0, LeafType::Constant, true);
        let right_add_1 = Expression::new_leaf(2.0, LeafType::Constant, false);
        let add_1 =
            Expression::new_operation(left_add_1, right_add_1, OperationType::Addition, true);
        let ind_1 = FuntreeIndividual {
            fitness: 0.0,
            genom: add_1,
        };

        let left_mul_1 = Expression::new_leaf(3.0, LeafType::Variable, false);
        let right_mul_1 = Expression::new_leaf(4.0, LeafType::Variable, true);
        let mul_1 =
            Expression::new_operation(left_mul_1, right_mul_1, OperationType::Multiplication, true);
        let ind_2 = FuntreeIndividual {
            fitness: 0.0,
            genom: mul_1,
        };

        // Check if nodes are correct
        assert_eq!(ind_1.genom.as_string(), "-(-1.00 + 2.00)");
        assert_eq!(ind_2.genom.as_string(), "-(x * -x)");

        // Get references to nodes
        let ind_1_nodes = ind_1.genom.get_nodes();
        let ind_2_nodes = ind_2.genom.get_nodes();

        // Perform crossover on nodes
        unsafe {
            ind_1_nodes[0].copy_to(ind_2_nodes[1].as_mut());
        }

        // Copy root node from ind_1 to left child of ind_2
        assert_eq!(ind_2.genom.as_string(), "-(-(-1.00 + 2.00) * -x)");
        assert_eq!(ind_2.genom.evaluate(11.0), -11.0);

        // Genom of ind_1 should not change
        assert_eq!(ind_1.genom.as_string(), "-(-1.00 + 2.00)");
        assert_eq!(ind_1.genom.evaluate(10.0), -1.0);

        // Get references to nodes
        let ind_1_nodes = ind_1.genom.get_nodes();
        let ind_2_nodes = ind_2.genom.get_nodes();

        assert_eq!(ind_1_nodes.len(), 3);
        assert_eq!(ind_2_nodes.len(), 5);

        // Check that structure of nodes is the same
        assert_eq!(ind_1_nodes[0].as_string(), ind_2_nodes[1].as_string());
        assert_eq!(ind_1_nodes[1].as_string(), ind_2_nodes[2].as_string());
        assert_eq!(ind_1_nodes[2].as_string(), ind_2_nodes[3].as_string());

        assert_eq!(
            ind_1_nodes[0].evaluate(-10.0),
            ind_2_nodes[1].evaluate(-10.0)
        );
    }
}
