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
    pub fn get_num(&self, key: &str, default: Option<f64>) -> Result<f64, String> {
        match self.json.find_path(&[key]) {
            None => match default {
                Some(val) => Ok(val),
                None => Err(format!(
                    "Key '{}' not found and no default value provided",
                    key
                )),
            },
            Some(data) => Ok(data.as_f64().unwrap()),
        }
    }

    pub fn update_num<T: From<f64>>(&self, key: &str, value: &mut T) -> Result<(), String> {
        if let Some(data) = self.json.find_path(&[key]) {
            if let Some(num) = data.as_f64() {
                *value = T::from(num);
                Ok(())
            } else {
                Err(format!("Key '{}' not found", key))
            }
        } else {
            Err(format!("Key '{}' not found", key))
        }
    }

    pub fn get_bool(&self, key: &str, default: Option<bool>) -> Result<bool, String> {
        match self.json.find_path(&[key]) {
            None => match default {
                Some(val) => Ok(val),
                None => Err(format!(
                    "Key '{}' not found and no default value provided",
                    key
                )),
            },
            Some(data) => Ok(data.as_boolean().unwrap()),
        }
    }

    pub fn update_bool(&self, key: &str, value: &mut bool) -> Result<(), String> {
        if let Some(data) = self.json.find_path(&[key]) {
            if let Some(any) = data.as_boolean() {
                *value = any;
                Ok(())
            } else {
                Err(format!("Key '{}' not found", key))
            }
        } else {
            Err(format!("Key '{}' not found", key))
        }
    }

    pub fn get_key<Key>(&self, key: &str, default: Option<Key>) -> Result<Key, String>
    where
        Key: FromStr,
        <Key as FromStr>::Err: Debug,
    {
        match self.json.find_path(&[key]) {
            None => match default {
                Some(val) => Ok(val),
                None => Err(format!(
                    "Key '{}' not found and no default value provided",
                    key
                )),
            },
            Some(data) => Ok(Key::from_str(data.as_string().unwrap()).unwrap()),
        }
    }

    pub fn update_key<Key: FromStr>(&self, key: &str, value: &mut Key) -> Result<(), String>
    where
        <Key as FromStr>::Err: Debug,
    {
        if let Some(data) = self.json.find_path(&[key]) {
            if let Some(any) = data.as_string() {
                *value = Key::from_str(any).unwrap();
                Ok(())
            } else {
                Err(format!("Key '{}' not found", key))
            }
        } else {
            Err(format!("Key '{}' not found", key))
        }
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
            json: Json::from_str("{\"pop_width\": 3, \"pop_height\": 4, \"test_enum\":\"bar\", \"another_test_enum\":\"foo\", \"test_bool\":\"true\", \"another_test_bool\":\"false\" }").unwrap(),
        };

        let mut num = config.get_num("pop_width", None).unwrap();
        assert_eq!(num as u32, 3);

        assert!(config.update_num("pop_height", &mut num).is_ok());
        assert_eq!(num as u32, 4);

        let mut test_enum: TestEnum = config.get_key("test_enum", None).unwrap();
        assert_eq!(test_enum, TestEnum::Bar);

        assert!(config
            .update_key("another_test_enum", &mut test_enum)
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
