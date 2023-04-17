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
    // Get a number value from the JSON data by key
    pub fn get_num(&self, key: &str, default: Option<f64>) -> Result<f64, String> {
        match self.json.find_path(&[key]) {
            Some(value) => match value.as_f64() {
                Some(num) => Ok(num),
                None => Err(format!("Value for key '{}' is not a number", key)),
            },
            None => match default {
                Some(num) => Ok(num),
                None => Err(format!("Key '{}' not found in JSON", key)),
            },
        }
    }

    // Update a number value in the JSON data by key
    pub fn update_num<T: From<f64>>(&self, key: &str, value: &mut T) -> Result<(), String> {
        *value = T::from(self.get_num(key, None)?);
        Ok(())
    }

    // Get a boolean value from the JSON data by key
    pub fn get_bool(&self, key: &str, default: Option<bool>) -> Result<bool, String> {
        match self.json.find_path(&[key]) {
            Some(value) => match value.as_boolean() {
                Some(bool_value) => Ok(bool_value),
                None => Err(format!("Value for key '{}' is not a boolean", key)),
            },
            None => match default {
                Some(bool_value) => Ok(bool_value),
                None => Err(format!("Key '{}' not found in JSON", key)),
            },
        }
    }

    // Update a boolean value in the JSON data by key
    pub fn update_bool(&self, key: &str, value: &mut bool) -> Result<(), String> {
        *value = self.get_bool(key, None)?;
        Ok(())
    }

    pub fn get_val<T>(&self, key: &str, default: Option<T>) -> Result<T, String>
    where
        T: FromStr,
        <T as FromStr>::Err: Debug,
        <T as FromStr>::Err: std::fmt::Display,
    {
        match self.json.find_path(&[key]) {
            Some(value) => match T::from_str(value.as_string().unwrap()) {
                Ok(value) => Ok(value),
                Err(err) => Err(format!("Converting value to T failed: '{}'", err)),
            },
            None => match default {
                Some(value) => Ok(value),
                None => Err(format!("Key '{}' not found in JSON", key)),
            },
        }
    }

    pub fn update_val<T>(&self, key: &str, value: &mut T) -> Result<(), String>
    where
        T: FromStr,
        <T as FromStr>::Err: Debug,
        <T as FromStr>::Err: std::fmt::Display,
    {
        *value = self.get_val(key, None)?;
        Ok(())
    }

    pub fn new(config_filename: &str) -> Self {
        let mut file = File::open(config_filename).unwrap();
        let mut data = String::new();
        file.read_to_string(&mut data).unwrap();

        let json = Json::from_str(&data).unwrap();

        Config { json }
    }
}

#[cfg(test)]
mod tests {
    use crate::config::Config;
    use rustc_serialize::json::Json;
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
    fn test_num() {
        let config = Config {
            json: Json::from_str("{\"pop_width\": 3, \"pop_height\": 4, \"test_enum\":\"bar\", \"another_test_enum\":\"foo\", \"test_bool\":true, \"another_test_bool\":false }").unwrap(),
        };

        let mut num = config.get_num("pop_width", None).unwrap();
        assert_eq!(num as u32, 3);

        assert!(config.update_num("pop_height", &mut num).is_ok());
        assert_eq!(num as u32, 4);

        let mut test_enum: TestEnum = config.get_val("test_enum", None).unwrap();
        assert_eq!(test_enum, TestEnum::Bar);

        assert!(config
            .update_val("another_test_enum", &mut test_enum)
            .is_ok());
        assert_eq!(test_enum, TestEnum::Foo);

        let mut test_bool = config.get_bool("test_bool", None).unwrap();
        assert_eq!(test_bool, true);

        assert!(config
            .update_bool("another_test_bool", &mut test_bool)
            .is_ok());
        assert_eq!(test_bool, false);
    }
}
