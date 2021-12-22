use rustc_serialize::json::Json;
use std::fs::File;
use std::io::Read;



pub struct PopulationConfig
{
    pub pop_width: usize,
    pub pop_height: usize,
    pub mut_prob: f32,
    pub mut_amount: f32,
    pub crossover_prob: f32,
}

const DEFAULT_POP_WIDTH: usize = 128;
const DEFAULT_POP_HEIGHT: usize = 128;
const DEFAULT_MUT_PROB: f32 = 0.1;
const DEFAULT_MUT_AMOUNT: f32 = 1.0;
const DEFAULT_CROSSOVER_PROB: f32 = 0.1;

impl PopulationConfig {
    pub fn new(config_filename: &str) -> Self
    {
        let mut file = File::open(config_filename).unwrap();
        let mut data = String::new();
        file.read_to_string(&mut data).unwrap();

        let json = Json::from_str(&data).unwrap();

        let pop_width: usize = match json.find_path(&["pop_width"])
        {
            None => DEFAULT_POP_WIDTH,
            Some(data) => {data.as_u64().unwrap() as usize}

        };

        let pop_height: usize = match json.find_path(&["pop_height"])
        {
            None => DEFAULT_POP_HEIGHT,
            Some(data) => {data.as_u64().unwrap() as usize}

        };

        let mut_prob: f32 = match json.find_path(&["mut_prob"])
        {
            None => DEFAULT_MUT_PROB,
            Some(data) => {data.as_f64().unwrap() as f32}

        };

        let mut_amount: f32 = match json.find_path(&["mut_amount"])
        {
            None => DEFAULT_MUT_AMOUNT,
            Some(data) => {data.as_f64().unwrap() as f32}

        };

        let crossover_prob: f32 = match json.find_path(&["crossover_prob"])
        {
            None => DEFAULT_CROSSOVER_PROB,
            Some(data) => {data.as_f64().unwrap() as f32}

        };


        PopulationConfig
        {
            pop_width,
            pop_height,
            mut_prob,
            mut_amount,
            crossover_prob,
        }
    }
}