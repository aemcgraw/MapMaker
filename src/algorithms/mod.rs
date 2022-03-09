pub mod diamond_square;
pub mod diamond_square_borderless;

use crate::config;
use image::RgbImage;
use std::path::Path;

pub trait Run {
    fn run(&mut self, chaos: f64);
}

pub trait Size {
    fn size(width: u32, height: u32) -> u32;
}

pub trait Save {
    fn save(&self, image: RgbImage, name: &String) {
        let save_path = Path::new(config::OUTPUT_DIR).join(Path::new(name));
        image.save(save_path).expect("Couldn't save image");
    }
}

pub trait ToImage {
    fn to_image(&self, algo: &str) -> RgbImage; 
}

pub trait ImageAlg: Run + Save + ToImage {
    fn imagealg(&self) -> &str {
        return "Hello"
    }
}
