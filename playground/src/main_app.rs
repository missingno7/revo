use crate::main_window::MainWindow;
use gtk::prelude::*;
use gtk::{Application, ApplicationWindow};
use revo::evo_individual::{EvoIndividual, Visualise};
use revo::population::Population;
use std::cell::RefCell;
use std::rc::Rc;
pub struct MainApp {
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
            window.add(&MainWindow::get_widget(&main_window));
            window.show_all();
        });

        MainApp { app }
    }

    pub fn run(&self) {
        self.app.run();
    }
}
