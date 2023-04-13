use gtk::prelude::*;
use gtk::{gdk_pixbuf, Box, Image, Label};
use revo::evo_individual::{EvoIndividual, Visualise};
use std::fs;

pub struct IndDisplay {
    img_path: String,
    images_width: i32,
    images_height: i32,
    image: Image,
    top_label: Label,
    bottom_label: Label,
}

impl IndDisplay {
    pub fn new(img_path: &str, images_width: i32, images_height: i32) -> Self {
        let image = Image::new();
        image.set_halign(gtk::Align::Start);
        image.set_valign(gtk::Align::Start);
        image.set_size_request(images_width, images_height);
        let top_label = Label::new(None);
        let bottom_label = Label::new(None);
        IndDisplay {
            img_path: img_path.to_string(),
            images_width,
            images_height,
            image,
            top_label,
            bottom_label,
        }
    }

    pub fn display_individual<Individual, IndividualData>(
        &self,
        ind: &Individual,
        ind_data: &IndividualData,
    ) where
        Individual: EvoIndividual<IndividualData> + Visualise<IndividualData> + Send + Sync + Clone,
        IndividualData: Sync,
    {
        ind.visualise(&self.img_path, ind_data);

        let mut pixbuf = gdk_pixbuf::Pixbuf::from_file(&self.img_path).unwrap();
        pixbuf = pixbuf
            .scale_simple(
                self.images_width,
                self.images_height,
                gdk_pixbuf::InterpType::Bilinear,
            )
            .unwrap();
        self.image.set_from_pixbuf(Some(&pixbuf));
        fs::remove_file(&self.img_path).unwrap();

        let ind_visuals = ind.get_visuals(ind_data);
        self.top_label
            .set_text(&format!("a: {:.4}, b: {:.4}", ind_visuals.0, ind_visuals.1));
        self.bottom_label
            .set_text(&format!("fitness: {:.4}", ind.get_fitness()));
    }

    pub fn get_widget(&self) -> Box {
        let box_ = Box::new(gtk::Orientation::Vertical, 0);
        box_.add(&self.image);
        box_.add(&self.top_label);
        box_.add(&self.bottom_label);
        box_
    }
}
