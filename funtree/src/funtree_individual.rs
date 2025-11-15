use crate::expression::Expression;
use crate::funtree_data::FuntreeIndividualData;
use rand::rngs::SmallRng;
use rand::Rng;
use revo::evo_individual::{EvoIndividual, Visualise};

use image::RgbImage;
use image::{ImageBuffer, Rgb};
use itertools::Itertools;
use plotters::coord::types::RangedCoordf64;
use plotters::prelude::*;

#[derive(Clone)]
pub struct FuntreeIndividual {
    pub fitness: f64,
    genom: Expression,
}

impl FuntreeIndividual {
    pub fn to_string(&self, ind_data: &FuntreeIndividualData) -> String {
        let mut result = String::new();

        result.push_str(&format!("y = {}\n", self.genom.to_string()));

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

    pub fn simplify(&self) -> Self {
        FuntreeIndividual {
            fitness: self.fitness,
            genom: self.genom.simplify(),
        }
    }

    fn _add_data(
        points: &[(f64, f64)],
        chart: &mut ChartContext<
            '_,
            plotters::prelude::BitMapBackend<'_>,
            Cartesian2d<RangedCoordf64, RangedCoordf64>,
        >,
        color: RGBColor,
    ) {
        chart
            .draw_series(
                points
                    .iter()
                    .map(|(x, y)| Circle::new((*x, *y), 2, color.filled())),
            )
            .unwrap();

        chart
            .draw_series(LineSeries::new(points.iter().map(|(x, y)| (*x, *y)), color))
            .unwrap();
    }

    fn _create_rgb_image_from_points(
        pred: &[(f64, f64)],
        gt: &[(f64, f64)],
        width: u32,
        height: u32,
        caption: String,
    ) -> RgbImage {
        let mut buffer: ImageBuffer<Rgb<u8>, Vec<u8>> = ImageBuffer::new(width, height);

        {
            let root = BitMapBackend::with_buffer(&mut buffer, (width, height)).into_drawing_area();
            root.fill(&WHITE).unwrap();

            let (min_x, max_x) = gt.iter().map(|p| p.0).minmax().into_option().unwrap();
            let (min_y, max_y) = gt.iter().map(|p| p.1).minmax().into_option().unwrap();

            let margin_x = (max_x - min_x) * 0.1;
            let margin_y = (max_y - min_y) * 0.1;

            let x_axis = min_x - margin_x..max_x + margin_x;
            let y_axis = min_y - margin_y..max_y + margin_y;

            let mut chart = ChartBuilder::on(&root)
                .caption(caption, ("Arial", 15).into_font())
                .margin(5)
                .x_label_area_size(30)
                .y_label_area_size(30)
                .build_cartesian_2d(x_axis, y_axis)
                .unwrap();

            chart
                .configure_mesh()
                .x_desc("X-Axis")
                .y_desc("Y-Axis")
                .axis_desc_style(("Arial", 15).into_font())
                .draw()
                .unwrap();

            Self::_add_data(gt, &mut chart, BLUE);

            // Truncate the prediction to the chart area
            let pred: Vec<(f64, f64)> = pred
                .iter()
                .filter(|(x, y)| {
                    x.is_finite()
                        && y.is_finite()
                        && *x >= min_x - margin_x * 10.0
                        && *x <= max_x + margin_x * 10.0
                        && *y >= min_y - margin_y * 10.0
                        && *y <= max_y + margin_y * 10.0
                })
                .map(|(x, y)| (*x, *y))
                .collect();

            Self::_add_data(&pred, &mut chart, RED);
        }

        buffer
    }
}

impl EvoIndividual<FuntreeIndividualData> for FuntreeIndividual {
    fn new_randomised(_ind_data: &FuntreeIndividualData, rng: &mut SmallRng) -> Self {
        FuntreeIndividual {
            fitness: 0.0,
            genom: Expression::new_randomised(rng, 5),
        }
    }

    fn mutate(
        &mut self,
        _ind_data: &FuntreeIndividualData,
        rng: &mut SmallRng,
        mut_prob: f32,
        mut_amount: f32,
    ) {
        self.genom.mutate(rng, mut_prob, mut_amount);
    }

    fn crossover(
        &self,
        another_ind: &FuntreeIndividual,
        _ind_data: &FuntreeIndividualData,
        rng: &mut SmallRng,
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

impl Visualise<FuntreeIndividualData> for FuntreeIndividual {
    fn visualise(&self, ind_data: &FuntreeIndividualData) -> RgbImage {
        let mut gt: Vec<(f64, f64)> = Vec::new();
        let mut pred: Vec<(f64, f64)> = Vec::new();

        // Count mean absolute error
        for (i, val) in ind_data.vals.iter().enumerate() {
            let (x, y) = val.as_tuple();
            gt.push((x, y));

            if i < ind_data.vals.len() - 1 {
                let (x_next, _) = ind_data.vals[i + 1].as_tuple();
                let margin = x_next - x;

                // Get 10 values uniformly distributed between x and x_next
                let steps = 10;
                let step_size = margin / (steps as f64);

                let x_values = (0..=steps).map(|i| x + i as f64 * step_size);

                for x_val in x_values {
                    let y_pred = self.genom.evaluate(x_val);
                    if !y_pred.is_nan() {
                        pred.push((x_val, y_pred));
                    }
                }
            } else {
                let y_pred = self.genom.evaluate(x);
                if !y_pred.is_nan() {
                    pred.push((x, y_pred));
                }
            }
        }

        let caption = format!("y = {}", self.genom.simplify().to_string());

        Self::_create_rgb_image_from_points(
            &pred,
            &gt,
            ind_data.plot_width,
            ind_data.plot_height,
            caption,
        )
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
        assert_eq!(ind_1.genom.to_string(), "-(-1.00 + 2.00)");
        assert_eq!(ind_2.genom.to_string(), "-(x * -x)");

        // Get references to nodes
        let ind_1_nodes = ind_1.genom.get_nodes();
        let ind_2_nodes = ind_2.genom.get_nodes();

        // Perform crossover on nodes
        unsafe {
            ind_1_nodes[0].copy_to(ind_2_nodes[1].as_mut());
        }

        // Copy root node from ind_1 to left child of ind_2
        assert_eq!(ind_2.genom.to_string(), "-(-(-1.00 + 2.00) * -x)");
        assert_eq!(ind_2.genom.evaluate(11.0), -11.0);

        // Genom of ind_1 should not change
        assert_eq!(ind_1.genom.to_string(), "-(-1.00 + 2.00)");
        assert_eq!(ind_1.genom.evaluate(10.0), -1.0);

        // Get references to nodes
        let ind_1_nodes = ind_1.genom.get_nodes();
        let ind_2_nodes = ind_2.genom.get_nodes();

        assert_eq!(ind_1_nodes.len(), 3);
        assert_eq!(ind_2_nodes.len(), 5);

        // Check that structure of nodes is the same
        assert_eq!(ind_1_nodes[0].to_string(), ind_2_nodes[1].to_string());
        assert_eq!(ind_1_nodes[1].to_string(), ind_2_nodes[2].to_string());
        assert_eq!(ind_1_nodes[2].to_string(), ind_2_nodes[3].to_string());

        assert_eq!(
            ind_1_nodes[0].evaluate(-10.0),
            ind_2_nodes[1].evaluate(-10.0)
        );
    }
}
