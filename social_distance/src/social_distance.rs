use image::{ImageBuffer, Rgb, RgbImage};
use imageproc::drawing::draw_hollow_rect_mut;
use imageproc::rect::Rect;
use rand::prelude::ThreadRng;
use rand::Rng;
use revo::config::Config;
use revo::evo_individual::{EvoIndividual, EvoIndividualData, Visualise};
use revo::utils::Coord;

const DEFAULT_SCREEN_WIDTH: u32 = 400;
const DEFAULT_SCREEN_HEIGHT: u32 = 400;
const DEFAULT_N_POINTS: usize = 50;
const DEFAULT_REQUIRED_DISTANCE: u32 = 20;

#[derive(Clone)]
pub struct DistanceIndividualData {
    pub screen_width: u32,
    pub screen_height: u32,
    pub n_points: usize,
    pub required_distance: u32,
}

impl EvoIndividualData for DistanceIndividualData {
    fn from_config(config: &Config) -> Self {
        Self::new(
            config
                .may_get_int("screen_width")
                .unwrap()
                .unwrap_or(DEFAULT_SCREEN_WIDTH),
            config
                .may_get_int("screen_height")
                .unwrap()
                .unwrap_or(DEFAULT_SCREEN_HEIGHT),
            config
                .may_get_int("n_points")
                .unwrap()
                .unwrap_or(DEFAULT_N_POINTS),
            config
                .may_get_int("required_distance")
                .unwrap()
                .unwrap_or(DEFAULT_REQUIRED_DISTANCE),
        )
    }
}

impl DistanceIndividualData {
    pub fn new(
        screen_width: u32,
        screen_height: u32,
        n_points: usize,
        required_distance: u32,
    ) -> Self {
        DistanceIndividualData {
            screen_width,
            screen_height,
            n_points,
            required_distance,
        }
    }
}

#[derive(Clone)]
pub struct DistanceIndividual {
    coords: Vec<Coord>,
}

impl Visualise<DistanceIndividualData> for DistanceIndividual {
    fn visualise(&self, ind_data: &DistanceIndividualData) -> RgbImage {
        let mut img: RgbImage = ImageBuffer::new(ind_data.screen_width, ind_data.screen_height);

        // Draw points
        for i in 0..self.coords.len() {
            let point_color = Rgb([255, ((i * 255) / (self.coords.len())) as u8, 0]);

            draw_hollow_rect_mut(
                &mut img,
                Rect::at(self.coords[i].x - 5, self.coords[i].y - 5).of_size(10, 10),
                point_color,
            );
        }

        img
    }
}

impl EvoIndividual<DistanceIndividualData> for DistanceIndividual {
    fn new_randomised(ind_data: &DistanceIndividualData, rng: &mut ThreadRng) -> Self {
        let mut coords: Vec<Coord> = Vec::new();
        for _ in 0..ind_data.n_points {
            coords.push(Coord {
                x: rng.gen_range(5..ind_data.screen_width - 5) as i32,
                y: rng.gen_range(5..ind_data.screen_height - 5) as i32,
            });
        }

        DistanceIndividual { coords }
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

                if (coord.x + x_mut) < 0 || (coord.x + x_mut) > ind_data.screen_width as i32 {
                    coord.x -= x_mut;
                } else {
                    coord.x += x_mut;
                }

                if (coord.y + y_mut) < 0 || (coord.y + y_mut) > ind_data.screen_height as i32 {
                    coord.y -= y_mut;
                } else {
                    coord.y += y_mut;
                }
            }
        }
    }

    fn crossover(
        &self,
        another_ind: &DistanceIndividual,
        _ind_data: &DistanceIndividualData,
        rng: &mut ThreadRng,
    ) -> DistanceIndividual {
        let mut dest_ind = self.clone();
        for i in 0..self.coords.len() {
            let ratio: f32 = rng.gen_range(0.0..1.0);
            dest_ind.coords[i].x = ((self.coords[i].x as f32 * ratio)
                + (another_ind.coords[i].x as f32 * (1.0 - ratio)))
                as i32;

            dest_ind.coords[i].y = ((self.coords[i].y as f32 * ratio)
                + (another_ind.coords[i].y as f32 * (1.0 - ratio)))
                as i32;
        }
        dest_ind
    }

    fn count_fitness(&self, ind_data: &DistanceIndividualData) -> f64 {
        let mut fitness = 0.0;

        let center_x: i32 = (ind_data.screen_width / 2) as i32;
        let center_y: i32 = (ind_data.screen_height / 2) as i32;
        let center: Coord = Coord {
            x: center_x,
            y: center_y,
        };

        for i in 0..self.coords.len() {
            let mut closest_dist = std::i64::MAX;

            for j in 0..self.coords.len() {
                if i == j {
                    continue;
                }

                let distance = Coord::distance_euclid(&self.coords[i], &self.coords[j]);

                if distance < closest_dist {
                    closest_dist = distance;
                }
            }

            fitness -= i64::abs(
                closest_dist
                    - (ind_data.required_distance as i64 * ind_data.required_distance as i64),
            ) as f64;
            fitness -= Coord::distance_euclid(&self.coords[i], &center) as f64
                / ((self.coords.len() as f64) * 1.0);
        }
        fitness
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
