use evo_salesman::salesman::SalesmanIndividual;
use evo_salesman::salesman_data::SalesmanIndividualData;

use revo::population::Population;

use playground::main_app::MainApp;
use revo::pop_config::PopulationConfig;
use social_distance::social_distance::{DistanceIndividual, DistanceIndividualData};

pub fn prepare_population_salesman() -> Population<SalesmanIndividual, SalesmanIndividualData> {
    let mut rng = rand::thread_rng();

    // Individual data
    let pop_config = PopulationConfig::new("pop_config.json");

    let ind_data: SalesmanIndividualData =
        SalesmanIndividualData::from_config(&mut rng, &pop_config);

    Population::new(&pop_config, ind_data)
}

pub fn prepare_population_social_distance() -> Population<DistanceIndividual, DistanceIndividualData>
{
    // Individual data
    let pop_config = PopulationConfig::new("pop_config.json");

    let ind_data: DistanceIndividualData = DistanceIndividualData::from_config(&pop_config);

    Population::new(&pop_config, ind_data)
}

fn main() {
    let pop = prepare_population_salesman();

    let main_app = MainApp::new(pop);
    main_app.run();
}
