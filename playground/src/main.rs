use evo_salesman::salesman::SalesmanIndividual;
use evo_salesman::salesman_data::SalesmanIndividualData;

use revo::population::Population;

use playground::main_app::MainApp;
use revo::config::Config;
use social_distance::social_distance::{DistanceIndividual, DistanceIndividualData};
use funtree::funtree_data::FuntreeIndividualData;
use funtree::funtree_individual::FuntreeIndividual;

pub fn prepare_population_salesman(
    config: &Config,
) -> Population<SalesmanIndividual, SalesmanIndividualData> {
    let mut rng = rand::thread_rng();

    // Individual data
    let ind_data: SalesmanIndividualData = SalesmanIndividualData::from_config(&mut rng, config);

    Population::new(config, ind_data)
}

pub fn prepare_population_social_distance(
    config: &Config,
) -> Population<DistanceIndividual, DistanceIndividualData> {
    // Individual data
    let ind_data: DistanceIndividualData = DistanceIndividualData::from_config(config);

    Population::new(config, ind_data)
}


pub fn prepare_population_funtree(
    config: &Config,
) -> Population<FuntreeIndividual, FuntreeIndividualData> {
    // Individual data
    let ind_data: FuntreeIndividualData = FuntreeIndividualData::from_config(config);

    Population::new(config, ind_data)
}

fn main() {
    let config = Config::new("config.json");

    //let pop = prepare_population_salesman(&config);
    let pop = prepare_population_funtree(&config);

    let main_app = MainApp::new(pop, &config);
    main_app.run();
}
