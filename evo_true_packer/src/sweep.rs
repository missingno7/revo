use crate::packer::PackerIndividual;

/// Event for the sweep line: rectangle enters (+1) or leaves (-1) at x.
#[derive(Clone, Copy, Debug)]
struct SweepEvent {
    x: i64,
    y1: i64,
    y2: i64,
    delta: i32, // +1 = add coverage, -1 = remove coverage
}

///  Segment tree node stores aggregated coverage statistics over a Y-interval.
///  We maintain:
///  - len: total length of this Y segment
///  - s1: sum(k_i * len_i) over leaves in this segment
///  - s2: sum(k_i^2 * len_i) over leaves in this segment
///    where k_i is the coverage count for leaf i.
///    Total pair contribution over Y is then:
///    pair_sum = 0.5 * (s2 - s1)
struct SegTree {
    len: Vec<f64>,
    s1: Vec<f64>,
    s2: Vec<f64>,
    lazy: Vec<i32>,
}

impl SegTree {
    fn new(y_coords: &[i64]) -> Self {
        // Number of leaf intervals = y_coords.len() - 1
        let m = y_coords.len();
        assert!(m >= 2, "Need at least two distinct y-coordinates");

        let size = 4 * (m - 1);
        let mut st = SegTree {
            len: vec![0.0; size],
            s1: vec![0.0; size],
            s2: vec![0.0; size],
            lazy: vec![0; size],
        };
        st.build(1, 0, (m - 1) as i32, y_coords);
        st
    }

    fn build(&mut self, idx: usize, l: i32, r: i32, y_coords: &[i64]) {
        if l + 1 == r {
            // Leaf: length is difference between consecutive y-coordinates
            let length = (y_coords[r as usize] - y_coords[l as usize]) as f64;
            self.len[idx] = length;
            self.s1[idx] = 0.0;
            self.s2[idx] = 0.0;
        } else {
            let mid = (l + r) / 2;
            self.build(idx * 2, l, mid, y_coords);
            self.build(idx * 2 + 1, mid, r, y_coords);
            self.len[idx] = self.len[idx * 2] + self.len[idx * 2 + 1];
            self.s1[idx] = self.s1[idx * 2] + self.s1[idx * 2 + 1];
            self.s2[idx] = self.s2[idx * 2] + self.s2[idx * 2 + 1];
        }
    }

    /// Apply a coverage delta (can be >1 or <-1) to this whole node segment.
    fn apply(&mut self, idx: usize, delta: i32) {
        if delta == 0 {
            return;
        }
        let d = delta as f64;
        let len = self.len[idx];
        let old_s1 = self.s1[idx];

        // New S1: Σ (k_i + d) * len_i = S1 + d * total_len
        self.s1[idx] = old_s1 + d * len;

        // New S2: Σ (k_i + d)^2 * len_i
        //        = Σ (k_i^2 + 2*d*k_i + d^2) * len_i
        //        = S2 + 2*d*S1 + d^2 * total_len
        self.s2[idx] = self.s2[idx] + 2.0 * d * old_s1 + d * d * len;

        self.lazy[idx] += delta;
    }

    fn push_down(&mut self, idx: usize) {
        let delta = self.lazy[idx];
        if delta != 0 {
            self.apply(idx * 2, delta);
            self.apply(idx * 2 + 1, delta);
            self.lazy[idx] = 0;
        }
    }

    /// Range update: add `delta` to coverage on interval [ql, qr) in leaf indices.
    fn update(&mut self, idx: usize, l: i32, r: i32, ql: i32, qr: i32, delta: i32) {
        if qr <= l || r <= ql {
            return;
        }
        if ql <= l && r <= qr {
            self.apply(idx, delta);
            return;
        }

        self.push_down(idx);
        let mid = (l + r) / 2;
        self.update(idx * 2, l, mid, ql, qr, delta);
        self.update(idx * 2 + 1, mid, r, ql, qr, delta);

        self.s1[idx] = self.s1[idx * 2] + self.s1[idx * 2 + 1];
        self.s2[idx] = self.s2[idx * 2] + self.s2[idx * 2 + 1];
    }

    /// Returns Σ C(k_i, 2) * len_i over all Y, where C(k,2) = k*(k-1)/2.
    fn total_pair_weight(&self) -> f64 {
        // On the root: s1 = Σ k_i * len_i, s2 = Σ k_i^2 * len_i
        // pair_sum = Σ (k_i^2 - k_i)/2 * len_i = (s2 - s1) / 2
        (self.s2[1] - self.s1[1]) * 0.5
    }
}

impl PackerIndividual {
    /// Total overlap area – computed in O(n log n) using a sweep-line algorithm.
    pub fn compute_overlap_area(placements: &[(u32, u32, u32, u32)]) -> f64 {
        let n = placements.len();
        if n == 0 {
            return 0.0;
        }

        let mut events: Vec<SweepEvent> = Vec::with_capacity(2 * n);
        let mut ys: Vec<i64> = Vec::with_capacity(2 * n);

        for &(x, y, w, h) in placements {
            if w == 0 || h == 0 {
                continue;
            }
            let x1 = x as i64;
            let x2 = (x + w) as i64;
            let y1 = y as i64;
            let y2 = (y + h) as i64;

            events.push(SweepEvent {
                x: x1,
                y1,
                y2,
                delta: 1,
            });
            events.push(SweepEvent {
                x: x2,
                y1,
                y2,
                delta: -1,
            });

            ys.push(y1);
            ys.push(y2);
        }

        if events.is_empty() {
            return 0.0;
        }

        // Sort and deduplicate Y coordinates
        ys.sort_unstable();
        ys.dedup();
        if ys.len() < 2 {
            return 0.0;
        }

        // Sort events by x
        events.sort_by_key(|e| e.x);

        // Helper to map y -> leaf index in [0, m-1)
        let m = ys.len();
        let y_index = |y: i64| -> i32 {
            // Safe to unwrap because all y1,y2 were pushed into ys
            ys.binary_search(&y).unwrap() as i32
        };

        // Build segment tree over Y
        let mut seg = SegTree::new(&ys);

        let mut area: f64 = 0.0;
        let mut i: usize = 0;
        let mut prev_x = events[0].x;

        while i < events.len() {
            let x = events[i].x;

            // Accumulate area from prev_x to x, using current pair weight on Y
            let dx = x - prev_x;
            if dx > 0 {
                let pair_weight_y = seg.total_pair_weight();
                area += pair_weight_y * (dx as f64);
                prev_x = x;
            }

            // Process all events at this x
            while i < events.len() && events[i].x == x {
                let e = events[i];
                let y1_idx = y_index(e.y1);
                let y2_idx = y_index(e.y2);

                if y1_idx < y2_idx {
                    // Update coverage on interval [y1_idx, y2_idx)
                    seg.update(1, 0, (m - 1) as i32, y1_idx, y2_idx, e.delta);
                }

                i += 1;
            }
        }

        area
    }
}
