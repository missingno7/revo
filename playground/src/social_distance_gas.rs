use revo::pop_config::PopulationConfig;
use revo::population::Population;
use social_distance::social_distance::{DistanceIndividual, DistanceIndividualData};
use std::cell::RefCell;
use std::rc::Rc;

pub type PlaygroundPopulation = Population<DistanceIndividual, DistanceIndividualData>;
pub type PlaygroundData = DistanceIndividualData;

pub struct GasData {
    pub ind_data: Rc<RefCell<PlaygroundData>>,
    pub pop: Rc<RefCell<PlaygroundPopulation>>,
}

pub fn prepare_gas(pop_img_path: &str, ind_img_path: &str) -> GasData {
    // Individual data
    let n_points: usize = 50;
    let required_distance = 20;
    let screen_width: u32 = 400;
    let screen_height: u32 = 400;

    let pop_config = PopulationConfig::new("pop_config.json");

    let ind_data: Rc<RefCell<PlaygroundData>> = Rc::new(RefCell::new(DistanceIndividualData {
        n_points,
        screen_width,
        screen_height,
        required_distance,
    }));

    let pop: Rc<RefCell<PlaygroundPopulation>> = Rc::new(RefCell::new(Population::new(
        &pop_config,
        ind_data.borrow().clone(),
    )));

    // Visualise initial population and best individual for the first time
    pop.borrow()
        .get_best()
        .visualise(ind_img_path, &ind_data.borrow());
    pop.borrow().visualise(pop_img_path);

    GasData { ind_data, pop }
}
