use crate::val::Val;
use crate::val::ValVec;
use revo::config::Config;

#[derive(Clone)]
pub struct FuntreeIndividualData {
    pub vals: Vec<Val>,
}

impl FuntreeIndividualData {
    pub fn from_config(config: &Config) -> Self {
        FuntreeIndividualData {
            vals: config.get_val::<ValVec>("values", None).unwrap().into(),
        }
    }
}
