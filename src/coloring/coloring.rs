use image::{RgbImage, Rgb};

use crate::map_data::MapData;
use crate::util::util;

pub struct Coloring<'a> {
    pub data: &'a MapData,
}

impl Coloring<'_> {
    pub fn new(data: &MapData) -> Coloring {
        return Coloring { data };
    }

    pub fn data_to_blue_green(&self, waterperc: f64) -> RgbImage {
        let map_vec = &self.data;

        let mut output = RgbImage::new(map_vec.width, map_vec.height);

        let mut counterx = 0;
        let mut countery = 0;

        let mut tempdata = map_vec.data.clone();
        let initvalue = (tempdata.len() as f64) * waterperc;
        let water = util::quickselect(&mut tempdata, initvalue as u32);

        for x in &map_vec.data {
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

            if counterx < map_vec.width - 1 {
                counterx = counterx + 1;
            } else {
                countery = countery + 1;
                counterx = 0;
            }
        }

        return output;
    }

    pub fn data_to_blue_green_vec(&self, waterperc: f64) -> Vec<u8> {
        let map_vec = &self.data;

        let mut data = Vec::new();

        let mut counterx = 0;
        let mut countery = 0;

        let mut tempdata = map_vec.data.clone();

        let initvalue = (tempdata.len() as f64) * waterperc;
        let water = util::quickselect(&mut tempdata, initvalue as u32);

        for x in &map_vec.data {
            let mut rgbvalue = (x * 125.0) as u8;
            if rgbvalue > 125 {
                rgbvalue = 250;
            } else {
                rgbvalue = rgbvalue + 125;
            }

            if x < &water {
                data.append(&mut vec![0, 0, rgbvalue, 255]);
            } else {
                data.append(&mut vec![0, rgbvalue, 0, 255]);
            }

            if counterx < map_vec.width - 1 {
                counterx = counterx + 1;
            } else {
                countery = countery + 1;
                counterx = 0;
            }
        }

        return data;
    }

    pub fn data_to_rainbow(&self) -> RgbImage{
        let map_vec = &self.data;

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

        let mut output = RgbImage::new(map_vec.width, map_vec.height);

        let mut counterx = 0;
        let mut countery = 0;

        for x in &map_vec.data {
            let greenv: u8 = normalize(x, 0.0);
            let bluev: u8 = normalize(x, 0.33);
            let redv: u8 = normalize_edge(x);

            output.put_pixel(counterx, countery, Rgb([redv, greenv, bluev]));

            if counterx < map_vec.width - 1 {
                counterx = counterx + 1;
            } else {
                countery = countery + 1;
                counterx = 0;
            }
        }

        return output;
    }

    pub fn data_to_rainbow_vec(&self) -> Vec<u8> {
        let map_vec = &self.data;

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

        let mut data = Vec::new();

        let mut counterx = 0;
        let mut countery = 0;

        for x in &map_vec.data {
            let greenv: u8 = normalize(x, 0.0);
            let bluev: u8 = normalize(x, 0.33);
            let redv: u8 = normalize_edge(x);

            data.append(&mut vec![redv, greenv, bluev, 255]);

            if counterx < map_vec.width - 1 {
                counterx = counterx + 1;
            } else {
                countery = countery + 1;
                counterx = 0;
            }
        }

        return data;
    }

    pub fn data_to_topographical_vec(&self, water: Option<f64>) -> Vec<u8> {
        let map_vec = &self.data;

        let mut counterx = 0;
        let mut countery = 0;

        let waterperc = match water {
            Some(w) => w,
            None => 0.0
        };

        let mut data = Vec::new();
        let mut tempdata = map_vec.data.clone();
        let initvalue = (tempdata.len() as f64) * waterperc;
        let waterheight = util::quickselect(&mut tempdata, initvalue as u32);

        for x in &map_vec.data {
            let fract = (x * 10.0).fract();

            if x < &waterheight {
                data.append(&mut vec![0, 0, 255, 255]);
            } else if (fract < 0.1) | (fract > 0.9) {
                data.append(&mut vec![0, 0, 0, 255]);
            } else {
                data.append(&mut vec![255, 255, 255, 255]);
            }

            if counterx < map_vec.width - 1 {
                counterx = counterx + 1
            } else {
                countery = countery + 1;
                counterx = 0;
            }
        }

        return data;
    }
}
