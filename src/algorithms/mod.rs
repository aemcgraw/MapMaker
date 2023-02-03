pub mod diamond_square;
pub mod diamond_square_borderless;

use crate::config;
use crate::map_data::MapData;
use image::RgbImage;
use std::path::Path;

pub trait Run {
    fn run(&mut self, chaos: f64, damping: f64, blocksize: u32);
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

//pub trait ToImage {
//    fn to_image(&self, coloring: &str) -> RgbImage; 
//}

pub trait GetData {
    fn get_data(&self) -> &MapData;
}

pub trait ImageAlg: Run + Save + GetData {
    fn imagealg(&self) -> &str {
        return "Hello"
    }
}
