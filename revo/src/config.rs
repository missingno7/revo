use rustc_serialize::json::Json;
use std::fmt::Debug;
use std::fs::File;
use std::io::Read;
use std::str::FromStr;

#[derive(Clone)]
pub struct Config {
    pub json: Json,
}

impl Config {
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

        Config {
            json,
        }
        }
}
