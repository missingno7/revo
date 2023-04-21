use gtk::prelude::*;
use gtk::{gdk_pixbuf, Box, Image, Label};
use image::{ImageOutputFormat, RgbImage};
use revo::evo_individual::{EvoIndividual, Visualise};
use std::io::Cursor;

pub struct IndDisplay {
    images_width: u32,
    images_height: u32,
    image: Image,
    top_label: Label,
    bottom_label: Label,
}

impl IndDisplay {
    pub fn new(images_width: u32, images_height: u32) -> Self {
        let image = Image::new();
        image.set_halign(gtk::Align::Start);
        image.set_valign(gtk::Align::Start);
        image.set_size_request(images_width as i32, images_height as i32);
        let top_label = Label::new(None);
        let bottom_label = Label::new(None);
        IndDisplay {
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
        let img: RgbImage = ind.visualise(ind_data);

        // Save the image to a vector in memory using a Cursor
        let mut buffer = Vec::new();
        let mut cursor = Cursor::new(&mut buffer);
        img.write_to(&mut cursor, ImageOutputFormat::Png).unwrap();

        // Create a PixbufLoader to load the image from the buffer
        let loader = gdk_pixbuf::PixbufLoader::new();
        loader.write(&buffer).unwrap();
        loader.close().unwrap();

        // Load the Pixbuf from the loader and scale it
        let mut pixbuf = loader.pixbuf().unwrap();
        pixbuf = pixbuf
            .scale_simple(
                self.images_width as i32,
                self.images_height as i32,
                gdk_pixbuf::InterpType::Bilinear,
            )
            .unwrap();

        // Set the scaled pixbuf to the image widget
        self.image.set_from_pixbuf(Some(&pixbuf));

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
