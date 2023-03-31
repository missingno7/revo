use rand::Rng;
use rand::prelude::ThreadRng;
use revo::evo_individual::EvoIndividual;
use image::{RgbImage, ImageBuffer, Rgb};
use imageproc::drawing::{draw_hollow_rect_mut, draw_line_segment_mut};
use imageproc::rect::Rect;
use rand::seq::SliceRandom;
use std::collections::HashSet;

#[derive(Clone)]
pub enum SalesmanInitType
{
    Naive ,
    Noise ,
    Insertion ,
    GreedyJoining ,
}

pub struct SalesmanIndividualData
{
    coords: Vec<(u32, u32)>,
    screen_width: u32,
    screen_height: u32,
    shift_prob: f64,
    rev_prob: f64,
    init_type: SalesmanInitType
}


impl SalesmanIndividualData
{
    pub fn new(rng: &mut ThreadRng, n_cities: u32, screen_width: u32, screen_height: u32, shift_prob: f64, rev_prob: f64, init_type: SalesmanInitType) -> Self {
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
            init_type,
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
            init_type: self.init_type.clone(),
        }
    }
}

pub struct SalesmanIndividual {
    pub fitness: f64,
    genom: Vec<u16>,
}

impl SalesmanIndividual
{
    pub fn draw(&self, output_filename: &str, ind_data: &SalesmanIndividualData)
    {
        let mut img: RgbImage = ImageBuffer::new(ind_data.screen_width, ind_data.screen_height);

        // Draw cities
        for i in 0..ind_data.coords.len()
        {
            let city_color = Rgb([255, 0, 0]);

            let x = ind_data.coords[i].0 as f32;
            let y = ind_data.coords[i].1 as f32;
            draw_hollow_rect_mut(&mut img, Rect::at(x as i32 - 5, y as i32 - 5).of_size(10, 10), city_color.clone());
        }

        // Draw roads
        for i in 0..self.genom.len() - 1 {
            let col = ((i * 255) / (self.genom.len())) as u8;
            let road_color = Rgb([col, 255 - col, 0]);

            let from_x = ind_data.coords[self.genom[i] as usize].0 as f32;
            let from_y = ind_data.coords[self.genom[i] as usize].1 as f32;
            let to_x = ind_data.coords[self.genom[i + 1] as usize].0 as f32;
            let to_y = ind_data.coords[self.genom[i + 1] as usize].1 as f32;
            draw_line_segment_mut(&mut img, (from_x, from_y), (to_x, to_y), road_color.clone());
        }


        let road_color = Rgb([0, 255, 0]);

        let from_x = ind_data.coords[self.genom[0] as usize].0 as f32;
        let from_y = ind_data.coords[self.genom[0] as usize].1 as f32;
        let to_x = ind_data.coords[self.genom[self.genom.len() - 1] as usize].0 as f32;
        let to_y = ind_data.coords[self.genom[self.genom.len() - 1] as usize].1 as f32;
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

    fn new_random_naive(ind_data: &SalesmanIndividualData, rng: &mut ThreadRng) -> Self
    {
        let mut visited: Vec<bool> = vec![false; ind_data.coords.len()];
        let mut genom: Vec<u16> = (0 as u16..ind_data.coords.len() as u16).collect();
        let starting_city = rng.gen_range(0..ind_data.coords.len() - 1);

        genom[0] = starting_city as u16;
        visited[starting_city] = true;

        for i in 1..genom.len()
        {
            let mut closest_dist = std::f64::MAX;
            let mut closest_j = 0;

            for j in 0..ind_data.coords.len()
            {
                if i == j || visited[j] { continue; }

                let x1 = ind_data.coords[genom[i - 1] as usize].0;
                let y1 = ind_data.coords[genom[i - 1] as usize].1;
                let x2 = ind_data.coords[j].0;
                let y2 = ind_data.coords[j].1;
                let distance = SalesmanIndividual::distance(x1, y1, x2, y2);

                if distance < closest_dist
                {
                    closest_j = j;
                    closest_dist = distance;
                }
            }
            visited[closest_j] = true;
            genom[i] = closest_j as u16;
        }

        SalesmanIndividual
        {
            genom,
            fitness: 0.0,
        }
    }

    fn new_random_noise(ind_data: &SalesmanIndividualData, rng: &mut ThreadRng) -> Self
    {
        let mut genom: Vec<u16> = (0 as u16..ind_data.coords.len() as u16).collect();
        genom.shuffle(rng);

        SalesmanIndividual
        {
            genom,
            fitness: 0.0,
        }
    }

    fn new_random_insertion(ind_data: &SalesmanIndividualData, rng: &mut ThreadRng) -> Self
    {
        let mut cities: HashSet<u16> = (0 as u16..ind_data.coords.len() as u16).collect();
        let mut genom: Vec<u16> = Vec::new();

        for _ in 0..3
        {
            // Select random city
            let available_cities: Vec<u16> = cities.clone().into_iter().collect();
            let selected_city = if available_cities.len() > 1 { available_cities[rng.gen_range(0..available_cities.len() - 1)] } else { available_cities[0] };
            cities.remove(&selected_city);

            // Insert random city to genom
            genom.push(selected_city);
        }

        for _ in 3..ind_data.coords.len()
        {
            // Select random city
            let available_cities: Vec<u16> = cities.clone().into_iter().collect();
            let selected_city = if available_cities.len() > 1 { available_cities[rng.gen_range(0..available_cities.len() - 1)] } else { available_cities[0] };
            cities.remove(&selected_city);


            let x1 = ind_data.coords[genom[0] as usize].0;
            let y1 = ind_data.coords[genom[0] as usize].1;

            let x2 = ind_data.coords[selected_city as usize].0;
            let y2 = ind_data.coords[selected_city as usize].1;

            let x3 = ind_data.coords[genom[genom.len() - 1] as usize].0;
            let y3 = ind_data.coords[genom[genom.len() - 1] as usize].1;

            let mut shortest_dist = Self::distance(x1, y1, x2, y2) + Self::distance(x2, y2, x3, y3);
            let mut shortest_j = genom.len();


            for j in 0..genom.len() - 1
            {
                let x1 = ind_data.coords[genom[j] as usize].0;
                let y1 = ind_data.coords[genom[j] as usize].1;

                let x2 = ind_data.coords[selected_city as usize].0;
                let y2 = ind_data.coords[selected_city as usize].1;

                let x3 = ind_data.coords[genom[j + 1] as usize].0;
                let y3 = ind_data.coords[genom[j + 1] as usize].1;
                let distance = Self::distance(x1, y1, x2, y2) + Self::distance(x2, y2, x3, y3);

                if distance < shortest_dist
                {
                    shortest_dist = distance;
                    shortest_j = j + 1;
                }
            }

            // Insert random city to genom
            genom.insert(shortest_j, selected_city);
        }

        SalesmanIndividual
        {
            genom,
            fitness: 0.0,
        }
    }


    fn new_random_greedy_joining(ind_data: &SalesmanIndividualData, rng: &mut ThreadRng) -> Self
    {
        let mut paths: Vec<Vec<u16>> = Vec::new();

        for i in 0..ind_data.coords.len()
        {
            paths.push(vec![i as u16]);
        }

        loop
        {
            let selected_path = rng.gen_range(0..paths.len());

            let mut insert_to_shortest: bool = false;
            let mut shortest: usize = 0;
            let mut shortest_distance: f64 = std::f64::MAX;


            let (x_first, y_first) = ind_data.coords[paths[selected_path][0] as usize];
            let (x_last, y_last) = ind_data.coords[paths[selected_path][paths[selected_path].len() - 1] as usize];


            for i in 0..paths.len()
            {
                if i == selected_path { continue; }

                // Insert to  shortest
                {
                    let (xi, yi) = ind_data.coords[paths[i][paths[i].len() - 1] as usize];
                    let distance = SalesmanIndividual::distance(x_first, y_first, xi, yi);

                    if distance < shortest_distance
                    {
                        shortest_distance = distance;
                        shortest = i;
                        insert_to_shortest = true;
                    }
                }

                // Insert from shortest
                {
                    let (xi, yi) = ind_data.coords[paths[i][0] as usize];
                    let distance = SalesmanIndividual::distance(x_last, y_last, xi, yi);

                    if distance < shortest_distance
                    {
                        shortest_distance = distance;
                        shortest = i;
                        insert_to_shortest = false;
                    }
                }
            }


            if insert_to_shortest
            {
                // Insert selected path to shortest
                let mut tmp_path = paths[selected_path].clone();
                paths[shortest].append(&mut tmp_path);
                paths.remove(selected_path);
            } else {
                let mut tmp_path = paths[shortest].clone();
                paths[selected_path].append(&mut tmp_path);
                paths.remove(shortest);
            }


            if paths.len() == 1
            {
                break;
            }
        }

        SalesmanIndividual
        {
            genom: paths[0].clone(),
            fitness: 0.0,
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


    fn new_randomised(ind_data: &SalesmanIndividualData, rng: &mut ThreadRng) -> Self
    {
        match ind_data.init_type
        {
            SalesmanInitType::Naive => {Self::new_random_naive(ind_data, rng)},
            SalesmanInitType::Noise => {Self::new_random_noise(ind_data, rng)},
            SalesmanInitType::Insertion => {Self::new_random_insertion(ind_data, rng)},
            SalesmanInitType::GreedyJoining => {Self::new_random_greedy_joining(ind_data, rng)},
        }
    }

    fn copy_to(&self, ind: &mut Self)
    {
        for i in 0..self.genom.len()
        {
            ind.genom[i] = self.genom[i];
        }
    }

    fn clone(&self) -> Self {
        SalesmanIndividual
        {
            fitness: self.fitness,
            genom: self.genom.clone(),
        }
    }

    fn mutate(&mut self, ind_data: &SalesmanIndividualData, rng: &mut ThreadRng, _mut_prob: f32, _mut_amount: f32)
    {
        /*
        Too much of unnecessary overhead
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
        */

        if rng.gen_range(0.0..1.0) < ind_data.shift_prob {
            // Shifting

            let cnttoshift = rng.gen_range(1..self.genom.len() - 1);

            self.shift_multiple(rng.gen_range(0..self.genom.len() - 1), rng.gen_range(0..self.genom.len() - 1), rng.gen_bool(0.5),
                                cnttoshift);
        }

        if rng.gen_range(0.0..1.0) < ind_data.rev_prob {
            self.reverse_part(rng.gen_range(0..self.genom.len() - 1), rng.gen_range(0..self.genom.len() - 1));
        }
    }

    fn crossover_to(&self, another_ind: &SalesmanIndividual, dest_int: &mut SalesmanIndividual, _ind_data: &SalesmanIndividualData, rng: &mut ThreadRng)
    {
        let mut used: Vec<bool> = vec![false; self.genom.len()];
        let cross_point = rng.gen_range(0..self.genom.len() - 1);
        let mut i = cross_point;

        loop {
            if used[self.genom[i] as usize] {
                if used[another_ind.genom[i] as usize] {
                    // Both on list
                    for j in 0..self.genom.len() {
                        if !used[j] {
                            dest_int.genom[i] = j as u16;
                            break;
                        }
                    }
                } else {
                    // second not used
                    dest_int.genom[i] = another_ind.genom[i];
                }
            } else {
                if used[another_ind.genom[i] as usize] {
                    // first not used
                    dest_int.genom[i] = self.genom[i];
                } else {
                    // None used
                    if rng.gen_bool(0.5) {
                        dest_int.genom[i] = self.genom[i];
                    } else {
                        dest_int.genom[i] = another_ind.genom[i];
                    }
                }
            }

            used[dest_int.genom[i] as usize] = true;

            i = (i + 1) % self.genom.len();

            if (i % self.genom.len()) == cross_point
            {
                break;
            }
        }
    }

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
