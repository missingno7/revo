use rand::prelude::ThreadRng;
use rand::Rng;
use revo::config::Config;
use revo::utils::Coord;
use std::str::FromStr;

const DEFAULT_N_CITIES: u32 = 500;
const DEFAULT_SCREEN_WIDTH: u32 = 1000;
const DEFAULT_SCREEN_HEIGHT: u32 = 1000;
const DEFAULT_SHIFT_PROB: f64 = 0.4;
const DEFAULT_REV_PROB: f64 = 0.4;
const DEFAULT_INIT_TYPE: SalesmanInitType = SalesmanInitType::GreedyJoining;

#[derive(Clone)]
pub enum SalesmanInitType {
    Naive,
    Noise,
    Insertion,
    GreedyJoining,
}

impl FromStr for SalesmanInitType {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.trim().to_lowercase().as_str() {
            "naive" => Ok(SalesmanInitType::Naive),
            "noise" => Ok(SalesmanInitType::Noise),
            "insertion" => Ok(SalesmanInitType::Insertion),
            "greedy" => Ok(SalesmanInitType::GreedyJoining),
            _ => Err(format!("Unknown type: {}", s)),
        }
    }
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

impl SalesmanIndividualData {
    pub fn new(
        rng: &mut ThreadRng,
        n_cities: u32,
        screen_width: u32,
        screen_height: u32,
        shift_prob: f64,
        rev_prob: f64,
        init_type: SalesmanInitType,
    ) -> Self {
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

    pub fn from_config(rng: &mut ThreadRng, config: &Config) -> Self {
        Self::new(
            rng,
            config
                .get_int("n_cities")
                .unwrap()
                .unwrap_or(DEFAULT_N_CITIES),
            config
                .get_int("screen_width")
                .unwrap()
                .unwrap_or(DEFAULT_SCREEN_WIDTH),
            config
                .get_int("screen_height")
                .unwrap()
                .unwrap_or(DEFAULT_SCREEN_HEIGHT),
            config
                .get_float("shift_prob")
                .unwrap()
                .unwrap_or(DEFAULT_SHIFT_PROB),
            config
                .get_float("rev_prob")
                .unwrap()
                .unwrap_or(DEFAULT_REV_PROB),
            config
                .get_val("init_type")
                .unwrap()
                .unwrap_or(DEFAULT_INIT_TYPE),
        )
    }
}
