use crate::val::Val;
use crate::val::ValVec;
use revo::config::Config;

const DEFAULT_MAX_DEPTH: u32 = 5;
const DEFAULT_PLOT_WIDTH: u32 = 400;
const DEFAULT_PLOT_HEIGHT: u32 = 400;


#[derive(Clone)]
pub struct FuntreeIndividualData {
    pub vals: Vec<Val>,
    pub max_depth: u32,

    pub plot_width: u32,
    pub plot_height: u32,
}

impl FuntreeIndividualData {
    pub fn from_config(config: &Config) -> Self {
        FuntreeIndividualData {
            vals: config.get_val::<ValVec>("values").unwrap().unwrap().into(),
            max_depth: config
                .get_int("max_depth")
                .unwrap()
                .unwrap_or(DEFAULT_MAX_DEPTH),
            plot_width: config.get_int("plot_width").unwrap().unwrap_or(DEFAULT_PLOT_WIDTH),
            plot_height: config.get_int("plot_height").unwrap().unwrap_or(DEFAULT_PLOT_HEIGHT),
        }
    }
}

impl Default for FuntreeIndividualData {
    fn default() -> Self {
        FuntreeIndividualData {
            vals: Vec::new(),
            max_depth: DEFAULT_MAX_DEPTH,
            plot_width: DEFAULT_PLOT_WIDTH,
            plot_height: DEFAULT_PLOT_HEIGHT,
        }
    }
}
