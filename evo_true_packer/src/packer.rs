use crate::packer_data::PackerIndividualData;
use image::{Rgb, RgbImage};
use imageproc::drawing::draw_filled_rect_mut;
use imageproc::rect::Rect;
use rand::rngs::SmallRng;
use rand::Rng;
use rand_distr::{Distribution, Normal};
use revo::evo_individual::{EvoIndividual, Visualise};

/// x, y, w, h – in the internal layout coordinate system
pub type RectPlacement = (u32, u32, u32, u32);
/// placements, total_width, total_height
pub type LayoutResult = (Vec<RectPlacement>, u32, u32);

#[derive(Clone, Debug)]
pub struct PackerIndividual {
    pub fitness: f64,
    // Top-left corner coordinates of each rectangle
    xs: Vec<f32>,
    ys: Vec<f32>,
    // true = rectangle is rotated by 90°
    rotations: Vec<bool>,
}

impl PackerIndividual {
    fn new_with_data(n_rects: usize, xs: Vec<f32>, ys: Vec<f32>, rotations: Vec<bool>) -> Self {
        assert_eq!(xs.len(), n_rects);
        assert_eq!(ys.len(), n_rects);
        assert_eq!(rotations.len(), n_rects);

        PackerIndividual {
            fitness: 0.0,
            xs,
            ys,
            rotations,
        }
    }

    /// Returns (x, y, w, h) for the i-th rectangle in *layout* coordinates (float).
    fn rect_f32(&self, ind_data: &PackerIndividualData, i: usize) -> (f32, f32, f32, f32) {
        let mut w = ind_data.rects[i].w as f32;
        let mut h = ind_data.rects[i].h as f32;

        if self.rotations[i] {
            std::mem::swap(&mut w, &mut h);
        }

        (self.xs[i], self.ys[i], w, h)
    }

    /// Computes the layout – shifts it so that min_x = 0 and min_y = 0.
    fn compute_layout(&self, ind_data: &PackerIndividualData) -> LayoutResult {
        let n = ind_data.rects.len();
        if n == 0 {
            return (Vec::new(), 0, 0);
        }

        // First, look at the float coordinates and compute their bounding box
        let mut min_x = f32::INFINITY;
        let mut min_y = f32::INFINITY;
        let mut max_x = f32::NEG_INFINITY;
        let mut max_y = f32::NEG_INFINITY;

        let mut raw: Vec<(f32, f32, f32, f32)> = Vec::with_capacity(n);

        for i in 0..n {
            let (x, y, w, h) = self.rect_f32(ind_data, i);
            raw.push((x, y, w, h));

            if x < min_x {
                min_x = x;
            }
            if y < min_y {
                min_y = y;
            }
            if x + w > max_x {
                max_x = x + w;
            }
            if y + h > max_y {
                max_y = y + h;
            }
        }

        if !min_x.is_finite() || !min_y.is_finite() || !max_x.is_finite() || !max_y.is_finite() {
            return (Vec::new(), 0, 0);
        }

        let mut width = (max_x - min_x).ceil() as i64;
        let mut height = (max_y - min_y).ceil() as i64;

        if width <= 0 {
            width = 1;
        }
        if height <= 0 {
            height = 1;
        }

        // Normalized layout – shift to (0,0) and round to pixels
        let mut placements: Vec<RectPlacement> = Vec::with_capacity(n);
        for (x, y, w, h) in raw {
            let nx = (x - min_x).round().max(0.0) as u32;
            let ny = (y - min_y).round().max(0.0) as u32;
            let nw = w.max(1.0).round() as u32;
            let nh = h.max(1.0).round() as u32;
            placements.push((nx, ny, nw, nh));
        }

        (placements, width as u32, height as u32)
    }

    /// Total overlap area – sum of intersections of all rectangle pairs.
    fn compute_overlap_area_simple(placements: &[(u32, u32, u32, u32)]) -> f64 {
        let mut overlap: f64 = 0.0;

        for (i, &(x1, y1, w1, h1)) in placements.iter().enumerate() {
            let x1_min = x1 as i64;
            let x1_max = (x1 + w1) as i64;
            let y1_min = y1 as i64;
            let y1_max = (y1 + h1) as i64;

            for &(x2, y2, w2, h2) in placements.iter().skip(i + 1) {
                let x2_min = x2 as i64;
                let x2_max = (x2 + w2) as i64;
                let y2_min = y2 as i64;
                let y2_max = (y2 + h2) as i64;

                let overlap_w = (x1_max.min(x2_max) - x1_min.max(x2_min)).max(0);
                let overlap_h = (y1_max.min(y2_max) - y1_min.max(y2_min)).max(0);

                if overlap_w > 0 && overlap_h > 0 {
                    overlap += (overlap_w as f64) * (overlap_h as f64);
                }
            }
        }

        overlap
    }
}

impl EvoIndividual<PackerIndividualData> for PackerIndividual {
    fn new_randomised(ind_data: &PackerIndividualData, rng: &mut SmallRng) -> Self {
        let n = ind_data.rects.len();
        let mut xs = Vec::with_capacity(n);
        let mut ys = Vec::with_capacity(n);
        let mut rotations = Vec::with_capacity(n);

        for i in 0..n {
            let rect = &ind_data.rects[i];

            let rot = rng.gen_bool(0.5);
            let (w, h) = if rot {
                (rect.h, rect.w)
            } else {
                (rect.w, rect.h)
            };

            // Try to scatter them randomly roughly inside the screen;
            // the layout is relative anyway, so this is not a hard constraint.
            let max_x = ind_data.screen_width.saturating_sub(w);
            let max_y = ind_data.screen_height.saturating_sub(h);

            let x = if max_x > 0 {
                rng.gen_range(0..=max_x) as f32
            } else {
                0.0
            };
            let y = if max_y > 0 {
                rng.gen_range(0..=max_y) as f32
            } else {
                0.0
            };

            xs.push(x);
            ys.push(y);
            rotations.push(rot);
        }

        PackerIndividual::new_with_data(n, xs, ys, rotations)
    }

    fn mutate(
        &mut self,
        ind_data: &PackerIndividualData,
        rng: &mut SmallRng,
        _mut_prob: f32,
        _mut_amount: f32,
    ) {
        let n = ind_data.rects.len();
        let move_prob = ind_data.move_prob;
        let rot_prob = ind_data.rot_prob;
        let swap_prob = ind_data.swap_prob;

        // We interpret move_amount as 1σ
        let sigma = ind_data.move_amount as f32;

        // If sigma <= 0, simply do not move
        let normal = if sigma > 0.0 {
            Some(Normal::new(0.0, sigma).unwrap())
        } else {
            None
        };

        for i in 0..n {
            // Movement – Gaussian-distributed step
            if rng.gen::<f32>() < move_prob {
                if let Some(dist) = &normal {
                    let dx = dist.sample(rng);
                    let dy = dist.sample(rng);

                    // Optionally clamp extreme jumps, e.g. to 3σ
                    let max_jump = 3.0 * sigma;
                    let dx = dx.clamp(-max_jump, max_jump);
                    let dy = dy.clamp(-max_jump, max_jump);

                    self.xs[i] += dx;
                    self.ys[i] += dy;
                }
            }

            // Flip rotation
            if rng.gen::<f32>() < rot_prob {
                self.rotations[i] = !self.rotations[i];
            }

            // Swap – keep it uniformly random
            if rng.gen::<f32>() < swap_prob {
                let mut j = rng.gen_range(0..n);
                if j == i {
                    j = (j + 1) % n;
                }

                self.xs.swap(i, j);
                self.ys.swap(i, j);
                self.rotations.swap(i, j);
            }
        }
    }

    fn crossover(
        &self,
        another_ind: &PackerIndividual,
        _ind_data: &PackerIndividualData,
        rng: &mut SmallRng,
    ) -> PackerIndividual {
        let n = self.xs.len();
        if n == 0 {
            return self.clone();
        }

        let mut xs = Vec::with_capacity(n);
        let mut ys = Vec::with_capacity(n);
        let mut rotations = Vec::with_capacity(n);

        for i in 0..n {
            // Some mixing + interpolation – smoother search space
            let alpha: f32 = rng.gen();
            let use_self = rng.gen_bool(0.5);

            let x = if use_self {
                alpha * self.xs[i] + (1.0 - alpha) * another_ind.xs[i]
            } else {
                alpha * another_ind.xs[i] + (1.0 - alpha) * self.xs[i]
            };

            let y = if use_self {
                alpha * self.ys[i] + (1.0 - alpha) * another_ind.ys[i]
            } else {
                alpha * another_ind.ys[i] + (1.0 - alpha) * self.ys[i]
            };

            let rot = if rng.gen_bool(0.5) {
                self.rotations[i]
            } else {
                another_ind.rotations[i]
            };

            xs.push(x);
            ys.push(y);
            rotations.push(rot);
        }

        PackerIndividual::new_with_data(n, xs, ys, rotations)
    }

    fn count_fitness(&mut self, ind_data: &PackerIndividualData) {
        let (placements, width, height) = self.compute_layout(ind_data);

        if width == 0 || height == 0 || placements.is_empty() {
            self.fitness = f64::NEG_INFINITY;
            return;
        }

        let area = (width as u64 * height as u64) as f64;

        let overlap_area = Self::compute_overlap_area_simple(&placements);

        let lambda = ind_data.overlap_penalty;

        self.fitness = -(area + lambda * overlap_area);
    }

    fn get_fitness(&self) -> f64 {
        self.fitness
    }

    fn get_visuals(&self, ind_data: &PackerIndividualData) -> (f64, f64) {
        // Visual embedding only for debugging / 2D population projection.
        let (placements, width, height) = self.compute_layout(ind_data);

        if placements.is_empty() || width == 0 || height == 0 {
            return (0.0, 0.0);
        }

        let mut a = 0.0f64;
        let mut b = 0.0f64;

        let w = width as f64;
        let h = height as f64;

        for (i, &(x, y, rw, rh)) in placements.iter().enumerate() {
            let fx = x as f64 / w;
            let fy = y as f64 / h;
            let fw = rw as f64 / w;
            let fh = rh as f64 / h;
            let t = i as f64;

            a += (fx + 1.3 * fw + 0.2 * t).sin();
            b += (fy + 0.7 * fh - 0.3 * t).cos();
        }

        for (i, &r) in self.rotations.iter().enumerate() {
            let v = if r { 1.0 } else { -1.0 };
            let t = i as f64;
            a += (v * 2.0 + 0.3 * t).sin();
            b += (v * 2.0 - 0.5 * t).cos();
        }

        (a, b)
    }
}
impl Visualise<PackerIndividualData> for PackerIndividual {
    fn visualise(&self, ind_data: &PackerIndividualData) -> RgbImage {
        let (placements, width, height) = self.compute_layout(ind_data);

        let mut img = RgbImage::new(ind_data.screen_width, ind_data.screen_height);

        if width == 0 || height == 0 || placements.is_empty() {
            return img;
        }

        // Scale layout to screen size
        let scale = f32::min(
            ind_data.screen_width as f32 / width as f32,
            ind_data.screen_height as f32 / height as f32,
        );

        // -------------------------------
        // Draw rectangles (original code)
        // -------------------------------
        for (i, &(x, y, w, h)) in placements.iter().enumerate() {
            let sx = (x as f32 * scale).round() as i32;
            let sy = (y as f32 * scale).round() as i32;
            let sw = (w as f32 * scale).max(1.0).round() as u32;
            let sh = (h as f32 * scale).max(1.0).round() as u32;

            let rect = Rect::at(sx, sy).of_size(sw, sh);

            // Pseudo colors based on index
            let r = (50 + (i * 37) % 205) as u8;
            let g = (80 + (i * 71) % 175) as u8;
            let b = (100 + (i * 53) % 155) as u8;

            draw_filled_rect_mut(&mut img, rect, Rgb([r, g, b]));
        }

        // ----------------------------------------------------
        // Highlight overlaps - bright red overlay (semi-opaque)
        // ----------------------------------------------------
        for (i, &(x1, y1, w1, h1)) in placements.iter().enumerate() {
            let x1_min = x1 as i32;
            let x1_max = (x1 + w1) as i32;
            let y1_min = y1 as i32;
            let y1_max = (y1 + h1) as i32;

            // iterate only over rectangles with index > i
            for &(x2, y2, w2, h2) in placements.iter().skip(i + 1) {
                let x2_min = x2 as i32;
                let x2_max = (x2 + w2) as i32;
                let y2_min = y2 as i32;
                let y2_max = (y2 + h2) as i32;

                // Compute intersection
                let ox1 = x1_min.max(x2_min);
                let oy1 = y1_min.max(y2_min);
                let ox2 = x1_max.min(x2_max);
                let oy2 = y1_max.min(y2_max);

                if ox2 > ox1 && oy2 > oy1 {
                    // Convert to screen coords
                    let sx = (ox1 as f32 * scale).round() as i32;
                    let sy = (oy1 as f32 * scale).round() as i32;
                    let sw = ((ox2 - ox1) as f32 * scale).max(1.0).round() as u32;
                    let sh = ((oy2 - oy1) as f32 * scale).max(1.0).round() as u32;

                    let overlap_rect = Rect::at(sx, sy).of_size(sw, sh);

                    // Bright red overlay
                    draw_filled_rect_mut(&mut img, overlap_rect, Rgb([255, 0, 0]));
                }
            }
        }

        img
    }
}
