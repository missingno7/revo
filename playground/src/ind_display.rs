use gtk::prelude::*;
use gtk::{gdk_pixbuf, Image};
use std::fs;

use crate::evo_salesman_gas::{PlaygroundData, PlaygroundIndividual};

#[derive(Clone)]
pub struct IndDisplay {
    img_path: String,
    images_width: i32,
    images_height: i32,
    image: Image,
}

impl IndDisplay {
    pub fn new(img_path: &str, images_width: i32, images_height: i32) -> Self {
        let image = Image::new();
        image.set_halign(gtk::Align::Start);
        image.set_valign(gtk::Align::Start);
        image.set_size_request(images_width, images_height);

        IndDisplay {
            img_path: img_path.to_string(),
            images_width,
            images_height,
            image,
        }
    }

    pub fn display_individual(&self, ind: &PlaygroundIndividual, ind_data: &PlaygroundData) {
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
    }

    pub fn get_widget(&self) -> Image {
        self.image.clone()
    }
}
