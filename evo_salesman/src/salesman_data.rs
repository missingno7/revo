use rand::prelude::ThreadRng;
use rand::Rng;
use revo::pop_config::PopulationConfig;
use revo::utils::Coord;

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

impl SalesmanInitType {
    pub fn from_string(string: &str) -> Self {
        match string {
            "naive" => SalesmanInitType::Naive,
            "noise" => SalesmanInitType::Noise,
            "insertion" => SalesmanInitType::Insertion,
            "greedy" => SalesmanInitType::GreedyJoining,
            _ => panic!("Unknown type"),
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
                x: rng.gen_range(5..screen_width - 5),
                y: rng.gen_range(5..screen_height - 5),
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

    pub fn from_config(rng: &mut ThreadRng, config: &PopulationConfig) -> Self {
        let n_cities: u32 = match config.json.find_path(&["n_cities"]) {
            None => DEFAULT_N_CITIES,
            Some(data) => data.as_u64().unwrap() as u32,
        };

        let screen_width: u32 = match config.json.find_path(&["screen_width"]) {
            None => DEFAULT_SCREEN_WIDTH,
            Some(data) => data.as_u64().unwrap() as u32,
        };

        let screen_height: u32 = match config.json.find_path(&["screen_height"]) {
            None => DEFAULT_SCREEN_HEIGHT,
            Some(data) => data.as_u64().unwrap() as u32,
        };

        let shift_prob: f64 = match config.json.find_path(&["shift_prob"]) {
            None => DEFAULT_SHIFT_PROB,
            Some(data) => data.as_f64().unwrap(),
        };

        let rev_prob: f64 = match config.json.find_path(&["rev_prob"]) {
            None => DEFAULT_REV_PROB,
            Some(data) => data.as_f64().unwrap(),
        };

        let init_type: SalesmanInitType = match config.json.find_path(&["init_type"]) {
            None => DEFAULT_INIT_TYPE,
            Some(data) => SalesmanInitType::from_string(data.as_string().unwrap()),
        };

        Self::new(
            rng,
            n_cities,
            screen_width,
            screen_height,
            shift_prob,
            rev_prob,
            init_type,
        )
    }
}
