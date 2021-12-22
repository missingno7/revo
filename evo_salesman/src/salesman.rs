use rand::Rng;
use rand::seq::SliceRandom;
use rand::prelude::ThreadRng;
use revo::evo_individual::EvoIndividual;
use image::{RgbImage, ImageBuffer, Rgb};
use imageproc::drawing::{draw_hollow_rect_mut, draw_line_segment_mut};
use imageproc::rect::Rect;

pub struct SalesmanIndividualData
{
    coords: Vec<(u32, u32)>,
    screen_width: u32,
    screen_height: u32,
    shift_prob: f64,
    rev_prob: f64,

}


impl SalesmanIndividualData
{
    pub fn new(rng: &mut ThreadRng, n_cities: u32, screen_width: u32, screen_height: u32, shift_prob: f64, rev_prob: f64) -> Self {
        let mut coords: Vec<(u32, u32)> = Vec::new();

        for _ in 0..n_cities
        {
            coords.push(
                (rng.gen_range(5..screen_width - 5),
                 rng.gen_range(5..screen_height - 5)));
        }


        SalesmanIndividualData
        {
            coords,
            screen_width,
            screen_height,
            shift_prob,
            rev_prob,
        }
    }

    pub fn clone(&self) -> Self
    {
        SalesmanIndividualData
        {
            coords: self.coords.clone(),
            screen_width: self.screen_width,
            screen_height: self.screen_height,
            shift_prob: self.shift_prob,
            rev_prob: self.rev_prob,
        }
    }
}

pub struct SalesmanIndividual {
    fitness: f64,
    genom: Vec<u16>,
}

impl SalesmanIndividual
{
    pub fn draw(&self, output_filename: &str, ind_data: &SalesmanIndividualData)
    {
        let mut img: RgbImage = ImageBuffer::new(ind_data.screen_width, ind_data.screen_height);
        let city_color = Rgb([255, 0, 0]);
        let road_color = Rgb([255, 255, 255]);

        // Draw cities
        for coord in &ind_data.coords
        {
            draw_hollow_rect_mut(&mut img, Rect::at(coord.0 as i32 - 5, coord.1 as i32 - 5).of_size(10, 10), city_color.clone());
        }

        // Draw roads
        for i in 0..self.genom.len() - 1 {
            let from_x = ind_data.coords[self.genom[i] as usize].0 as f32;
            let from_y = ind_data.coords[self.genom[i] as usize].1 as f32;
            let to_x = ind_data.coords[self.genom[i + 1] as usize].0 as f32;
            let to_y = ind_data.coords[self.genom[i + 1] as usize].1 as f32;
            draw_line_segment_mut(&mut img, (from_x, from_y), (to_x, to_y), road_color.clone());
        }


        let from_x = ind_data.coords[self.genom[0] as usize].0 as f32;
        let from_y = ind_data.coords[self.genom[0] as usize].1 as f32;
        let to_x = ind_data.coords[self.genom[self.genom.len()-1] as usize].0 as f32;
        let to_y = ind_data.coords[self.genom[self.genom.len()-1] as usize].1 as f32;
        draw_line_segment_mut(&mut img, (from_x, from_y), (to_x, to_y), road_color.clone());


        img.save(output_filename).unwrap();
    }

    fn distance(x1: u32, y1: u32, x2: u32, y2: u32) -> f64
    {
        let x = x2 as f64 - x1 as f64;
        let y = y2 as f64 - y1 as f64;
        return (x * x) + (y * y);
    }


    fn reverse_part(&mut self, from: usize, to: usize) {
        if from == to {
            return;
        }

        let mut frmi = from;
        let mut toi = to;

        loop {
            let tmp = self.genom[frmi];
            self.genom[frmi] = self.genom[toi];
            self.genom[toi] = tmp;


            if toi < 1 {
                toi = self.genom.len() - 1;
            } else {
                toi -= 1;
            }

            if frmi + 1 >= self.genom.len() {
                frmi = 0;
            } else {
                frmi += 1;
            }

            if (frmi - 1) == toi || frmi == toi
            {
                break;
            }
        }
    }


    fn shift_multiple(&mut self, from: usize, to: usize, forward: bool, cnt: usize) {
        if forward {
            for i in cnt - 1..=0 {
                let frmi = (from + i) % self.genom.len();
                let toi = (to + i) % self.genom.len();

                self.shift(frmi, toi, true);
            }
        } else {
            // backwards
            for i in 0..cnt {
                let frmi = (from + i) % self.genom.len();
                let toi = (to + i) % self.genom.len();

                self.shift(frmi, toi, false);
            }
        }
    }

    fn shift(&mut self, from: usize, to: usize, forward: bool) {
        if from == to {
            return;
        }

        let mut i = from;

        if forward {
            loop {
                let next = (i + 1) % self.genom.len();

                let tmp = self.genom[i];
                self.genom[i] = self.genom[next];
                self.genom[next] = tmp;

                i = next;
                if i == to
                {
                    break;
                }
            }
        } else {

            // backwards
            loop {
                let next = if i < 1 {
                    self.genom.len() - 1
                } else {
                    i - 1
                };

                let tmp = self.genom[i];
                self.genom[i] = self.genom[next];
                self.genom[next] = tmp;

                i = next;
                if i == to
                {
                    break;
                }
            }
        }
    }
}

impl EvoIndividual<SalesmanIndividualData> for SalesmanIndividual {
    fn new(ind_data: &SalesmanIndividualData) -> Self {
        SalesmanIndividual
        {
            genom: (0 as u16..ind_data.coords.len() as u16).collect(),
            fitness: 0.0,
        }
    }

    fn new_randomised(ind_data: &SalesmanIndividualData, rng: &mut ThreadRng) -> Self {
        let mut genom: Vec<u16> = (0 as u16..ind_data.coords.len() as u16).collect();
        genom.shuffle(rng);

        SalesmanIndividual
        {
            genom,
            fitness: 0.0,
        }
    }

    fn copy_to(&self, ind: &mut Self)
    {
        ind.fitness = self.fitness;
        ind.genom = self.genom.clone();
    }

    fn clone(&self) -> Self {
        SalesmanIndividual
        {
            fitness: self.fitness,
            genom: self.genom.clone(),
        }
    }

    fn mutate(&mut self, ind_data: &SalesmanIndividualData, rng: &mut ThreadRng, mut_prob: f32, mut_amount: f32)
    {
        for i in 0..self.genom.len() {
            if rng.gen_range(0.0..1.0) < mut_prob {
                let swap_with = rng.gen_range(0..self.genom.len() - 1);
                if swap_with == i {
                    continue;
                }
                // Simple swap
                let tmp = self.genom[i];
                self.genom[i] = self.genom[swap_with];
                self.genom[swap_with] = tmp;
            }
        }

        if rng.gen_range(0.0..1.0) < ind_data.shift_prob {
            // Shifting

            let mut cnttoshift = rng.gen_range(1..self.genom.len() - 1);

            self.shift_multiple(rng.gen_range(0..self.genom.len() - 1), rng.gen_range(0..self.genom.len() - 1), rng.gen_bool(0.5),
                                cnttoshift);
        }

        if rng.gen_range(0.0..1.0) < ind_data.rev_prob {
            self.reverse_part(rng.gen_range(0..self.genom.len() - 1), rng.gen_range(0..self.genom.len() - 1));
        }
    }

    fn crossover_to(&self, another_ind: &SalesmanIndividual, dest_int: &mut SalesmanIndividual, _ind_data: &SalesmanIndividualData, rng: &mut ThreadRng)
    {}

    fn count_fitness(&mut self, ind_data: &SalesmanIndividualData)
    {
        self.fitness = 0.0;

        for i in 0..ind_data.coords.len() - 1
        {
            let x1 = ind_data.coords[self.genom[i] as usize].0;
            let x2 = ind_data.coords[self.genom[i + 1] as usize].0;
            let y1 = ind_data.coords[self.genom[i] as usize].1;
            let y2 = ind_data.coords[self.genom[i + 1] as usize].1;
            self.fitness -= SalesmanIndividual::distance(x1, y1, x2, y2);
        }

        let x1 = ind_data.coords[self.genom[0] as usize].0;
        let x2 = ind_data.coords[self.genom[self.genom.len() - 1] as usize].0;
        let y1 = ind_data.coords[self.genom[0] as usize].1;
        let y2 = ind_data.coords[self.genom[self.genom.len() - 1] as usize].1;
        self.fitness -= SalesmanIndividual::distance(x1, y1, x2, y2);
    }

    fn get_fitness(&self) -> f64 {
        return self.fitness;
    }
}
