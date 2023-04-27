use crate::ind_display::IndDisplay;
use crate::pop_display::PopDisplay;
use gtk::prelude::*;
use gtk::Box;
use gtk::Button;
use revo::config::Config;
use revo::evo_individual::Visualise;
use revo::evo_individual::{EvoIndividual, EvoIndividualData};
use revo::population::Population;
use std::cell::RefCell;
use std::rc::Rc;

const DEFAULT_DISPLAY_WIDTH: u32 = 400;
const DEFAULT_DISPLAY_HEIGHT: u32 = 400;

pub struct MainWindow<Individual, IndividualData> {
    pop: Rc<RefCell<Population<Individual, IndividualData>>>,
    ind_display: Rc<RefCell<IndDisplay>>,
    pop_display: Rc<RefCell<PopDisplay>>,
}

impl<
        Individual: EvoIndividual<IndividualData> + Visualise<IndividualData> + 'static,
        IndividualData: EvoIndividualData + 'static,
    > MainWindow<Individual, IndividualData>
{
    pub fn new(
        pop: Rc<RefCell<Population<Individual, IndividualData>>>,
        config: &Config,
    ) -> MainWindow<Individual, IndividualData> {
        let display_width: u32 = config
            .may_get_uint("display_width")
            .unwrap()
            .unwrap_or(DEFAULT_DISPLAY_WIDTH);
        let display_height: u32 = config
            .may_get_uint("display_height")
            .unwrap()
            .unwrap_or(DEFAULT_DISPLAY_HEIGHT);

        let ind_display = Rc::new(RefCell::new(IndDisplay::new(display_width, display_height)));
        ind_display
            .borrow_mut()
            .display_individual(pop.borrow().get_best(), pop.borrow().get_individual_data());

        let pop_display = Rc::new(RefCell::new(PopDisplay::new(
            display_width,
            display_height,
            ind_display.clone(),
        )));
        pop_display.borrow_mut().display_pop(&pop.borrow_mut());

        MainWindow {
            pop,
            ind_display,
            pop_display,
        }
    }

    pub fn get_widget(self_pointer: &Rc<RefCell<Self>>) -> Box {
        let self_ = self_pointer.borrow_mut();

        // Add buttons next to each other
        let buttons_box = Box::new(gtk::Orientation::Horizontal, 0);

        // +1 gens button
        buttons_box.add(&Self::_get_plus_n_button(self_pointer, 1));

        // +10 gens button
        buttons_box.add(&Self::_get_plus_n_button(self_pointer, 10));

        // +100 gens button
        buttons_box.add(&Self::_get_plus_n_button(self_pointer, 100));

        // Show best button
        buttons_box.add(&Self::_get_show_best_button(self_pointer));

        // Add displays next to each other
        let displays_box = Box::new(gtk::Orientation::Horizontal, 0);
        displays_box.add(&PopDisplay::get_widget(
            &self_.pop_display,
            self_.pop.clone(),
        ));
        displays_box.add(&self_.ind_display.borrow().get_widget());

        // Add displays and buttons to the main window - vertical layout
        let box_ = Box::new(gtk::Orientation::Vertical, 0);
        box_.add(&displays_box);
        box_.add(&buttons_box);

        box_
    }

    fn _get_show_best_button(self_pointer: &Rc<RefCell<Self>>) -> Button {
        let self_pointer_clone = self_pointer.clone();
        let button = Button::with_label("Show best");
        button.connect_clicked(move |_| {
            let self_ = self_pointer_clone.borrow_mut();
            self_.ind_display.borrow().display_individual(
                self_.pop.borrow().get_best(),
                self_.pop.borrow().get_individual_data(),
            );
        });
        button
    }

    fn _get_plus_n_button(self_pointer: &Rc<RefCell<Self>>, n: usize) -> Button {
        // +N gens button
        let self_pointer_clone = self_pointer.clone();
        let button = Button::with_label(format!("+{} gen", n).as_str());
        button.connect_clicked(move |_| {
            let self_ = self_pointer_clone.borrow_mut();
            for _ in 0..n {
                self_.pop.borrow_mut().next_gen();
            }

            self_
                .pop_display
                .borrow_mut()
                .display_pop(&self_.pop.borrow());
            self_.ind_display.borrow().display_individual(
                self_.pop.borrow().get_best(),
                self_.pop.borrow().get_individual_data(),
            );
        });
        button
    }
}
