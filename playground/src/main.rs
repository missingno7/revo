use revo::population::Population;

use evo_salesman::salesman::SalesmanIndividual;
use evo_salesman::salesman_data::SalesmanIndividualData;
use evo_true_packer::packer::PackerIndividual;
use evo_true_packer::packer_data::PackerIndividualData;
use funtree::funtree_data::FuntreeIndividualData;
use funtree::funtree_individual::FuntreeIndividual;
use playground::main_app::MainApp;
use revo::config::{Config, DEFAULT_CONFIG_FILENAME};
use social_distance::social_distance::{DistanceIndividual, DistanceIndividualData};
use strum_macros::{Display, EnumIter, EnumString};

#[derive(Clone, EnumString, EnumIter, Display)]
pub enum ExampleType {
    #[strum(serialize = "salesman")]
    Salesman,
    #[strum(serialize = "social_distance")]
    SocialDistance,
    #[strum(serialize = "funtree")]
    Funtree,
    #[strum(serialize = "packer")]
    Packer,
}

fn main() {
    let config = Config::new(DEFAULT_CONFIG_FILENAME);

    // Get the example type from the config file
    let example_type: ExampleType = config.get_enum("example").unwrap();

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
        ExampleType::Packer => {
            let pop = Population::<PackerIndividual, PackerIndividualData>::new(&config);
            MainApp::new(pop, &config)
        }
    };

    main_app.run();
}
