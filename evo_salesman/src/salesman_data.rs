use rand::Rng;
use revo::config::Config;
use revo::evo_individual::EvoIndividualData;
use revo::utils::Coord;
use strum_macros::{Display, EnumIter, EnumString};

const DEFAULT_N_CITIES: u32 = 500;
const DEFAULT_SCREEN_WIDTH: u32 = 1000;
const DEFAULT_SCREEN_HEIGHT: u32 = 1000;
const DEFAULT_SHIFT_PROB: f64 = 0.4;
const DEFAULT_REV_PROB: f64 = 0.4;
const DEFAULT_INIT_TYPE: SalesmanInitType = SalesmanInitType::GreedyJoining;

#[derive(Clone, EnumString, EnumIter, Display)]
pub enum SalesmanInitType {
    #[strum(serialize = "naive")]
    Naive,
    #[strum(serialize = "noise")]
    Noise,
    #[strum(serialize = "insertion")]
    Insertion,
    #[strum(serialize = "greedy")]
    GreedyJoining,
    #[strum(serialize = "two_opt")]
    TwoOpt,
    #[strum(serialize = "auto")]
    Auto,
}

#[derive(Clone)]
pub struct SalesmanIndividualData {
    pub coords: Vec<Coord>,
    pub screen_width: u32,
    pub screen_height: u32,
    pub shift_prob: f64,
    pub rev_prob: f64,
    pub init_type: SalesmanInitType,
}

impl EvoIndividualData for SalesmanIndividualData {
    fn from_config(config: &Config) -> Self {
        Self::new(
            config
                .may_get_int("n_cities")
                .unwrap()
                .unwrap_or(DEFAULT_N_CITIES),
            config
                .may_get_int("screen_width")
                .unwrap()
                .unwrap_or(DEFAULT_SCREEN_WIDTH),
            config
                .may_get_int("screen_height")
                .unwrap()
                .unwrap_or(DEFAULT_SCREEN_HEIGHT),
            config
                .may_get_float("shift_prob")
                .unwrap()
                .unwrap_or(DEFAULT_SHIFT_PROB),
            config
                .may_get_float("rev_prob")
                .unwrap()
                .unwrap_or(DEFAULT_REV_PROB),
            config
                .may_get_enum("init_type")
                .unwrap()
                .unwrap_or(DEFAULT_INIT_TYPE),
        )
    }
}

impl SalesmanIndividualData {
    pub fn new(
        n_cities: u32,
        screen_width: u32,
        screen_height: u32,
        shift_prob: f64,
        rev_prob: f64,
        init_type: SalesmanInitType,
    ) -> Self {
        let mut rng = rand::thread_rng();

        let mut coords: Vec<Coord> = Vec::new();

        for _ in 0..n_cities {
            coords.push(Coord {
                x: rng.gen_range(5..screen_width - 5) as i32,
                y: rng.gen_range(5..screen_height - 5) as i32,
            });
        }

        SalesmanIndividualData {
            coords,
            screen_width,
            screen_height,
            shift_prob,
            rev_prob,
            init_type,
        }
    }
}
