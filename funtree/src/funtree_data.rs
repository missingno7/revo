use crate::val::Val;
use crate::val::ValVec;
use revo::config::Config;

const DEFAULT_MAX_DEPTH: u32 = 5;

#[derive(Clone)]
pub struct FuntreeIndividualData {
    pub vals: Vec<Val>,
    pub max_depth: u32,
}

impl FuntreeIndividualData {
    pub fn from_config(config: &Config) -> Self {
        FuntreeIndividualData {
            vals: config.get_val::<ValVec>("values", None).unwrap().into(),
            max_depth: config
                .get_num("max_depth", Some(DEFAULT_MAX_DEPTH.into()))
                .unwrap() as u32,
        }
    }
}
