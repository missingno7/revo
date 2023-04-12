use evo_salesman::salesman::SalesmanIndividual;
use evo_salesman::salesman_data::SalesmanIndividualData;

use revo::pop_config::PopulationConfig;
use revo::population::Population;
use std::cell::RefCell;
use std::rc::Rc;

pub type PlaygroundData = SalesmanIndividualData;
pub type PlaygroundIndividual = SalesmanIndividual;
pub type PlaygroundPopulation = Population<PlaygroundIndividual, PlaygroundData>;

pub struct GasData {
    pub ind_data: Rc<RefCell<PlaygroundData>>,
    pub pop: Rc<RefCell<PlaygroundPopulation>>,
}

pub fn prepare_gas(pop_img_path: &str, ind_img_path: &str) -> GasData {
    let mut rng = rand::thread_rng();

    // Individual data
    let pop_config = PopulationConfig::new("pop_config.json");

    let ind_data: Rc<RefCell<PlaygroundData>> = Rc::new(RefCell::new(
        SalesmanIndividualData::from_config(&mut rng, &pop_config),
    ));

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
