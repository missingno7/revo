use json5;
use num::{FromPrimitive, Num};
use serde_json::Value;
use std::fmt::Debug;
use std::fs::File;
use std::io::Read;
use std::str::FromStr;

pub const DEFAULT_CONFIG_FILENAME: &str = "config.json5";

#[derive(Clone)]
pub struct Config {
    pub json: Value,
}

impl Config {
    // Get a floating point value from the JSON data by key
    pub fn get_float<T: Num + FromPrimitive>(&self, key: &str) -> Result<Option<T>, String> {
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

    // Update a floating point value from the JSON data by key
    pub fn update_float<T: Num + FromPrimitive>(
        &self,
        key: &str,
        value: &mut T,
    ) -> Result<(), String> {
        if let Some(num) = self.get_float(key)? {
            *value = num;
        }
        Ok(())
    }

    // Get a unsigned integer value from the JSON data by key
    pub fn get_uint<T: Num + FromPrimitive>(&self, key: &str) -> Result<Option<T>, String> {
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

    // Update a unsigned integer value from the JSON data by key
    pub fn update_uint<T: Num + FromPrimitive>(
        &self,
        key: &str,
        value: &mut T,
    ) -> Result<(), String> {
        if let Some(num) = self.get_uint(key)? {
            *value = num;
        }
        Ok(())
    }

    // Get an integer value from the JSON data by key
    pub fn get_int<T: Num + FromPrimitive>(&self, key: &str) -> Result<Option<T>, String> {
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

    // Update an integer value from the JSON data by key
    pub fn update_int<T: Num + FromPrimitive>(
        &self,
        key: &str,
        value: &mut T,
    ) -> Result<(), String> {
        if let Some(num) = self.get_int(key)? {
            *value = num;
        }
        Ok(())
    }

    // Get a boolean value from the JSON data by key
    pub fn get_bool(&self, key: &str) -> Result<Option<bool>, String> {
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

    // Update a boolean value from the JSON data by key
    pub fn update_bool(&self, key: &str, value: &mut bool) -> Result<(), String> {
        if let Some(bool_value) = self.get_bool(key)? {
            *value = bool_value;
        }
        Ok(())
    }

    // Get a T value that implements FromStr from the JSON data by key
    pub fn get_val<T>(&self, key: &str) -> Result<Option<T>, String>
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

    // Update a T value that implements FromStr from the JSON data by key
    pub fn update_val<T>(&self, key: &str, value: &mut T) -> Result<(), String>
    where
        T: FromStr,
        <T as FromStr>::Err: Debug,
        <T as FromStr>::Err: std::fmt::Display,
    {
        if let Some(val) = self.get_val(key)? {
            *value = val;
        }
        Ok(())
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

    #[derive(Clone, PartialEq, Debug)]
    pub enum TestEnum {
        Foo,
        Bar,
    }

    impl FromStr for TestEnum {
        type Err = String;

        fn from_str(s: &str) -> Result<Self, Self::Err> {
            match s.trim().to_lowercase().as_str() {
                "foo" => Ok(TestEnum::Foo),
                "bar" => Ok(TestEnum::Bar),
                _ => Err(format!("Unknown type: {}", s)),
            }
        }
    }

    #[test]
    fn test_int() {
        // Regular integer
        let config = Config::from_str("{\"pop_width\": 3, \"pop_height\": 4}").unwrap();

        let mut test_int: u32 = config.get_uint("pop_width").unwrap().unwrap();
        assert_eq!(test_int, 3);

        config.update_uint("pop_height", &mut test_int).unwrap();
        assert_eq!(test_int, 4);

        // u64 max value integer
        let config = Config::from_str("{\"big_int\": 1844674407370955161}").unwrap();

        let big_int: u64 = config.get_uint("big_int").unwrap().unwrap();
        assert_eq!(big_int, 1844674407370955161);

        // Negative integer
        let config = Config::from_str("{\"negative_int\": -2, \"negative_int_2\": -3}").unwrap();

        // Cannot get negative number as unsigned integer
        assert!(config.get_uint::<u8>("negative_int").is_err());

        let mut test_int: i32 = config.get_int("negative_int").unwrap().unwrap();
        assert_eq!(test_int, -2);

        config.update_int("negative_int_2", &mut test_int).unwrap();
        assert_eq!(test_int, -3);
    }

    #[test]
    fn test_val() {
        let config =
            Config::from_str("{ \"test_enum\":\"bar\", \"another_test_enum\":\"foo\" }").unwrap();

        let mut test_enum: TestEnum = config.get_val("test_enum").unwrap().unwrap();
        assert_eq!(test_enum, TestEnum::Bar);

        assert!(config
            .update_val("another_test_enum", &mut test_enum)
            .is_ok());
        assert_eq!(test_enum, TestEnum::Foo);
    }

    #[test]
    fn test_float() {
        let config = Config::from_str("{\"pop_width\": 3.1, \"pop_height\": -4.2}").unwrap();

        let mut test_num: f32 = config.get_float("pop_width").unwrap().unwrap();
        assert_eq!(test_num, 3.1);

        config.update_float("pop_height", &mut test_num).unwrap();
        assert_eq!(test_num, -4.2);
    }

    #[test]
    fn test_bool() {
        let config = Config::from_str("{\"test_bool\":true, \"another_test_bool\":false}").unwrap();

        let mut test_bool = config.get_bool("test_bool").unwrap().unwrap();
        assert_eq!(test_bool, true);

        assert!(config
            .update_bool("another_test_bool", &mut test_bool)
            .is_ok());
        assert_eq!(test_bool, false);
    }
}
