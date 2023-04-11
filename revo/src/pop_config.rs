use rustc_serialize::json::Json;
use std::fmt::Debug;
use std::fs::File;
use std::io::Read;
use std::str::FromStr;

pub struct PopulationConfig {
    pub pop_width: usize,
    pub pop_height: usize,
    pub mut_prob: f32,
    pub mut_amount: f32,
    pub crossover_prob: f32,
    pub visualise: bool,
    pub json: Json,
}

const DEFAULT_POP_WIDTH: usize = 128;
const DEFAULT_POP_HEIGHT: usize = 128;
const DEFAULT_MUT_PROB: f32 = 0.1;
const DEFAULT_MUT_AMOUNT: f32 = 1.0;
const DEFAULT_CROSSOVER_PROB: f32 = 0.1;
const DEFAULT_VISUALISE: bool = false;

impl PopulationConfig {
    pub fn get_num(&self, key: &str, default: f64) -> f64 {
        match self.json.find_path(&[key]) {
            None => default,
            Some(data) => data.as_f64().unwrap(),
        }
    }

    pub fn get_bool(&self, key: &str, default: bool) -> bool {
        match self.json.find_path(&[key]) {
            None => default,
            Some(data) => data.as_boolean().unwrap(),
        }
    }

    pub fn get_key<Key: FromStr>(&self, key: &str, default: Key) -> Key
    where
        <Key as FromStr>::Err: Debug,
    {
        match self.json.find_path(&[key]) {
            None => default,
            Some(data) => Key::from_str(data.as_string().unwrap()).unwrap(),
        }
    }

    pub fn new(config_filename: &str) -> Self {
        let mut file = File::open(config_filename).unwrap();
        let mut data = String::new();
        file.read_to_string(&mut data).unwrap();

        let json = Json::from_str(&data).unwrap();

        let mut config = PopulationConfig {
            pop_width: DEFAULT_POP_WIDTH,
            pop_height: DEFAULT_POP_HEIGHT,
            mut_prob: DEFAULT_MUT_PROB,
            mut_amount: DEFAULT_MUT_AMOUNT,
            crossover_prob: DEFAULT_CROSSOVER_PROB,
            visualise: DEFAULT_VISUALISE,
            json,
        };

        // Load pre-defined values from config file
        config.pop_width = config.get_num("pop_width", DEFAULT_POP_WIDTH as f64) as usize;
        config.pop_height = config.get_num("pop_height", DEFAULT_POP_HEIGHT as f64) as usize;
        config.mut_prob = config.get_num("mut_prob", DEFAULT_MUT_PROB as f64) as f32;
        config.mut_amount = config.get_num("mut_amount", DEFAULT_MUT_AMOUNT as f64) as f32;
        config.crossover_prob =
            config.get_num("crossover_prob", DEFAULT_CROSSOVER_PROB as f64) as f32;
        config.visualise = config.get_bool("visualise", DEFAULT_VISUALISE);

        config
    }
}
