pub mod diamond_square;
pub mod diamond_square_borderless;

use crate::utilities::file_util::file_util;
use crate::map_data::MapData;
use image::{RgbaImage};

pub trait Run {
    fn run(&mut self, chaos: f64, damping: f64, blocksize: u32);
}

pub trait Size {
    fn size(width: u32, height: u32) -> u32;
}

pub trait Save {
    fn save(&self, image: RgbaImage, name: &String) {
        file_util::save(image, name);
    }
}

//pub trait ToImage {
//    fn to_image(&self, coloring: &str) -> RgbImage; 
//}

pub trait GetData {
    fn get_data(&self) -> &MapData;
}

pub trait GetDim {
    fn get_dim(&self) -> u32;
}

pub trait ImageAlg: Run + Save + GetData + GetDim {
    fn imagealg(&self) -> &str {
        return "Hello"
    }
}
