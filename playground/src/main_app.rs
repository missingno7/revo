use crate::main_window::MainWindow;
use gtk::prelude::*;
use gtk::{Application, ApplicationWindow};
use revo::config::Config;
use revo::evo_individual::{EvoIndividual, EvoIndividualData, Visualise};
use revo::population::Population;
use std::cell::RefCell;
use std::rc::Rc;

pub struct MainApp {
    app: Application,
}

impl MainApp {
    pub fn new<Individual, IndividualData>(
        pop: Population<Individual, IndividualData>,
        config: &Config,
    ) -> MainApp
    where
        Individual: EvoIndividual<IndividualData> + Visualise<IndividualData> + 'static,
        IndividualData: EvoIndividualData + 'static,
    {
        let rc_pop = Rc::new(RefCell::new(pop));
        let app = Application::new(Some("com.example.image_box"), Default::default());

        let config: Config = config.clone();
        app.connect_activate(move |app| {
            let main_window = Rc::new(RefCell::new(MainWindow::<Individual, IndividualData>::new(
                rc_pop.clone(),
                &config,
            )));

            // Add everything to the main window and show it
            let window = ApplicationWindow::new(app);
            window.set_title("Evolutionary algorithm playground");
            window.set_resizable(false);
            window.add(&MainWindow::get_widget(&main_window));
            window.show_all();
        });

        MainApp { app }
    }

    pub fn run(&self) {
        self.app.run();
    }
}
