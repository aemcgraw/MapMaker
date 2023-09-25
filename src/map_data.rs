use image::{RgbImage, Rgb};

use crate::util::util::Util;

pub struct MapData {
    pub data : Vec<f64>,
    pub width : u32,    //TODO : Maybe shouldn't be public
    pub height : u32,
}

impl MapData{
    pub fn data(&self) -> &Vec<f64> {
       return &self.data;
    }

    pub fn new(a: Vec<f64>, x: u32, y: u32) -> MapData {
        return MapData { data : a, width : x, height : y }
    }

    pub fn mapdata_from_points(a: Vec<[u32; 2]>, x: u32, y: u32 ) -> MapData {
        let mut backing_vec = vec![0.0; (x * y) as usize];
        
        let data_iter = a.iter();
        for val in data_iter {
            if val[0] * val[1] < x * y {
                backing_vec[(x * val[1] + val[0]) as usize] = 1.0;
            } else {
                panic!("Point is outside the range of {}", x * y);
            }
        }

        return MapData { data : backing_vec, width : x, height : y }
    }

    pub fn get_point(&self, x : u32) -> f64 {
        let index = x as usize;
        return self.data[index];
    }

    pub fn in_bounds(&self, x : u32, y : u32) -> bool {
        if x > self.width - 1 {
            return false;
        } 
        if y > self.width - 1 {
            return false;
        }
        return true;
    }

    pub fn get_pixel(&self, x : u32, y : u32) -> f64 {
        let index = (y * self.width + x) as usize;
        return self.data[index];
    }

    pub fn get_pixel_modular(&self, x: u32, y: u32) -> f64 {
        let new_x = Util::modu(x, self.width);
        let new_y = Util::modu(y, self.height);

        let index = (new_y * self.width + new_x) as usize;
        return self.data[index];
    }

    pub fn put_pixel(&mut self, x : u32, y : u32, a : f64) {
        let index = (y * self.width + x) as usize;
        //println!("{}", index);
        //println!("{}", a);
        self.data[index] = a;
    }

    pub fn put_pixel_modular(&mut self, x: u32, y: u32, a: f64) {
        let new_x = Util::modu(x, self.width);
        let new_y = Util::modu(y, self.height);

        let index = (new_y * self.width + new_x) as usize;
        self.data[index] = a;
    }

    /*
    pub fn to_image(&self, coloring: &str) -> RgbImage{
        match coloring {
            "rainbow" | "r" => self.data_to_rainbow(),
            "bluegreen" | "bg" => self.data_to_blue_green(),
            _ => { 
                println!("Provided coloring scheme {} not recognized, defaulting to bluegreen", algo);
                self.data_to_bluegreen() 
            }
        }
    }
    */
    
    //pub fn to_imagedata(&self, coloring: &str) -> Vec<u8>{
    //    self.data_to_blue_green_vec()
    //}

    pub fn data_to_red(&self) -> RgbImage {
        let mut output = RgbImage::new(self.width, self.height);

        let mut counterx = 0;
        let mut countery = 0;

        for x in &self.data {
            let rgbvalue = (x * 255.0) as u8;
            output.put_pixel(counterx, countery, Rgb([rgbvalue, 0, 0]));

            if counterx < self.width - 1 {
                counterx = counterx + 1;
            } else {
                countery = countery + 1;
                counterx = 0;
            }
        }

        return output;
    }
}
