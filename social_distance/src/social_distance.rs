use image::{ImageBuffer, Rgb, RgbImage};
use imageproc::drawing::draw_hollow_rect_mut;
use imageproc::rect::Rect;
use rand::prelude::ThreadRng;
use rand::Rng;
use revo::evo_individual::EvoIndividual;
use revo::utils::Coord;

#[derive(Clone)]
pub struct DistanceIndividualData {
    pub screen_width: u32,
    pub screen_height: u32,
    pub n_points: usize,
    pub required_distance: u32,
}

#[derive(Clone)]
pub struct DistanceIndividual {
    fitness: f64,
    coords: Vec<Coord>,
}

impl DistanceIndividual {
    pub fn visualise(&self, output_filename: &str, ind_data: &DistanceIndividualData) {
        let mut img: RgbImage = ImageBuffer::new(ind_data.screen_width, ind_data.screen_height);

        // Draw points
        for i in 0..self.coords.len() {
            let point_color = Rgb([255, ((i * 255) / (self.coords.len())) as u8, 0]);

            draw_hollow_rect_mut(
                &mut img,
                Rect::at(self.coords[i].x as i32 - 5, self.coords[i].y as i32 - 5).of_size(10, 10),
                point_color,
            );
        }

        img.save(output_filename).unwrap();
    }

    fn distance(x1: i32, y1: i32, x2: i32, y2: i32) -> i64 {
        let x = x2 as i64 - x1 as i64;
        let y = y2 as i64 - y1 as i64;
        (x * x) + (y * y)
    }
}

impl EvoIndividual<DistanceIndividualData> for DistanceIndividual {
    fn new(ind_data: &DistanceIndividualData) -> Self {
        let mut coords: Vec<Coord> = Vec::new();

        for _ in 0..ind_data.n_points {
            coords.push(Coord { x: 100, y: 100 });
        }

        DistanceIndividual {
            coords,
            fitness: 0.0,
        }
    }

    fn new_randomised(ind_data: &DistanceIndividualData, rng: &mut ThreadRng) -> Self {
        let mut coords: Vec<Coord> = Vec::new();
        for _ in 0..ind_data.n_points {
            coords.push(Coord {
                x: rng.gen_range(5..ind_data.screen_width - 5) as i32,
                y: rng.gen_range(5..ind_data.screen_height - 5) as i32,
            });
        }

        DistanceIndividual {
            coords,
            fitness: 0.0,
        }
    }

    fn copy_to(&self, ind: &mut Self) {
        for i in 0..self.coords.len() {
            ind.coords[i] = self.coords[i];
        }
    }

    fn mutate(
        &mut self,
        ind_data: &DistanceIndividualData,
        rng: &mut ThreadRng,
        mut_prob: f32,
        mut_amount: f32,
    ) {
        for coord in &mut self.coords {
            if rng.gen_range(0.0..1.0) < mut_prob {
                let x_mut = rng.gen_range(-mut_amount..mut_amount) as i32;
                let y_mut = rng.gen_range(-mut_amount..mut_amount) as i32;

                if (coord.x as i32 + x_mut) < 0
                    || (coord.x as i32 + x_mut) > ind_data.screen_width as i32
                {
                    coord.x = coord.x as i32 - x_mut;
                } else {
                    coord.x = coord.x as i32 + x_mut;
                }

                if (coord.y as i32 + y_mut) < 0
                    || (coord.y as i32 + y_mut) > ind_data.screen_height as i32
                {
                    coord.y = coord.y as i32 - y_mut;
                } else {
                    coord.y = coord.y as i32 + y_mut;
                }
            }
        }
    }

    fn crossover_to(
        &self,
        another_ind: &DistanceIndividual,
        dest_int: &mut DistanceIndividual,
        _ind_data: &DistanceIndividualData,
        rng: &mut ThreadRng,
    ) {
        for i in 0..self.coords.len() {
            let ratio: f32 = rng.gen_range(0.0..1.0);
            dest_int.coords[i].x = ((self.coords[i].x as f32 * ratio)
                + (another_ind.coords[i].x as f32 * (1.0 - ratio)))
                as i32;

            dest_int.coords[i].y = ((self.coords[i].y as f32 * ratio)
                + (another_ind.coords[i].y as f32 * (1.0 - ratio)))
                as i32;
        }
    }

    fn count_fitness(&mut self, ind_data: &DistanceIndividualData) {
        self.fitness = 0.0;

        let center_x: i32 = (ind_data.screen_width / 2) as i32;
        let center_y: i32 = (ind_data.screen_height / 2) as i32;

        for i in 0..self.coords.len() {
            let mut closest_dist = std::i64::MAX;

            let x1 = self.coords[i].x;
            let y1 = self.coords[i].y;

            for j in 0..self.coords.len() {
                if i == j {
                    continue;
                }

                let x2 = self.coords[j].x;
                let y2 = self.coords[j].y;
                let distance = DistanceIndividual::distance(x1, y1, x2, y2);

                if distance < closest_dist {
                    closest_dist = distance;
                }
            }

            self.fitness -= i64::abs(
                closest_dist
                    - (ind_data.required_distance as i64 * ind_data.required_distance as i64),
            ) as f64;
            self.fitness -= DistanceIndividual::distance(x1, y1, center_x, center_y) as f64
                / ((self.coords.len() as f64) * 1.0);
        }
    }

    fn get_fitness(&self) -> f64 {
        self.fitness
    }

    fn get_visuals(&self, _ind_data: &DistanceIndividualData) -> (f64, f64) {
        let mut a: f64 = 0.0;
        let mut b: f64 = 0.0;

        for i in 0..self.coords.len() {
            a += self.coords[i].x as f64;
            b += self.coords[i].y as f64;
        }

        (a, b)
    }
}
