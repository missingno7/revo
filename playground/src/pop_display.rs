use crate::ind_display::IndDisplay;
use gtk::prelude::*;
use gtk::{gdk_pixbuf, Box, EventBox, Image, Label};
use std::cell::RefCell;
use std::fs;
use std::rc::Rc;

use revo::evo_individual::{EvoIndividual, Visualise};
use revo::population::Population;

#[derive(Clone)]
pub struct PopDisplay {
    img_path: String,
    images_width: i32,
    images_height: i32,
    original_image_width: i32,
    original_image_height: i32,
    image: Image,
    event_box: EventBox,
    label: Label,
    ind_display: IndDisplay,
}

impl PopDisplay {
    pub fn new(
        img_path: &str,
        images_width: i32,
        images_height: i32,
        ind_display: IndDisplay,
    ) -> Self {
        let image = Image::new();
        image.set_halign(gtk::Align::Start);
        image.set_valign(gtk::Align::Start);

        let event_box = gtk::EventBox::new();
        event_box.add(&image);

        let label = Label::new(None);

        PopDisplay {
            img_path: img_path.to_string(),
            images_width,
            images_height,
            original_image_width: 0,
            original_image_height: 0,
            image,
            event_box,
            label,
            ind_display,
        }
    }

    pub fn display_pop<Individual, IndividualData>(
        &mut self,
        pop: &Population<Individual, IndividualData>,
    ) where
        Individual: EvoIndividual<IndividualData> + Send + Sync + Clone,
        IndividualData: Sync,
    {
        pop.visualise(&self.img_path);
        let mut pixbuf = gdk_pixbuf::Pixbuf::from_file(&self.img_path).unwrap();
        self.original_image_width = pixbuf.width();
        self.original_image_height = pixbuf.height();
        pixbuf = pixbuf
            .scale_simple(
                self.images_width,
                self.images_height,
                gdk_pixbuf::InterpType::Nearest,
            )
            .unwrap();
        self.image.set_from_pixbuf(Some(&pixbuf));
        fs::remove_file(&self.img_path).unwrap();
    }

    pub fn get_widget<Individual, IndividualData>(
        self,
        pop: Rc<RefCell<Population<Individual, IndividualData>>>,
    ) -> Box
    where
        Individual: EvoIndividual<IndividualData>
            + Visualise<IndividualData>
            + Send
            + Sync
            + Clone
            + 'static,
        IndividualData: Sync + 'static,
    {
        let box_ = Box::new(gtk::Orientation::Vertical, 0);

        let label = self.label.clone();
        self.event_box
            .connect_button_press_event(move |_widget, event| {
                let (mut x, mut y) = event.position();

                x *= self.original_image_width as f64 / self.images_width as f64;
                y *= self.original_image_height as f64 / self.images_height as f64;

                if x as usize >= pop.borrow().get_width() || y as usize >= pop.borrow().get_height()
                {
                    println!("Out of bounds: ({}, {})", x, y);
                    return Inhibit(false);
                }

                let ind = pop.borrow().get_at(x as usize, y as usize);
                self.ind_display
                    .display_individual(&ind, pop.borrow().get_individual_data());

                label.set_text(&format!("x: {:.0},y: {:.0}", x, y));

                Inhibit(false)
            });

        box_.add(&self.event_box);
        box_.add(&self.label);
        box_
    }

    pub fn get_path(&self) -> String {
        self.img_path.clone()
    }
}
