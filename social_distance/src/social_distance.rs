use rand::Rng;
use rand::prelude::ThreadRng;
use revo::evo_individual::EvoIndividual;
use image::{RgbImage, ImageBuffer, Rgb};
use imageproc::drawing::{draw_hollow_rect_mut};
use imageproc::rect::Rect;


pub struct DistanceIndividualData
{
    screen_width: u32,
    screen_height: u32,
    n_points: usize,
    required_distance: u32
}


impl DistanceIndividualData
{
    pub fn new(n_points: usize, screen_width: u32, screen_height: u32, required_distance: u32) -> Self {
        DistanceIndividualData
        {
            screen_width,
            screen_height,
          n_points,
            required_distance
        }
    }

    pub fn clone(&self) -> Self
    {
        DistanceIndividualData
        {
            screen_width: self.screen_width,
            screen_height: self.screen_height,
            n_points: self.n_points,
            required_distance: self.required_distance
        }
    }
}

pub struct DistanceIndividual {
    fitness: f64,
    coords: Vec<(u32, u32)>,
}

impl DistanceIndividual
{
    pub fn draw(&self, output_filename: &str, ind_data: &DistanceIndividualData)
    {
        let mut img: RgbImage = ImageBuffer::new(ind_data.screen_width, ind_data.screen_height);

        // Draw points
        for i in 0..self.coords.len()
        {
            let point_color = Rgb([255, ((i*255)/(self.coords.len())) as u8 , 0]);

            draw_hollow_rect_mut(&mut img, Rect::at(self.coords[i].0 as i32 - 5, self.coords[i].1 as i32 - 5).of_size(10, 10), point_color.clone());
        }

        img.save(output_filename).unwrap();
    }

    fn distance(x1: u32, y1: u32, x2: u32, y2: u32) -> i64
    {
        let x = x2 as i64 - x1 as i64;
        let y = y2 as i64 - y1 as i64;
        return (x * x) + (y * y);
    }


}

impl EvoIndividual<DistanceIndividualData> for DistanceIndividual {
    fn new(ind_data: &DistanceIndividualData) -> Self {

        let mut coords: Vec<(u32, u32)> = Vec::new();

        for _ in 0..ind_data.n_points
        {
            coords.push((100,100));
        }


        DistanceIndividual
        {
            coords,
            fitness: 0.0,
        }
    }

    fn new_randomised(ind_data: &DistanceIndividualData, rng: &mut ThreadRng) -> Self {

        let mut coords: Vec<(u32, u32)> = Vec::new();
        for _ in 0..ind_data.n_points
        {
            coords.push(
                (rng.gen_range(5..ind_data.screen_width - 5),
                 rng.gen_range(5..ind_data.screen_height - 5)));
        }


        DistanceIndividual
        {
            coords,
            fitness: 0.0,
        }
    }

    fn copy_to(&self, ind: &mut Self)
    {
        for i in 0..self.coords.len()
        {
            ind.coords[i] = self.coords[i];
        }
    }

    fn clone(&self) -> Self {
        DistanceIndividual
        {
            fitness: self.fitness,
            coords: self.coords.clone(),
        }
    }

    fn mutate(&mut self, ind_data: &DistanceIndividualData, rng: &mut ThreadRng, mut_prob: f32, mut_amount: f32)
    {

        for coord in &mut self.coords
        {
            if rng.gen_range(0.0..1.0) < mut_prob
            {

                let x_mut = rng.gen_range(-mut_amount..mut_amount) as i32;
                let y_mut = rng.gen_range(-mut_amount..mut_amount) as i32;

                if (coord.0 as i32 + x_mut) < 0 ||  (coord.0 as i32 + x_mut) > ind_data.screen_width as i32
                {
                    coord.0 = (coord.0 as i32 - x_mut) as u32;
                }
                else
                {
                    coord.0 = (coord.0 as i32 + x_mut) as u32;
                }

                if (coord.1 as i32 + y_mut) < 0 ||  (coord.1 as i32 + y_mut) > ind_data.screen_height as i32
                {
                    coord.1 = (coord.1 as i32 - y_mut) as u32;
                }
                else
                {
                    coord.1 = (coord.1 as i32 + y_mut) as u32;
                }
            }
        }

    }

    fn crossover_to(&self, another_ind: &DistanceIndividual, dest_int: &mut DistanceIndividual, _ind_data: &DistanceIndividualData, rng: &mut ThreadRng)
    {


        for i in 0..self.coords.len()
        {
            let ratio: f32 = rng.gen_range(0.0..1.0);
            dest_int.coords[i].0 = ((self.coords[i].0 as f32*ratio) + (another_ind.coords[i].0 as f32 * (1.0-ratio))) as u32;
            dest_int.coords[i].1 = ((self.coords[i].1 as f32*ratio) + (another_ind.coords[i].1 as f32 * (1.0-ratio))) as u32;
        }

    }

    fn count_fitness(&mut self, ind_data: &DistanceIndividualData)
    {
        self.fitness = 0.0;

        let center_x = ind_data.screen_width/2;
        let center_y = ind_data.screen_height/2;

        for i in 0..self.coords.len()
        {
            let mut closest_dist = std::i64::MAX;

            let x1 = self.coords[i].0;
            let y1 = self.coords[i].1;

            for j in 0..self.coords.len()
            {
                if i == j
                {
                    continue;
                }

                let x2 = self.coords[j].0;
                let y2 = self.coords[j].1;
                let distance = DistanceIndividual::distance(x1, y1, x2, y2);

                if distance < closest_dist
                {
                    closest_dist = distance;
                }

            }

            self.fitness -= i64::abs(closest_dist - (ind_data.required_distance as i64 *  ind_data.required_distance as i64)) as f64;
            self.fitness -= DistanceIndividual::distance(x1, y1, center_x, center_y) as f64 / ((self.coords.len()  as f64) * 1.0);


        }

    }

    fn get_fitness(&self) -> f64 {
        return self.fitness;
    }
}
