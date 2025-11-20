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
    xs: Vec<u32>,
    ys: Vec<u32>,
    // true = rectangle is rotated by 90°
    rotations: Vec<bool>,
    untouched: Vec<bool>,
    overlap_surface: u64,
}

impl PackerIndividual {
    fn new_with_data(n_rects: usize, xs: Vec<u32>, ys: Vec<u32>, rotations: Vec<bool>) -> Self {
        assert_eq!(xs.len(), n_rects);
        assert_eq!(ys.len(), n_rects);
        assert_eq!(rotations.len(), n_rects);

        PackerIndividual {
            fitness: 0.0,
            xs,
            ys,
            rotations,
            untouched: vec![false; n_rects],
            overlap_surface: 0,
        }
    }

    /// Returns (x, y, w, h) for the i-th rectangle in *layout* coordinates (float).
    fn rect_f32(&self, ind_data: &PackerIndividualData, i: usize) -> RectPlacement {
        let mut w = ind_data.rects[i].w;
        let mut h = ind_data.rects[i].h;

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
        let mut min_x = u32::MAX;
        let mut min_y = u32::MAX;
        let mut max_x = u32::MIN;
        let mut max_y = u32::MIN;

        let mut raw: Vec<RectPlacement> = Vec::with_capacity(n);

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

        let width = max_x - min_x;
        let height = max_y - min_y;

        // Normalized layout – shift to (0,0) and round to pixels
        let mut placements: Vec<RectPlacement> = Vec::with_capacity(n);
        for (x, y, w, h) in raw {
            let nx = x - min_x;
            let ny = y - min_y;
            let nw = w.max(1);
            let nh = h.max(1);
            placements.push((nx, ny, nw, nh));
        }

        (placements, width, height)
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

    pub fn untouch(&mut self, i: usize, placements: &[RectPlacement]) {
        // If this rectangle is already marked as changed, do nothing.
        if !self.untouched[i] {
            return;
        }

        // If there is no overlap at all, nothing to subtract – just mark as changed.
        if self.overlap_surface == 0 {
            self.untouched[i] = false;
            return;
        }

        let n = self.untouched.len();

        // Subtract old overlaps where rectangle i participates and the other
        // rectangle is still "untouched" (i.e., its old overlaps are still in the sum).
        for j in 0..n {
            if j == i {
                continue;
            }

            if self.untouched[j] {
                if let Some((_, _, ow, oh)) = Self::rect_intersection(placements[i], placements[j])
                {
                    self.overlap_surface -= ow as u64 * oh as u64;
                }
            }
        }

        // Mark i as changed: its overlaps will be recomputed later.
        self.untouched[i] = false;
    }

    pub fn update_overlap_surface(&mut self, placements: &[RectPlacement]) {
        let n = placements.len();

        // Add new overlaps for all pairs where at least one endpoint is changed.
        // We must add each unordered pair {i, j} exactly once.
        for i in 0..n {
            if self.untouched[i] {
                continue;
            }

            for j in 0..n {
                if j == i {
                    continue;
                }

                if self.untouched[j] {
                    // changed–unchanged pair: this pair is not in the sum yet, add it once
                    if let Some((_, _, ow, oh)) =
                        Self::rect_intersection(placements[i], placements[j])
                    {
                        self.overlap_surface += ow as u64 * oh as u64;
                    }
                } else {
                    // changed–changed pair: add only once, for i < j
                    if i < j {
                        if let Some((_, _, ow, oh)) =
                            Self::rect_intersection(placements[i], placements[j])
                        {
                            self.overlap_surface += ow as u64 * oh as u64;
                        }
                    }
                }
            }
        }

        // All rectangles are now up-to-date: their overlaps are part of overlap_surface.
        for i in 0..n {
            self.untouched[i] = true;
        }
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
                rng.gen_range(0..=max_x)
            } else {
                0
            };
            let y = if max_y > 0 {
                rng.gen_range(0..=max_y)
            } else {
                0
            };

            xs.push(x);
            ys.push(y);
            rotations.push(rot);
        }

        PackerIndividual::new_with_data(n, xs, ys, rotations)
    }

    // mutate_into: in-place mutation without allocating new vectors
    fn mutate_into(
        &self,
        target: &mut Self,
        ind_data: &PackerIndividualData,
        rng: &mut SmallRng,
        _mut_prob: f32,
        _mut_amount: f32,
    ) {
        let n = ind_data.rects.len();
        let move_prob = ind_data.move_prob;
        let rot_prob = ind_data.rot_prob;
        let swap_prob = ind_data.swap_prob;
        let sigma = ind_data.move_amount as f32;

        // Ensure target has correct length
        if target.xs.len() != n {
            target.xs.resize(n, 0);
            target.ys.resize(n, 0);
            target.rotations.resize(n, false);
        }

        // Copy current genome into target
        target.xs.copy_from_slice(&self.xs);
        target.ys.copy_from_slice(&self.ys);
        target.rotations.copy_from_slice(&self.rotations);
        target.untouched.copy_from_slice(&self.untouched);
        target.overlap_surface = self.overlap_surface;

        // Precompute Gaussian
        let normal = if sigma > 0.0 {
            Some(Normal::new(0.0, sigma).unwrap())
        } else {
            None
        };

        let (placements, _, _) = target.compute_layout(ind_data);

        for i in 0..n {
            // Movement
            if rng.gen::<f32>() < move_prob {
                if let Some(dist) = &normal {
                    target.untouch(i, &placements);
                    let dx = dist.sample(rng);
                    let dy = dist.sample(rng);
                    target.xs[i] += dx as u32;
                    target.ys[i] += dy as u32;
                }
            }

            // Do not flip squares
            if ind_data.rects[i].w != ind_data.rects[i].h {
                // Rotation flip
                if rng.gen::<f32>() < rot_prob {
                    target.untouch(i, &placements);
                    target.rotations[i] = !target.rotations[i];
                }
            }

            // Swap
            if rng.gen::<f32>() < swap_prob {
                let mut j = rng.gen_range(0..n);
                if j == i {
                    j = (j + 1) % n;
                }

                // Do not switch same shaped rectangles
                if !(ind_data.rects[i].w == ind_data.rects[j].w
                    && ind_data.rects[i].h == ind_data.rects[j].h)
                {
                    target.untouch(i, &placements);
                    target.untouch(j, &placements);

                    target.xs.swap(i, j);
                    target.ys.swap(i, j);
                    target.rotations.swap(i, j);
                }
            }
        }
    }

    // crossover_into: writes crossover result directly into target without allocations
    fn crossover_into(
        &self,
        another_ind: &Self,
        target: &mut Self,
        _ind_data: &PackerIndividualData,
        rng: &mut SmallRng,
    ) {
        let n = self.xs.len();

        // Resize target if necessary
        if target.xs.len() != n {
            target.xs.resize(n, 0);
            target.ys.resize(n, 0);
            target.rotations.resize(n, false);
        }

        for i in 0..n {
            let alpha: f32 = rng.gen();

            // Interpolated crossover
            let x = (alpha * self.xs[i] as f32 + (1.0 - alpha) * another_ind.xs[i] as f32) as u32;

            let y = (alpha * self.ys[i] as f32 + (1.0 - alpha) * another_ind.ys[i] as f32) as u32;

            let rot = if rng.gen_bool(0.5) {
                self.rotations[i]
            } else {
                another_ind.rotations[i]
            };

            target.xs[i] = x;
            target.ys[i] = y;
            target.rotations[i] = rot;
        }
        target.overlap_surface = 0;
        target.untouched = vec![false; n];
    }

    fn count_fitness(&mut self, ind_data: &PackerIndividualData) {
        let (placements, width, height) = self.compute_layout(ind_data);

        if width == 0 || height == 0 || placements.is_empty() {
            self.fitness = f64::NEG_INFINITY;
            return;
        }

        let area = width as u64 * height as u64;

        self.update_overlap_surface(&placements);

        let lambda = ind_data.overlap_penalty;
        self.fitness = -(area as f64 + lambda * self.overlap_surface as f64);
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

#[cfg(test)]
mod tests {
    use super::*;

    /// Total overlap area – naive O(n²) version, but using the shared intersection helper.
    fn compute_overlap_area_simple(placements: &[RectPlacement]) -> u64 {
        let mut overlap: u64 = 0;

        for (i, &r1) in placements.iter().enumerate() {
            for &r2 in placements.iter().skip(i + 1) {
                if let Some((_, _, ow, oh)) = PackerIndividual::rect_intersection(r1, r2) {
                    overlap += (ow as u64) * (oh as u64);
                }
            }
        }

        overlap
    }

    #[test]
    fn test_overlap_opt() {
        let n_rects = 5;
        let xs: Vec<u32> = vec![1, 2, 3, 4, 5];
        let ys: Vec<u32> = vec![1, 2, 3, 4, 5];
        let rotations: Vec<bool> = vec![false, false, true, false, true];

        let ind_data: PackerIndividualData =
            PackerIndividualData::new(5, 5, 5, 50, 50, 0.0, 0.0, 0.0, 0, 1.0);

        let mut ind = PackerIndividual::new_with_data(n_rects, xs, ys, rotations);

        let (placements, _, _) = ind.compute_layout(&ind_data);

        let overlap_area_gt = compute_overlap_area_simple(&placements);

        ind.update_overlap_surface(&placements);

        assert_eq!(ind.overlap_surface, overlap_area_gt);

        ind.untouch(1, &placements);
        ind.xs[1] += 5;

        assert_ne!(ind.overlap_surface, overlap_area_gt);

        // Recompute placements
        let (placements, _, _) = ind.compute_layout(&ind_data);
        let overlap_area_gt = compute_overlap_area_simple(&placements);

        ind.update_overlap_surface(&placements);

        assert_eq!(ind.overlap_surface, overlap_area_gt);

        ind.untouch(1, &placements);
        ind.xs[1] += 5;
        ind.untouch(2, &placements);
        ind.xs[2] += 1;

        // Recompute placements
        let (placements, _, _) = ind.compute_layout(&ind_data);
        let overlap_area_gt = compute_overlap_area_simple(&placements);

        ind.update_overlap_surface(&placements);

        assert_eq!(ind.overlap_surface, overlap_area_gt);

        for i in 0..placements.len() - 1 {
            ind.untouch(i, &placements);
        }

        assert_eq!(ind.overlap_surface, 0);

        ind.update_overlap_surface(&placements);
        assert_eq!(ind.overlap_surface, overlap_area_gt);

        ind.update_overlap_surface(&placements);
        assert_eq!(ind.overlap_surface, overlap_area_gt);
    }
}
