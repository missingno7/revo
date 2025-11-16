use crate::packer_data::PackerIndividualData;
use image::{Rgb, RgbImage};
use imageproc::drawing::draw_filled_rect_mut;
use imageproc::rect::Rect;
use rand::rngs::SmallRng;
use rand::seq::SliceRandom;
use rand::Rng;
use revo::evo_individual::{EvoIndividual, Visualise};

pub type RectPlacement = (u32, u32, u32, u32); // x, y, w, h
pub type LayoutResult = (Vec<RectPlacement>, u32, u32); // placements, total_width, total_height

#[derive(Clone, Debug)]
pub struct PackerIndividual {
    pub fitness: f64,
    // Permutation of rectangle indices
    order: Vec<u16>,
    // true = rectangle is rotated by 90°
    rotations: Vec<bool>,
    // Maximum length of a row in pixels
    row_len: u32,
}

impl PackerIndividual {
    fn new_with_data(
        n_rects: usize,
        row_len: u32,
        mut order: Vec<u16>,
        rotations: Vec<bool>,
    ) -> Self {
        if order.is_empty() {
            order = (0_u16..n_rects as u16).collect();
        }

        PackerIndividual {
            fitness: 0.0,
            order,
            rotations,
            row_len,
        }
    }

    /// Rectangle layout:
    /// - takes rectangles in order `order`
    /// - starts a new row after `row_len` pixels of width
    /// - returns (x, y, w, h) for each rectangle + resulting overall width & height
    fn compute_layout(&self, ind_data: &PackerIndividualData) -> LayoutResult {
        let n = self.order.len();
        if n == 0 {
            return (Vec::new(), 0, 0);
        }

        let mut placements: Vec<(u32, u32, u32, u32)> = Vec::with_capacity(n);
        let mut cur_x: u32 = 0;
        let mut cur_y: u32 = 0;
        let mut row_height: u32 = 0;
        let mut max_width: u32 = 0;

        let row_len = self.row_len;

        for (i, &rect_idx_u16) in self.order.iter().enumerate() {
            let rect_idx = rect_idx_u16 as usize;
            let mut w = ind_data.rects[rect_idx].w;
            let mut h = ind_data.rects[rect_idx].h;

            // Optional 90° rotation
            if self.rotations[rect_idx] {
                std::mem::swap(&mut w, &mut h);
            }

            // Start a new row
            if i > 0 && cur_x >= row_len {
                cur_x = 0;
                cur_y += row_height;
                row_height = 0;
            }

            placements.push((cur_x, cur_y, w, h));
            cur_x += w;
            row_height = row_height.max(h);
            max_width = max_width.max(cur_x);
        }

        let total_height = cur_y + row_height;

        (placements, max_width, total_height)
    }
}

impl EvoIndividual<PackerIndividualData> for PackerIndividual {
    fn new_randomised(ind_data: &PackerIndividualData, rng: &mut SmallRng) -> Self {
        let n = ind_data.rects.len();

        let mut order: Vec<u16> = (0_u16..n as u16).collect();
        order.shuffle(rng);

        let row_len = rng.gen_range(1..=ind_data.max_width);

        let rotations: Vec<bool> = (0..n).map(|_| rng.gen_bool(0.5)).collect();

        PackerIndividual::new_with_data(n, row_len, order, rotations)
    }

    fn mutate(
        &mut self,
        ind_data: &PackerIndividualData,
        rng: &mut SmallRng,
        mut_prob: f32,
        _mut_amount: f32,
    ) {
        let n = ind_data.rects.len();
        if n < 2 {
            return;
        }

        for i in 0..n {
            // Random rotation flip
            if rng.gen::<f32>() < mut_prob {
                self.rotations[i] = !self.rotations[i];
            }

            // Swap two rectangles randomly
            if rng.gen::<f32>() < ind_data.swap_prob {
                let mut j = rng.gen_range(0..n);
                if i == j {
                    j = (j + 1) % n;
                }
                self.rotations.swap(i, j);
                self.order.swap(i, j);
            }
        }

        // Reverse a random subsequence
        if rng.gen::<f32>() < ind_data.reverse_prob {
            let start = rng.gen_range(0..n - 1);
            let end = rng.gen_range(start + 1..n);
            self.order[start..=end].reverse();
            self.rotations[start..=end].reverse();
        }

        // Random row length change
        if rng.gen::<f32>() < ind_data.height_change_prob {
            let height_change_amount: i32 = ind_data.height_change_amount as i32;
            let delta: i32 = rng.gen_range(-height_change_amount..height_change_amount);
            let mut new_len = self.row_len as i32 + delta;

            if new_len < 1 {
                new_len = 1;
            }
            if new_len > ind_data.max_width as i32 {
                new_len = ind_data.max_width as i32;
            }
            self.row_len = new_len as u32;
        }
    }

    fn crossover(
        &self,
        another_ind: &PackerIndividual,
        _ind_data: &PackerIndividualData,
        rng: &mut SmallRng,
    ) -> PackerIndividual {
        let len = self.order.len();
        if len < 2 {
            return self.clone();
        }

        // --- OX crossover applied on `order` ---
        let mut child_order = vec![u16::MAX; len];
        let start = rng.gen_range(0..len);
        let end = rng.gen_range(start..len);

        let mut used = vec![false; len];

        // Copy segment from parent 1
        #[allow(clippy::needless_range_loop)]
        for i in start..=end {
            let gene = self.order[i];
            child_order[i] = gene;
            used[gene as usize] = true;
        }

        // Fill remaining positions using parent 2
        let mut pos = (end + 1) % len;
        for &gene in another_ind.order.iter() {
            if !used[gene as usize] {
                child_order[pos] = gene;
                used[gene as usize] = true;
                pos = (pos + 1) % len;
            }
        }

        // --- rotations: uniform crossover per rectangle index ---
        let mut child_rotations = Vec::with_capacity(len);
        for i in 0..len {
            let r = if rng.gen_bool(0.5) {
                self.rotations[i]
            } else {
                another_ind.rotations[i]
            };
            child_rotations.push(r);
        }

        // Blend row_len via random ratio
        let ratio: f32 = rng.gen();
        let row_len =
            (self.row_len as f32 * ratio + (1.0 - ratio) * another_ind.row_len as f32) as u32;

        PackerIndividual::new_with_data(len, row_len, child_order, child_rotations)
    }

    fn count_fitness(&mut self, ind_data: &PackerIndividualData) {
        let (_placements, width, height) = self.compute_layout(ind_data);

        if width == 0 || height == 0 {
            self.fitness = f64::NEG_INFINITY;
            return;
        }

        let area = (width as u64 * height as u64) as f64;
        let aspect = (width.max(height) as f64) / (width.min(height) as f64);

        // Penalization strength for stretched layouts
        let lambda = 0.01_f64;

        // 1.0 + lambda*(aspect - 1) = 1 for a square, grows with "noodliness"
        let penalty = 1.0 + lambda * (aspect - 1.0);

        // We minimize area * penalty
        self.fitness = -(area * penalty);
    }

    fn get_fitness(&self) -> f64 {
        self.fitness
    }

    fn get_visuals(&self, ind_data: &PackerIndividualData) -> (f64, f64) {
        let n = self.order.len();
        if n == 0 {
            return (0.0, 0.0);
        }

        // 1) Permutation smoothness: how "locally ordered" the permutation is.
        //    Small |Δ| -> smooth, large |Δ| -> scrambled.
        let mut adj_sum: u64 = 0;
        for i in 0..(n - 1) {
            let a = self.order[i] as i32;
            let b = self.order[i + 1] as i32;
            adj_sum += (a - b).unsigned_abs() as u64;
        }

        // Maximal possible adjacency sum (very rough upper bound):
        // assume worst case ~ (n-1)*(n-1)
        let max_adj = ((n as u64).saturating_sub(1)).pow(2).max(1);
        let perm_smooth = 1.0 - (adj_sum as f64 / max_adj as f64);
        // perm_smooth ~1.0 => "nice smooth" permutation
        // perm_smooth ~0.0 => "very scrambled"

        // 2) Rotation features
        let mut rot_count = 0u32;
        let mut rot_pos_sum = 0.0;

        for (i, &rot) in self.rotations.iter().enumerate() {
            if rot {
                rot_count += 1;
                // normalized position in [0,1]
                rot_pos_sum += i as f64 / (n - 1).max(1) as f64;
            }
        }

        let rot_ratio = rot_count as f64 / n as f64; // 0..1
        let rot_center = if rot_count > 0 {
            rot_pos_sum / rot_count as f64 // 0..1
        } else {
            0.5 // no rotations -> neutral
        };

        // 3) Row length normalized
        let row_norm = if ind_data.max_width > 0 {
            self.row_len as f64 / ind_data.max_width as f64
        } else {
            0.0
        };

        // 4) Compose A and B from these features.
        let a = perm_smooth + 0.5 * rot_ratio; // more about permutation
        let b = row_norm + 0.5 * rot_center; // more about row layout

        (a, b)
    }
}

impl Visualise<PackerIndividualData> for PackerIndividual {
    fn visualise(&self, ind_data: &PackerIndividualData) -> RgbImage {
        let (placements, width, height) = self.compute_layout(ind_data);

        let mut img = RgbImage::new(ind_data.screen_width, ind_data.screen_height);

        if width == 0 || height == 0 {
            return img;
        }

        // Scale layout to screen size
        let scale = f32::min(
            ind_data.screen_width as f32 / width as f32,
            ind_data.screen_height as f32 / height as f32,
        );

        for (i, (x, y, w, h)) in placements.iter().enumerate() {
            let xf = (*x as f32) * scale;
            let yf = (*y as f32) * scale;
            let wf = (*w as f32) * scale;
            let hf = (*h as f32) * scale;

            let rx = xf.round() as i32;
            let ry = yf.round() as i32;
            let rw = wf.max(1.0).round() as u32;
            let rh = hf.max(1.0).round() as u32;

            let rect = Rect::at(rx, ry).of_size(rw, rh);

            // Simple pseudo-color based on index
            let r = (50 + (i * 37) % 205) as u8;
            let g = (80 + (i * 71) % 175) as u8;
            let b = (100 + (i * 53) % 155) as u8;

            draw_filled_rect_mut(&mut img, rect, Rgb([r, g, b]));
        }

        img
    }
}
