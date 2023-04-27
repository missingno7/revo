use revo::population::Population;

use evo_salesman::salesman::SalesmanIndividual;
use evo_salesman::salesman_data::SalesmanIndividualData;
use funtree::funtree_data::FuntreeIndividualData;
use funtree::funtree_individual::FuntreeIndividual;
use playground::main_app::MainApp;
use revo::config::{Config, DEFAULT_CONFIG_FILENAME};
use revo::evo_population::EvoPopulation;
use social_distance::social_distance::{DistanceIndividual, DistanceIndividualData};
use strum::IntoEnumIterator;
use strum_macros::{Display, EnumIter, EnumString};

#[derive(Clone, EnumString, EnumIter, Display)]
pub enum ExampleType {
    #[strum(serialize = "salesman")]
    Salesman,
    #[strum(serialize = "social_distance")]
    SocialDistance,
    #[strum(serialize = "funtree")]
    Funtree,
}

fn main() {
    let config = Config::new(DEFAULT_CONFIG_FILENAME);

    // Get the example type from the config file
    let example_type: ExampleType = config.get_val("example").unwrap().unwrap_or_else(|| {
        let possible_example_types = ExampleType::iter()
            .map(|v| v.to_string())
            .collect::<Vec<_>>();

        panic!(
            "No example type specified in {}, possible values for \"example\" are: {:?}",
            DEFAULT_CONFIG_FILENAME, possible_example_types
        )
    });

    let main_app = match example_type {
        ExampleType::Funtree => {
            let pop = Population::<FuntreeIndividual, FuntreeIndividualData>::new(&config);
            MainApp::new(pop, &config)
        }
        ExampleType::SocialDistance => {
            let pop = Population::<DistanceIndividual, DistanceIndividualData>::new(&config);
            MainApp::new(pop, &config)
        }
        ExampleType::Salesman => {
            let pop = Population::<SalesmanIndividual, SalesmanIndividualData>::new(&config);
            MainApp::new(pop, &config)
        }
    };

    main_app.run();
}
