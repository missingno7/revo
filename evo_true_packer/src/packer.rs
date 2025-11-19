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
    pub fn compute_layout(&self, ind_data: &PackerIndividualData) -> LayoutResult {
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

    /// Computes intersection of two rectangles in layout coordinates.
    /// Returns (x, y, w, h) of the intersection, or None if they do not overlap.
    fn rect_intersection(a: RectPlacement, b: RectPlacement) -> Option<RectPlacement> {
        let (x1, y1, w1, h1) = a;
        let (x2, y2, w2, h2) = b;

        let x1_min = x1 as i64;
        let x1_max = (x1 + w1) as i64;
        let y1_min = y1 as i64;
        let y1_max = (y1 + h1) as i64;

        let x2_min = x2 as i64;
        let x2_max = (x2 + w2) as i64;
        let y2_min = y2 as i64;
        let y2_max = (y2 + h2) as i64;

        let ox1 = x1_min.max(x2_min);
        let oy1 = y1_min.max(y2_min);
        let ox2 = x1_max.min(x2_max);
        let oy2 = y1_max.min(y2_max);

        if ox2 > ox1 && oy2 > oy1 {
            let ow = (ox2 - ox1) as u32;
            let oh = (oy2 - oy1) as u32;
            Some((ox1 as u32, oy1 as u32, ow, oh))
        } else {
            None
        }
    }

    /// Total overlap area – naive O(n²) version, but using the shared intersection helper.
    fn compute_overlap_area_simple(placements: &[RectPlacement]) -> f64 {
        let mut overlap: f64 = 0.0;

        for (i, &r1) in placements.iter().enumerate() {
            for &r2 in placements.iter().skip(i + 1) {
                if let Some((_, _, ow, oh)) = Self::rect_intersection(r1, r2) {
                    overlap += (ow as f64) * (oh as f64);
                }
            }
        }

        overlap
    }

    /// Computes the packing density in percent.
    /// density = (sum of areas of rectangles) / (bounding box area) * 100.0
    pub fn compute_density(placements: &[RectPlacement], width: u32, height: u32) -> f64 {
        if width == 0 || height == 0 {
            return 0.0;
        }

        let mut total_rect_area = 0u64;

        for &(_, _, w, h) in placements {
            total_rect_area += w as u64 * h as u64;
        }

        let bbox_area = width as u64 * height as u64;

        if bbox_area == 0 {
            return 0.0;
        }

        (total_rect_area as f64 / bbox_area as f64) * 100.0
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
                    //let max_jump = 3.0 * sigma;
                    //let dx = dx.clamp(-max_jump, max_jump);
                    //let dy = dy.clamp(-max_jump, max_jump);

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
        let n = ind_data.rects.len();

        let mut a = 0f64;
        let mut b = 0f64;

        for i in 0..n {
            if i % 2 == 0 {
                a += self.xs[i] as f64;
                b += self.ys[i] as f64;
            } else {
                a -= self.xs[i] as f64;
                b -= self.ys[i] as f64;
            }
        }

        (a, b)
    }
}
impl Visualise<PackerIndividualData> for PackerIndividual {
    fn visualise(&self, ind_data: &PackerIndividualData) -> RgbImage {
        let (placements, width, height) = self.compute_layout(ind_data);
        let scaling_factor: u32 = 5;

        // If there is nothing to draw, return empty image
        if width == 0 || height == 0 || placements.is_empty() {
            return RgbImage::new(1, 1);
        }

        // First render at 1:1 layout resolution
        let mut base_img = RgbImage::new(width, height);

        // --------------------------------------
        // 1) Draw all rectangles in layout space
        // --------------------------------------
        for (i, &(x, y, w, h)) in placements.iter().enumerate() {
            let sx = x as i32;
            let sy = y as i32;
            let sw = w.max(1);
            let sh = h.max(1);

            let rect = Rect::at(sx, sy).of_size(sw, sh);

            // Pseudo-colors based on index
            let r = (50 + (i * 37) % 205) as u8;
            let g = (80 + (i * 71) % 175) as u8;
            let b = (100 + (i * 53) % 155) as u8;

            draw_filled_rect_mut(&mut base_img, rect, Rgb([r, g, b]));
        }

        // -------------------------------------------------------
        // 2) Draw overlaps in layout space (still unscaled)
        // -------------------------------------------------------
        for i in 0..placements.len() {
            let r1 = placements[i];

            for (_, &r2) in placements.iter().enumerate().skip(i + 1) {
                if let Some((ox, oy, ow, oh)) = PackerIndividual::rect_intersection(r1, r2) {
                    let sx = ox as i32;
                    let sy = oy as i32;
                    let sw = ow.max(1);
                    let sh = oh.max(1);

                    let overlap_rect = Rect::at(sx, sy).of_size(sw, sh);

                    // Solid red for overlapping area
                    draw_filled_rect_mut(&mut base_img, overlap_rect, Rgb([255, 0, 0]));
                }
            }
        }

        // -------------------------------------------------------
        // 3) Integer-scale the image by scaling_factor
        // -------------------------------------------------------
        let scaled_w = width * scaling_factor;
        let scaled_h = height * scaling_factor;

        let mut scaled_img = RgbImage::new(scaled_w, scaled_h);

        for y in 0..height {
            for x in 0..width {
                let pixel = base_img.get_pixel(x, y);

                // Write a scaling_factor × scaling_factor block
                for dy in 0..scaling_factor {
                    for dx in 0..scaling_factor {
                        let sx = x * scaling_factor + dx;
                        let sy = y * scaling_factor + dy;
                        scaled_img.put_pixel(sx, sy, *pixel);
                    }
                }
            }
        }

        scaled_img
    }
}
