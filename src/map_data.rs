use image::{RgbImage, Rgb};

use crate::util::util::util;

pub struct MapData {
    pub data : Vec<f64>,
    pub width : u32,    //TODO : Maybe shouldn't be public
    pub height : u32,
}

impl MapData{
    pub fn data(&self) -> &Vec<f64> {
       return &self.data;
    }

    pub fn new(a : Vec<f64>, x : u32, y : u32) -> MapData {
        return MapData { data : a, width : x, height : y }
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
        let new_x = util::modu(x, self.width);
        let new_y = util::modu(y, self.height);

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
        let new_x = util::modu(x, self.width);
        let new_y = util::modu(y, self.height);

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
            //println!("{}", rgbvalue);
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

/*
    pub fn data_to_blue_green(&self) -> RgbImage {
        let mut output = RgbImage::new(self.width, self.height);

        let mut counterx = 0;
        let mut countery = 0;

        let mut tempdata = self.data.clone();
        let initvalue = (tempdata.len() * 0.7) as u32;
        let water = util::quickselect(&mut tempdata, initvalue as u32);

        for x in &self.data {
            let mut rgbvalue = (x * 125.0) as u8;
            if rgbvalue > 125 {
                rgbvalue = 250;
            } else {
                rgbvalue = rgbvalue + 125;
            }

            if x < &water {
                output.put_pixel(counterx, countery, Rgb([0, 0, rgbvalue]));
            } else {
                output.put_pixel(counterx, countery, Rgb([0, rgbvalue, 0]));
            }

            if counterx < self.width - 1 {
                counterx = counterx + 1;
            } else {
                countery = countery + 1;
                counterx = 0;
            } 
        }

        return output;
    }

    pub fn data_to_blue_green_vec(&self) -> Vec<u8> {
        let mut data = Vec::new(); 

        let mut counterx = 0;
        let mut countery = 0;

        let mut tempdata = self.data.clone();

        let initvalue = (tempdata.len() * 0.7) as u32;
        let water = util::quickselect(&mut tempdata, initvalue as u32);

        for x in &self.data {
            let mut rgbvalue = (x * 125.0) as u8;
            if rgbvalue > 125 {
                rgbvalue = 250;
            } else {
                rgbvalue = rgbvalue + 125;
            }

            if x < &water {
                data.push(0);
                data.push(0);
                data.push(rgbvalue);
                data.push(255);
            } else {
                data.push(0);
                data.push(rgbvalue);
                data.push(0);
                data.push(255);
            }

            if counterx < self.width - 1 {
                counterx = counterx + 1;
            } else {
                countery = countery + 1;
                counterx = 0;
            } 
        }

        return data; 
    }

    pub fn data_to_rainbow(&self) -> RgbImage{
        fn normalize(value: &f64, low: f64) -> u8 {
            if value < &low {
                return 0;
            } else if value < &(low + 0.16) {
                return ((value - low) * 6.0 * 255.0) as u8;
            } else if value < &(low + 0.5) {
                return 255;
            } else if value < &(low + 0.66) {
                return ((1.0 - ((value - 0.5) * 6.0)) * 255.0) as u8;
            } else {
                return 0;
            }
        }

        fn normalize_edge(value: &f64) -> u8 {
            if (value < &0.16) | (value > &0.84) {
                return 255;
            } else if value < &0.33 {
                return ((1.0 - ((value - 0.16) * 6.0)) * 255.0) as u8;
            } else if value > &0.66 {
                return ((value - 0.66) * 6.0 * 255.0) as u8
            } else {
                return 0;
            }
        }

        let mut output = RgbImage::new(self.width, self.height);

        let mut counterx = 0;
        let mut countery = 0;

        for x in &self.data {
            let greenv: u8 = normalize(x, 0.0);
            let bluev: u8 = normalize(x, 0.33);
            //let mut bluev: u8 = 0;
            let redv: u8 = normalize_edge(x);
            //let mut redv: u8 = 0;

            output.put_pixel(counterx, countery, Rgb([redv, greenv, bluev]));

            if counterx < self.width - 1 {
                counterx = counterx + 1;
            } else {
                countery = countery + 1;
                counterx = 0;
            }  
        }

        return output;
    }
*/
}


