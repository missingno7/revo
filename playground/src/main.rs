use evo_salesman::salesman::SalesmanIndividual;
use evo_salesman::salesman_data::SalesmanIndividualData;
use gtk::prelude::*;
use gtk::{Application, ApplicationWindow};
use revo::evo_individual::{EvoIndividual, Visualise};
use revo::population::Population;
use std::cell::RefCell;
use std::rc::Rc;

use playground::main_window::MainWindow;
use revo::pop_config::PopulationConfig;

struct MainApp {
    app: Application,
}

impl MainApp {
    pub fn new<Individual, IndividualData>(pop: Population<Individual, IndividualData>) -> MainApp
    where
        Individual: EvoIndividual<IndividualData>
            + Visualise<IndividualData>
            + Send
            + Sync
            + Clone
            + 'static,
        IndividualData: Sync + 'static,
    {
        let rc_pop = Rc::new(RefCell::new(pop));
        let app = Application::new(Some("com.example.image_box"), Default::default());

        app.connect_activate(move |app| {
            let main_window = Rc::new(RefCell::new(MainWindow::<Individual, IndividualData>::new(
                rc_pop.clone(),
            )));

            // Add everything to the main window and show it
            let window = ApplicationWindow::new(app);
            window.set_title("Evolutionary algorithm playground");
            window.set_resizable(false);
            window.add(&MainWindow::get_widget(main_window));
            window.show_all();
        });

        MainApp { app }
    }

    pub fn run(&self) {
        self.app.run();
    }
}

pub fn prepare_population_salesman() -> Population<SalesmanIndividual, SalesmanIndividualData> {
    let mut rng = rand::thread_rng();

    // Individual data
    let pop_config = PopulationConfig::new("pop_config.json");

    let ind_data: SalesmanIndividualData =
        SalesmanIndividualData::from_config(&mut rng, &pop_config);

    Population::new(&pop_config, ind_data)
}

fn main() {
    let pop = prepare_population_salesman();

    let main_app = MainApp::new(pop);
    main_app.run();
}
