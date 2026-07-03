use rand::Rng;
use revo::config::Config;
use revo::evo_individual::EvoIndividualData;

/// Defaults – you can adjust these in the config
const DEFAULT_N_RECTS: u32 = 50;
const DEFAULT_RECT_MIN: u32 = 10;
const DEFAULT_RECT_MAX: u32 = 80;
const DEFAULT_SCREEN_WIDTH: u32 = 1000;
const DEFAULT_SCREEN_HEIGHT: u32 = 1000;

/// Probability of moving a given rectangle during mutation
const DEFAULT_MOVE_PROB: f32 = 0.4;
/// Probability of flipping rotation during mutation
const DEFAULT_ROT_PROB: f32 = 0.2;
/// Probability of swapping two rectangles in mutation
const DEFAULT_SWAP_PROB: f32 = 0.05;

/// Movement scale (interpreted as 1σ for Gaussian mutation now)
const DEFAULT_MOVE_AMOUNT: u32 = 20;

/// Overlap penalty weight (higher = stronger push toward non-overlapping layouts)
const DEFAULT_OVERLAP_PENALTY: f64 = 1.0;

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

    /// Probability of moving a single rectangle
    pub move_prob: f32,
    /// Probability of flipping rotation of a rectangle
    pub rot_prob: f32,
    /// Probability of swapping parameters of two rectangles
    pub swap_prob: f32,

    /// Mutation movement scale (interpreted as max delta or as σ depending on algorithm)
    pub move_amount: u32,

    /// Penalty multiplier for total overlap area
    pub overlap_penalty: f64,
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
                .may_get_float("move_prob")
                .unwrap()
                .unwrap_or(DEFAULT_MOVE_PROB as f64) as f32,
            config
                .may_get_float("rot_prob")
                .unwrap()
                .unwrap_or(DEFAULT_ROT_PROB as f64) as f32,
            config
                .may_get_float("swap_prob")
                .unwrap()
                .unwrap_or(DEFAULT_SWAP_PROB as f64) as f32,
            config
                .may_get_int("move_amount")
                .unwrap()
                .unwrap_or(DEFAULT_MOVE_AMOUNT),
            config
                .may_get_float("overlap_penalty")
                .unwrap()
                .unwrap_or(DEFAULT_OVERLAP_PENALTY),
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
        move_prob: f32,
        rot_prob: f32,
        swap_prob: f32,
        move_amount: u32,
        overlap_penalty: f64,
    ) -> Self {
        let mut rng = rand::thread_rng();
        let mut rects = Vec::with_capacity(n_rects as usize);

        for _ in 0..n_rects {
            let w = rng.gen_range(rect_min..=rect_max);
            let h = rng.gen_range(rect_min..=rect_max);
            rects.push(RectSpec { w, h });
        }

        PackerIndividualData {
            rects,
            screen_width,
            screen_height,
            move_prob,
            rot_prob,
            swap_prob,
            move_amount,
            overlap_penalty,
        }
    }
}
