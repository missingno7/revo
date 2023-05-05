use crate::salesman_data::{SalesmanIndividualData, SalesmanInitType};
use image::{Rgb, RgbImage};
use imageproc::drawing::{draw_hollow_rect_mut, draw_line_segment_mut};
use imageproc::rect::Rect;
use rand::prelude::ThreadRng;
use rand::seq::SliceRandom;
use rand::Rng;
use revo::evo_individual::{EvoIndividual, Visualise};
use revo::utils::Coord;
use std::collections::HashSet;

#[derive(Clone)]
pub struct SalesmanIndividual {
    pub fitness: f64,
    genom: Vec<u16>,
}

impl SalesmanIndividual {
    fn reverse_part(&mut self, from: usize, to: usize) {
        let len = self.genom.len();

        let to = match from < to {
            true => to,
            false => to + len,
        };

        let mut frmi = from;
        let mut toi = to;

        while frmi <= toi {
            let abs_toi = toi % len;
            let abs_frmi = frmi % len;

            self.genom.swap(abs_frmi, abs_toi);

            frmi += 1;
            toi -= 1;
        }
    }

    fn shift_multiple(&mut self, from: usize, to: usize, shift: usize) {
        let len = self.genom.len();
        let mut i_from = from;
        let mut i_to = to + 1;

        let slice_len = if to < from {
            i_to + len - i_from
        } else {
            i_to - i_from
        };

        let mut tmp: Vec<u16> = Vec::with_capacity(slice_len);

        let mut source_i = i_from;
        for _ in 0..slice_len {
            tmp.push(self.genom[source_i]);

            source_i += 1;
            if source_i >= len {
                source_i = 0
            };
        }

        // Do shifting
        for _ in 0..shift {
            self.genom.swap(i_from, i_to);

            i_from += 1;
            if i_from >= len {
                i_from = 0;
            }

            i_to += 1;
            if i_to >= len {
                i_to = 0;
            }
        }

        // Put stuff back
        let mut source_i = i_from;
        for item in tmp.iter().take(slice_len) {
            self.genom[source_i] = *item;

            source_i += 1;
            if source_i >= len {
                source_i = 0
            };
        }
    }

    fn _impl_crossover_to(
        &self,
        another_ind: &SalesmanIndividual,
        dest_ind: &mut SalesmanIndividual,
        start_cross_point: usize,
        end_cross_point: usize,
        other_ind_start_cross_point: usize,
    ) {
        let mut used: Vec<bool> = vec![false; self.genom.len()];

        let mut i = start_cross_point;

        // Copy points from first parent
        loop {
            dest_ind.genom[i] = self.genom[i];
            used[self.genom[i] as usize] = true;

            if i == end_cross_point {
                break;
            }

            i += 1;
            if i >= self.genom.len() {
                i = 0;
            }
        }

        // Copy points from second parent
        let mut other_i = other_ind_start_cross_point;
        let mut i = end_cross_point + 1;
        loop {
            if !used[another_ind.genom[other_i] as usize] {
                dest_ind.genom[i] = another_ind.genom[other_i];
                used[another_ind.genom[other_i] as usize] = true;

                i += 1;
                if i >= self.genom.len() {
                    i = 0;
                }
            }

            if i == start_cross_point {
                break;
            }

            other_i += 1;
            if other_i >= self.genom.len() {
                other_i = 0;
            }
        }
    }

    fn new_random_naive(ind_data: &SalesmanIndividualData, rng: &mut ThreadRng) -> Self {
        let mut visited: Vec<bool> = vec![false; ind_data.coords.len()];
        let mut genom: Vec<u16> = (0_u16..ind_data.coords.len() as u16).collect();

        // Start from random city
        let starting_city = rng.gen_range(0..ind_data.coords.len() - 1);
        genom[0] = starting_city as u16;
        visited[starting_city] = true;

        for i in 1..genom.len() {
            // Find closest city that is not visited yet
            let (closest_j, _) = ind_data
                .coords
                .iter()
                .enumerate()
                .filter(|&(j, _)| i != j && !visited[j])
                .min_by_key(|(_, coord)| {
                    Coord::distance_euclid(coord, &ind_data.coords[genom[i - 1] as usize])
                })
                .unwrap();

            // Mark city as visited and add it to genom
            visited[closest_j] = true;
            genom[i] = closest_j as u16;
        }

        SalesmanIndividual {
            genom,
            fitness: 0.0,
        }
    }

    fn new_random_noise(ind_data: &SalesmanIndividualData, rng: &mut ThreadRng) -> Self {
        let mut genom: Vec<u16> = (0_u16..ind_data.coords.len() as u16).collect();
        genom.shuffle(rng);

        SalesmanIndividual {
            genom,
            fitness: 0.0,
        }
    }

    fn new_random_insertion(ind_data: &SalesmanIndividualData, rng: &mut ThreadRng) -> Self {
        let mut cities: HashSet<u16> = (0_u16..ind_data.coords.len() as u16).collect();
        let mut genom: Vec<u16> = Vec::new();

        for _ in 0..3 {
            // Select random city
            let available_cities: Vec<u16> = cities.clone().into_iter().collect();
            let selected_city = if available_cities.len() > 1 {
                available_cities[rng.gen_range(0..available_cities.len() - 1)]
            } else {
                available_cities[0]
            };
            cities.remove(&selected_city);

            // Insert random city to genom
            genom.push(selected_city);
        }

        for _ in 3..ind_data.coords.len() {
            // Select random city
            let available_cities: Vec<u16> = cities.clone().into_iter().collect();
            let selected_city = if available_cities.len() > 1 {
                available_cities[rng.gen_range(0..available_cities.len() - 1)]
            } else {
                available_cities[0]
            };
            cities.remove(&selected_city);

            let city_1 = &ind_data.coords[genom[0] as usize];
            let city_2 = &ind_data.coords[selected_city as usize];
            let city_3 = &ind_data.coords[genom[genom.len() - 1] as usize];

            let mut shortest_dist =
                Coord::distance_euclid(city_1, city_2) + Coord::distance_euclid(city_2, city_3);
            let mut shortest_j = genom.len();

            for j in 0..genom.len() - 1 {
                let city_1 = &ind_data.coords[genom[j] as usize];
                let city_3 = &ind_data.coords[genom[j + 1] as usize];
                let distance =
                    Coord::distance_euclid(city_1, city_2) + Coord::distance_euclid(city_2, city_3);

                if distance < shortest_dist {
                    shortest_dist = distance;
                    shortest_j = j + 1;
                }
            }

            // Insert random city to genom
            genom.insert(shortest_j, selected_city);
        }

        SalesmanIndividual {
            genom,
            fitness: 0.0,
        }
    }

    fn new_random_greedy_joining(ind_data: &SalesmanIndividualData, rng: &mut ThreadRng) -> Self {
        let mut paths: Vec<Vec<u16>> = Vec::new();

        for i in 0..ind_data.coords.len() {
            paths.push(vec![i as u16]);
        }

        loop {
            let selected_path = rng.gen_range(0..paths.len());

            let mut insert_to_shortest: bool = false;
            let mut shortest: usize = 0;
            let mut shortest_distance: i64 = i64::MAX;

            let city_first = &ind_data.coords[paths[selected_path][0] as usize];
            let city_last =
                &ind_data.coords[paths[selected_path][paths[selected_path].len() - 1] as usize];

            for (i, _) in paths.iter().enumerate() {
                if i == selected_path {
                    continue;
                }

                // Insert to shortest
                {
                    let city_i = &ind_data.coords[paths[i][paths[i].len() - 1] as usize];
                    let distance = Coord::distance_euclid(city_first, city_i);

                    if distance < shortest_distance {
                        shortest_distance = distance;
                        shortest = i;
                        insert_to_shortest = true;
                    }
                }

                // Insert from shortest
                {
                    let city_i = &ind_data.coords[paths[i][0] as usize];
                    let distance = Coord::distance_euclid(city_last, city_i);

                    if distance < shortest_distance {
                        shortest_distance = distance;
                        shortest = i;
                        insert_to_shortest = false;
                    }
                }
            }

            if insert_to_shortest {
                // Insert selected path to shortest
                let mut tmp_path = paths[selected_path].clone();
                paths[shortest].append(&mut tmp_path);
                paths.remove(selected_path);
            } else {
                let mut tmp_path = paths[shortest].clone();
                paths[selected_path].append(&mut tmp_path);
                paths.remove(shortest);
            }

            if paths.len() == 1 {
                break;
            }
        }

        SalesmanIndividual {
            genom: paths[0].clone(),
            fitness: 0.0,
        }
    }
}

impl EvoIndividual<SalesmanIndividualData> for SalesmanIndividual {
    fn new_randomised(ind_data: &SalesmanIndividualData, rng: &mut ThreadRng) -> Self {
        match ind_data.init_type {
            SalesmanInitType::Naive => Self::new_random_naive(ind_data, rng),
            SalesmanInitType::Noise => Self::new_random_noise(ind_data, rng),
            SalesmanInitType::Insertion => Self::new_random_insertion(ind_data, rng),
            SalesmanInitType::GreedyJoining => Self::new_random_greedy_joining(ind_data, rng),
        }
    }

    fn mutate(
        &mut self,
        ind_data: &SalesmanIndividualData,
        rng: &mut ThreadRng,
        mut_prob: f32,
        _mut_amount: f32,
    ) {
        // Shifting
        if rng.gen_range(0.0..1.0) < ind_data.shift_prob {
            self.shift_multiple(
                rng.gen_range(0..self.genom.len() - 1),
                rng.gen_range(0..self.genom.len() - 1),
                rng.gen_range(1..self.genom.len() - 1),
            );
        }

        // Reversing
        if rng.gen_range(0.0..1.0) < ind_data.rev_prob {
            self.reverse_part(
                rng.gen_range(0..self.genom.len() - 1),
                rng.gen_range(0..self.genom.len() - 1),
            );
        }

        // Swapping
        if rng.gen_range(0.0..1.0) < mut_prob {
            let i = rng.gen_range(0..self.genom.len() - 1);
            let j = rng.gen_range(0..self.genom.len() - 1);

            if i != j {
                self.genom.swap(i, j);
            }
        }
    }

    fn crossover(
        &self,
        another_ind: &SalesmanIndividual,
        _ind_data: &SalesmanIndividualData,
        rng: &mut ThreadRng,
    ) -> SalesmanIndividual {
        let start_cross_point = rng.gen_range(0..self.genom.len() - 1);
        let end_cross_point = rng.gen_range(0..self.genom.len() - 1);
        let other_i = rng.gen_range(0..self.genom.len() - 1);

        let mut dest_ind = self.clone();

        self._impl_crossover_to(
            another_ind,
            &mut dest_ind,
            start_cross_point,
            end_cross_point,
            other_i,
        );

        dest_ind
    }

    fn count_fitness(&self, ind_data: &SalesmanIndividualData) -> f64 {
        let mut fitness = 0.0;

        for i in 0..ind_data.coords.len() - 1 {
            fitness -= Coord::distance_euclid(
                &ind_data.coords[self.genom[i] as usize],
                &ind_data.coords[self.genom[i + 1] as usize],
            ) as f64;
        }

        fitness -= Coord::distance_euclid(
            &ind_data.coords[self.genom[0] as usize],
            &ind_data.coords[self.genom[self.genom.len() - 1] as usize],
        ) as f64;
        fitness
    }

    fn get_visuals(&self, ind_data: &SalesmanIndividualData) -> (f64, f64) {
        let mut a: f64 = 0.0;
        let mut b: f64 = 0.0;

        let len = self.genom.len();

        for i in 0..len - 1 {
            let city_1 = ind_data.coords[self.genom[i] as usize];
            let city_2 = ind_data.coords[self.genom[i + 1] as usize];

            let (dx, dy) = Coord::normalized_distance_between_points(&city_1, &city_2);
            a += dx.abs();
            b += dy.abs();
        }

        let city_1 = ind_data.coords[self.genom[0] as usize];
        let city_2 = ind_data.coords[self.genom[len - 1] as usize];

        let (dx, dy) = Coord::normalized_distance_between_points(&city_1, &city_2);

        a += dx.abs();
        b += dy.abs();

        (a, b)
    }
}

impl Visualise<SalesmanIndividualData> for SalesmanIndividual {
    fn visualise(&self, ind_data: &SalesmanIndividualData) -> RgbImage {
        let mut img = RgbImage::new(ind_data.screen_width, ind_data.screen_height);

        // Draw cities
        for i in 0..ind_data.coords.len() {
            let city_color = Rgb([255, 0, 0]);

            let i_city = &ind_data.coords[i];
            draw_hollow_rect_mut(
                &mut img,
                Rect::at(i_city.x - 5, i_city.y - 5).of_size(10, 10),
                city_color,
            );
        }

        // Draw roads
        for i in 0..self.genom.len() - 1 {
            let col = ((i * 255) / (self.genom.len())) as u8;
            let road_color = Rgb([col, 255 - col, 0]);

            let from_city = &ind_data.coords[self.genom[i] as usize];
            let to_city = &ind_data.coords[self.genom[i + 1] as usize];

            draw_line_segment_mut(&mut img, from_city.as_f32(), to_city.as_f32(), road_color);
        }

        let road_color = Rgb([0, 255, 0]);

        let from_city = &ind_data.coords[self.genom[0] as usize];
        let to_city = &ind_data.coords[self.genom[self.genom.len() - 1] as usize];
        draw_line_segment_mut(&mut img, from_city.as_f32(), to_city.as_f32(), road_color);

        img
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_shift_multiple() {
        // 6 cities test
        // [0, 1, 2, 3, 4, 5]
        let ind_6 = SalesmanIndividual {
            fitness: 0.0,
            genom: vec![0, 1, 2, 3, 4, 5],
        };

        let mut ind = ind_6.clone();
        ind.shift_multiple(0, 2, 3);
        assert_eq!(ind.genom, vec![3, 4, 5, 0, 1, 2]);

        ind = ind_6.clone();
        ind.shift_multiple(0, 2, 1);
        assert_eq!(ind.genom, vec![3, 0, 1, 2, 4, 5]);

        ind = ind_6.clone();
        ind.shift_multiple(0, 2, 6);
        assert_eq!(ind.genom, vec![0, 1, 2, 3, 4, 5]);

        // 7 cities test
        // [0, 1, 2, 3, 4, 5, 6]
        let ind_7 = SalesmanIndividual {
            fitness: 0.0,
            genom: vec![0, 1, 2, 3, 4, 5, 6],
        };

        ind = ind_7.clone();
        ind.shift_multiple(1, 3, 7);
        assert_eq!(ind.genom, vec![6, 1, 2, 3, 0, 4, 5]);

        ind = ind_7.clone();
        ind.shift_multiple(1, 3, 13);
        assert_eq!(ind.genom, vec![1, 2, 3, 5, 6, 0, 4]);

        ind = ind_7.clone();
        ind.shift_multiple(1, 3, 0);
        assert_eq!(ind.genom, vec![0, 1, 2, 3, 4, 5, 6]);

        ind = ind_7.clone();
        ind.shift_multiple(3, 3, 5);
        assert_eq!(ind.genom, vec![1, 3, 2, 4, 5, 6, 0]);

        ind = ind_7.clone();
        ind.shift_multiple(3, 3, 6);
        assert_eq!(ind.genom, vec![1, 2, 3, 4, 5, 6, 0]);
    }

    #[test]
    fn test_reverse_part() {
        // [0, 1, 2, 3, 4, 5]
        let ind_6 = SalesmanIndividual {
            fitness: 0.0,
            genom: vec![0, 1, 2, 3, 4, 5],
        };

        let mut ind = ind_6.clone();
        ind.reverse_part(0, 2);
        assert_eq!(ind.genom, vec![2, 1, 0, 3, 4, 5]);

        ind = ind_6.clone();
        ind.reverse_part(5, 0);
        assert_eq!(ind.genom, vec![5, 1, 2, 3, 4, 0]);

        ind = ind_6.clone();
        ind.reverse_part(5, 1);
        assert_eq!(ind.genom, vec![0, 5, 2, 3, 4, 1]);

        ind = ind_6.clone();
        ind.reverse_part(5, 2);
        assert_eq!(ind.genom, vec![1, 0, 5, 3, 4, 2]);

        ind = ind_6.clone();
        ind.reverse_part(2, 2);
        assert_eq!(ind.genom, vec![4, 3, 2, 1, 0, 5]);
    }

    #[test]
    fn test_impl_crossover_to() {
        let mut ind_1 = SalesmanIndividual {
            fitness: 0.0,
            genom: vec![0, 1, 2, 3, 4, 5],
        };

        let mut ind_2 = SalesmanIndividual {
            fitness: 0.0,
            genom: vec![5, 4, 3, 2, 1, 0],
        };

        let mut res_ind = ind_1.clone();

        ind_1.genom = vec![0, 1, 2, 3, 4, 5];
        ind_2.genom = vec![5, 4, 3, 2, 1, 0];

        ind_1._impl_crossover_to(&ind_2, &mut res_ind, 0, 2, 0);
        assert_eq!(res_ind.genom, vec![0, 1, 2, 5, 4, 3]);

        ind_1._impl_crossover_to(&ind_2, &mut res_ind, 5, 0, 1);
        assert_eq!(res_ind.genom, vec![0, 4, 3, 2, 1, 5]);

        ind_1.genom = vec![4, 1, 3, 2, 5, 0];
        ind_2.genom = vec![3, 5, 0, 1, 4, 2];
        ind_1._impl_crossover_to(&ind_2, &mut res_ind, 1, 3, 2);
        assert_eq!(res_ind.genom, vec![5, 1, 3, 2, 0, 4]);
    }
}
