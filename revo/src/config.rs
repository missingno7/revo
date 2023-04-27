use json5;
use num::{FromPrimitive, Num};
use serde_json::Value;
use std::fmt::Debug;
use std::fs::File;
use std::io::Read;
use std::str::FromStr;
use strum::IntoEnumIterator;

pub const DEFAULT_CONFIG_FILENAME: &str = "config.json5";

#[derive(Clone)]
pub struct Config {
    pub json: Value,
}

impl Config {
    // Get a floating point value from the JSON data by key
    pub fn may_get_float<T: Num + FromPrimitive>(&self, key: &str) -> Result<Option<T>, String> {
        match self.json.get(key) {
            // Value found in JSON
            Some(value) => match value.as_f64() {
                Some(num) => Ok(Some(T::from_f64(num).unwrap())),
                None => Err(format!("Value for key '{}' is not a float", key)),
            },
            // Value not found in JSON - use default
            None => Ok(None),
        }
    }

    pub fn get_float<T: Num + FromPrimitive>(&self, key: &str) -> Result<T, String> {
        match self.may_get_float(key)? {
            Some(value) => Ok(value),
            None => Err(format!("Value for key '{}' not found", key)),
        }
    }

    // Get a unsigned integer value from the JSON data by key
    pub fn may_get_uint<T: Num + FromPrimitive>(&self, key: &str) -> Result<Option<T>, String> {
        match self.json.get(key) {
            // Value found in JSON
            Some(value) => match value.as_u64() {
                Some(num) => Ok(Some(T::from_u64(num).unwrap())),
                None => Err(format!("Value for key '{}' is not an uint", key)),
            },
            // Value not found in JSON - use default
            None => Ok(None),
        }
    }

    pub fn get_uint<T: Num + FromPrimitive>(&self, key: &str) -> Result<T, String> {
        match self.may_get_uint(key)? {
            Some(value) => Ok(value),
            None => Err(format!("Value for key '{}' not found", key)),
        }
    }

    // Get an integer value from the JSON data by key
    pub fn may_get_int<T: Num + FromPrimitive>(&self, key: &str) -> Result<Option<T>, String> {
        match self.json.get(key) {
            // Value found in JSON
            Some(value) => match value.as_i64() {
                Some(num) => Ok(Some(T::from_i64(num).unwrap())),
                None => Err(format!("Value for key '{}' is not an int", key)),
            },
            // Value not found in JSON - use default
            None => Ok(None),
        }
    }

    pub fn get_int<T: Num + FromPrimitive>(&self, key: &str) -> Result<T, String> {
        match self.may_get_int(key)? {
            Some(value) => Ok(value),
            None => Err(format!("Value for key '{}' not found", key)),
        }
    }

    // Get a boolean value from the JSON data by key
    pub fn may_get_bool(&self, key: &str) -> Result<Option<bool>, String> {
        match self.json.get(key) {
            // Value found in JSON
            Some(value) => match value.as_bool() {
                Some(bool_value) => Ok(Some(bool_value)),
                None => Err(format!("Value for key '{}' is not a boolean", key)),
            },
            // Value not found in JSON - use default
            None => Ok(None),
        }
    }

    pub fn get_bool(&self, key: &str) -> Result<bool, String> {
        match self.may_get_bool(key)? {
            Some(value) => Ok(value),
            None => Err(format!("Value for key '{}' not found", key)),
        }
    }

    // Get a T enum that implements FromStr from the JSON data by key
    pub fn may_get_enum<T>(&self, key: &str) -> Result<Option<T>, String>
    where
        T: FromStr + IntoEnumIterator + std::fmt::Display,
        <T as FromStr>::Err: Debug,
        <T as FromStr>::Err: std::fmt::Display,
    {
        match self.json.get(key) {
            // Value found in JSON
            Some(value) => match T::from_str(value.as_str().unwrap()) {
                Ok(value) => Ok(Some(value)),
                Err(_) => {
                    let possible_values = T::iter().map(|v| v.to_string()).collect::<Vec<_>>();
                    Err(format!(
                        "Unknown value \"{}\", options are: {:?}",
                        value, possible_values
                    ))
                }
            },
            // Value not found in JSON - use default
            None => Ok(None),
        }
    }

    pub fn get_enum<T>(&self, key: &str) -> Result<T, String>
    where
        T: FromStr + IntoEnumIterator + std::fmt::Display,
        <T as FromStr>::Err: Debug,
        <T as FromStr>::Err: std::fmt::Display,
    {
        match self.may_get_enum(key)? {
            Some(value) => Ok(value),
            None => Err(format!("Value for key '{}' not found", key)),
        }
    }

    // Get a T value that implements FromStr from the JSON data by key
    pub fn may_get_val<T>(&self, key: &str) -> Result<Option<T>, String>
    where
        T: FromStr,
        <T as FromStr>::Err: Debug,
        <T as FromStr>::Err: std::fmt::Display,
    {
        match self.json.get(key) {
            // Value found in JSON
            Some(value) => match T::from_str(value.as_str().unwrap()) {
                Ok(value) => Ok(Some(value)),
                Err(err) => Err(format!("Converting value to T failed: '{}'", err)),
            },
            // Value not found in JSON - use default
            None => Ok(None),
        }
    }

    pub fn get_val<T>(&self, key: &str) -> Result<T, String>
    where
        T: FromStr,
        <T as FromStr>::Err: Debug,
        <T as FromStr>::Err: std::fmt::Display,
    {
        match self.may_get_val(key)? {
            Some(value) => Ok(value),
            None => Err(format!("Value for key '{}' not found", key)),
        }
    }

    pub fn new(config_filename: &str) -> Self {
        let mut file = File::open(config_filename).unwrap();
        let mut data = String::new();
        file.read_to_string(&mut data).unwrap();

        let json = json5::from_str(&data).unwrap();

        Config { json }
    }
}

impl FromStr for Config {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let json = json5::from_str(s).unwrap();
        Ok(Config { json })
    }
}

#[cfg(test)]
mod tests {
    use crate::config::Config;
    use std::str::FromStr;
    use strum_macros::{Display, EnumIter, EnumString};

    #[derive(Clone, PartialEq, Debug, EnumString, EnumIter, Display)]
    pub enum TestEnum {
        #[strum(serialize = "foo")]
        Foo,
        #[strum(serialize = "bar")]
        Bar,
    }

    #[test]
    fn test_int() {
        // Regular integer
        let config = Config::from_str("{\"pop_width\": 3, \"pop_height\": 4}").unwrap();

        let test_int: u32 = config.get_uint("pop_width").unwrap();
        assert_eq!(test_int, 3);

        // u64 max value integer
        let config = Config::from_str("{\"big_int\": 1844674407370955161}").unwrap();

        let big_int: u64 = config.get_uint("big_int").unwrap();
        assert_eq!(big_int, 1844674407370955161);

        // Negative integer
        let config = Config::from_str("{\"negative_int\": -2, \"negative_int_2\": -3}").unwrap();

        // Cannot get negative number as unsigned integer
        assert!(config.get_uint::<u8>("negative_int").is_err());

        let test_int: i32 = config.get_int("negative_int").unwrap();
        assert_eq!(test_int, -2);
    }

    #[test]
    fn test_val() {
        let config =
            Config::from_str("{ \"test_enum\":\"bar\", \"another_test_enum\":\"foo\" }").unwrap();

        let test_enum: TestEnum = config.get_val("test_enum").unwrap();
        assert_eq!(test_enum, TestEnum::Bar);
    }

    #[test]
    fn test_get_enum() {
        let config =
            Config::from_str("{ \"test_enum\":\"bar\", \"another_test_enum\":\"foo\" }").unwrap();

        let test_enum: TestEnum = config.get_enum("test_enum").unwrap();
        assert_eq!(test_enum, TestEnum::Bar);
    }

    #[test]
    fn test_float() {
        let config = Config::from_str("{\"pop_width\": 3.1, \"pop_height\": -4.2}").unwrap();

        let test_num: f32 = config.get_float("pop_width").unwrap();
        assert_eq!(test_num, 3.1);
    }

    #[test]
    fn test_bool() {
        let config = Config::from_str("{\"test_bool\":true, \"another_test_bool\":false}").unwrap();

        let test_bool = config.get_bool("test_bool").unwrap();
        assert_eq!(test_bool, true);
    }
}
