use gtk::prelude::*;
use gtk::{Application, ApplicationWindow, Box, Button};
use std::cell::RefCell;
use std::rc::Rc;

use playground::ind_display::IndDisplay;
use playground::pop_display::PopDisplay;

// use playground::social_distance_gas::{prepare_gas, PlaygroundData, PlaygroundPopulation};
use playground::evo_salesman_gas::{prepare_gas, PlaygroundPopulation};

struct MainApp {
    pop: Rc<RefCell<PlaygroundPopulation>>,
    app: Application,
    ind_display: IndDisplay,
    pop_display: PopDisplay,
}

impl MainApp {
    pub fn new(app: &Application) -> MainApp {
        let pop_img_path: &str = "pop.png";
        let ind_img_path: &str = "ind.png";
        let images_width: i32 = 400;
        let images_height: i32 = 400;

        let gas = prepare_gas(pop_img_path, ind_img_path);

        let ind_display = IndDisplay::new(ind_img_path, images_width, images_height);
        ind_display.display_individual(&gas.pop.borrow().get_best(), &gas.ind_data.borrow());

        let pop_display = PopDisplay::new(
            pop_img_path,
            images_width,
            images_height,
            ind_display.clone(),
        );
        pop_display.display_pop(&gas.pop.borrow());

        MainApp {
            pop: gas.pop,
            app: app.clone(),
            ind_display,
            pop_display,
        }
    }

    pub fn add_functionality(self) {
        // Add buttons next to each other
        let buttons_box = Box::new(gtk::Orientation::Horizontal, 0);

        // +1 gen button
        let pop_clone = self.pop.clone();
        let pop_display_clone = self.pop_display.clone();
        let ind_display_clone = self.ind_display.clone();
        let button1 = Button::with_label("Next gen");
        buttons_box.add(&button1);
        button1.connect_clicked(move |_| {
            pop_clone.borrow_mut().next_gen();
            pop_display_clone.display_pop(&pop_clone.borrow());
            ind_display_clone.display_individual(
                &pop_clone.borrow().get_best(),
                pop_clone.borrow().get_individual_data(),
            );
        });

        // +10 gens button
        let pop_clone = self.pop.clone();
        let pop_display_clone = self.pop_display.clone();
        let ind_display_clone = self.ind_display.clone();
        let button2 = Button::with_label("+10 gens");
        buttons_box.add(&button2);
        button2.connect_clicked(move |_| {
            for _ in 0..10 {
                pop_clone.borrow_mut().next_gen();
            }

            pop_display_clone.display_pop(&pop_clone.borrow());
            ind_display_clone.display_individual(
                &pop_clone.borrow().get_best(),
                pop_clone.borrow().get_individual_data(),
            );
        });

        // Show best button
        let pop_clone = self.pop.clone();
        let ind_display_clone = self.ind_display.clone();
        let button3 = Button::with_label("Show best");
        buttons_box.add(&button3);
        button3.connect_clicked(move |_| {
            ind_display_clone.display_individual(
                &pop_clone.borrow().get_best(),
                pop_clone.borrow().get_individual_data(),
            );
        });

        // Layout - display images next to each other
        let displays_box = Box::new(gtk::Orientation::Horizontal, 0);
        displays_box.add(&self.pop_display.get_widget(self.pop.clone()));
        displays_box.add(&self.ind_display.get_widget());

        // Add everything to the main window - vertical layout
        let box_ = Box::new(gtk::Orientation::Vertical, 0);
        box_.add(&displays_box);
        box_.add(&buttons_box);

        // Add everything to the main window and show it
        let window = ApplicationWindow::new(&self.app);
        window.set_title("Evolutionary algorithm playground");
        window.set_resizable(false);
        window.add(&box_);
        window.show_all();
    }
}

fn main() {
    let app = Application::new(Some("com.example.image_box"), Default::default());

    app.connect_activate(move |app| {
        let app = MainApp::new(app);
        app.add_functionality();
    });

    app.run();
}
