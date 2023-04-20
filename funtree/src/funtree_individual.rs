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

    fn unsafe_copy(source_expr: &Expression, dest_expr: &Expression) {
        // Copy random node from source genom to random node in  destination genom
        // I am using unsafe code because I can't get mutable reference to random node in destination genom
        unsafe {
            // Cast immutable reference to raw pointer
            let ptr_x: *const Expression = dest_expr as *const Expression;
            // Cast raw pointer to mutable pointer
            let ptr_x_mut: *mut Expression = ptr_x as *mut Expression;
            // Copy internal values from source expression to destination expression
            (*ptr_x_mut).copy_from(source_expr);
        }
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
        rng: &mut ThreadRng,
        mut_prob: f32,
        _mut_amount: f32,
    ) {
        self.genom.mutate(rng, mut_prob);
    }

    fn crossover_to(
        &self,
        another_ind: &FuntreeIndividual,
        dest_ind: &mut FuntreeIndividual,
        _ind_data: &FuntreeIndividualData,
        rng: &mut ThreadRng,
    ) {
        // Select random source genom and copy the other not selected genom to destination genom
        let (mut source_genom_it, mut dest_genom_it) = if rng.gen_bool(0.5) {
            dest_ind.genom = self.genom.clone();
            (&another_ind.genom, &dest_ind.genom)
        } else {
            dest_ind.genom = another_ind.genom.clone();
            (&self.genom, &dest_ind.genom)
        };

        // Choose random node in source and destination genom
        source_genom_it = source_genom_it.choose_random_node(rng);
        dest_genom_it = dest_genom_it.choose_random_node(rng);

        // Unsafe function that actually modifies destination genom even though it is immutable
        Self::unsafe_copy(source_genom_it, dest_genom_it);
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

#[cfg(test)]
mod tests {
    use super::*;
    use crate::leaf::LeafType;
    use crate::operation::OperationType;

    #[test]
    fn crossover() {
        let ind_data = FuntreeIndividualData::default();

        // Create individuals
        let mut ind_1 = FuntreeIndividual::new(&ind_data);
        let left_add_1 = Expression::new_leaf(1.0, LeafType::Constant, true);
        let right_add_1 = Expression::new_leaf(2.0, LeafType::Constant, false);
        let add_1 =
            Expression::new_operation(left_add_1, right_add_1, OperationType::Addition, true);
        ind_1.genom = add_1;

        let mut ind_2 = FuntreeIndividual::new(&ind_data);
        let left_mul_1 = Expression::new_leaf(3.0, LeafType::Variable, false);
        let right_mul_1 = Expression::new_leaf(4.0, LeafType::Variable, true);
        let mul_1 =
            Expression::new_operation(left_mul_1, right_mul_1, OperationType::Multiplication, true);
        ind_2.genom = mul_1;

        // Check if nodes are correct
        assert_eq!(ind_1.genom.as_string(), "-(-1.00 + 2.00)");
        assert_eq!(ind_2.genom.as_string(), "-(x * -x)");

        // Get references to nodes
        let ind_1_nodes = ind_1.genom.get_nodes();
        let ind_2_nodes = ind_2.genom.get_nodes();

        // Perform crossover on nodes
        FuntreeIndividual::unsafe_copy(&ind_1_nodes[0], &ind_2_nodes[1]);

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

        // Check if pointers are different even though node structure is the same
        assert!(!std::ptr::eq(&ind_1_nodes[0].expr, &ind_2_nodes[1].expr));
        assert!(!std::ptr::eq(&ind_1_nodes[1].expr, &ind_2_nodes[2].expr));
        assert!(!std::ptr::eq(&ind_1_nodes[2].expr, &ind_2_nodes[3].expr));
    }
}
