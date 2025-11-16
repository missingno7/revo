use rand::Rng;
use revo::config::Config;
use revo::evo_individual::EvoIndividualData;
use std::cmp::max;

const DEFAULT_N_RECTS: u32 = 50;
const DEFAULT_RECT_MIN: u32 = 10;
const DEFAULT_RECT_MAX: u32 = 80;
const DEFAULT_SCREEN_WIDTH: u32 = 1000;
const DEFAULT_SCREEN_HEIGHT: u32 = 1000;
const DEFAULT_SWAP_PROB: f32 = 0.4;
const DEFAULT_REVERSE_PROB: f32 = 0.1;

const DEFAULT_HEIGHT_CHANGE_PROB: f32 = 0.5;
const DEFAULT_HEIGHT_CHANGE_AMOUNT: u32 = 5;

#[derive(Clone)]
pub struct RectSpec {
    pub w: u32,
    pub h: u32,
}

#[derive(Clone)]
pub struct PackerIndividualData {
    pub rects: Vec<RectSpec>,
    pub screen_width: u32,
    pub screen_height: u32,
    pub total_area: u64,
    pub swap_prob: f32,
    pub reverse_prob: f32,
    pub height_change_prob: f32,
    pub height_change_amount: u32,
    pub max_width: u32,
    pub vis_w1: Vec<f64>,
    pub vis_w2: Vec<f64>,
}

impl EvoIndividualData for PackerIndividualData {
    fn from_config(config: &Config) -> Self {
        Self::new(
            config
                .may_get_int("n_rects")
                .unwrap()
                .unwrap_or(DEFAULT_N_RECTS),
            config
                .may_get_int("rect_min")
                .unwrap()
                .unwrap_or(DEFAULT_RECT_MIN),
            config
                .may_get_int("rect_max")
                .unwrap()
                .unwrap_or(DEFAULT_RECT_MAX),
            config
                .may_get_int("screen_width")
                .unwrap()
                .unwrap_or(DEFAULT_SCREEN_WIDTH),
            config
                .may_get_int("screen_height")
                .unwrap()
                .unwrap_or(DEFAULT_SCREEN_HEIGHT),
            config
                .may_get_float("swap_prob")
                .unwrap()
                .unwrap_or(DEFAULT_SWAP_PROB),
            config
                .may_get_float("reverse_prob")
                .unwrap()
                .unwrap_or(DEFAULT_REVERSE_PROB),
            config
                .may_get_float("height_change_prob")
                .unwrap()
                .unwrap_or(DEFAULT_HEIGHT_CHANGE_PROB),
            config
                .may_get_int("height_change_amount")
                .unwrap()
                .unwrap_or(DEFAULT_HEIGHT_CHANGE_AMOUNT),
        )
    }
}

impl PackerIndividualData {
    #[allow(clippy::too_many_arguments)]
    pub fn new(
        n_rects: u32,
        rect_min: u32,
        rect_max: u32,
        screen_width: u32,
        screen_height: u32,
        swap_prob: f32,
        reverse_prob: f32,
        height_change_prob: f32,
        height_change_amount: u32,
    ) -> Self {
        let mut rng = rand::thread_rng();
        let mut rects = Vec::with_capacity(n_rects as usize);
        let mut total_area: u64 = 0;

        let mut max_width = 0u32;
        for _ in 0..n_rects {
            let w = rng.gen_range(rect_min..=rect_max);
            let h = rng.gen_range(rect_min..=rect_max);
            max_width += max(w, h);

            total_area += (w as u64) * (h as u64);
            rects.push(RectSpec { w, h });
        }

        let dim = n_rects * 2 + 1; // order + rotations + row_len
        let vis_w1: Vec<f64> = (0..dim).map(|_| rng.gen_range(-1.0..1.0)).collect();
        let vis_w2: Vec<f64> = (0..dim).map(|_| rng.gen_range(-1.0..1.0)).collect();

        PackerIndividualData {
            rects,
            screen_width,
            screen_height,
            total_area,
            swap_prob,
            reverse_prob,
            height_change_prob,
            height_change_amount,
            max_width,
            vis_w1,
            vis_w2,
        }
    }
}
