use image::{RgbaImage, Rgba};

use crate::map_data::MapData;
use crate::utilities::util::util;

pub struct Coloring<'a> {
    pub data: &'a MapData,
    pub colormod: ColorModel,
    pub image: Option<RgbaImage>,
    pub imagevec: Option<Vec<u8>>,
    pub waterlevel: f64,
}

pub enum ColorModel {
    Binary,
    BlueGreen,
    Rainbow,
    Topographical,
}

// TODO : Does this need to be public?
enum OutputFormat {
    Image(RgbaImage),
    ImageVec(Vec<u8>),
}

//impl OutputFormat {
//    fn output(&self) {
//        match self {
//
//        }
//    }
//}

impl Coloring<'_> {
    pub fn new<'a>(data: &'a MapData, color: &str) -> Coloring<'a> {
        let coloring = match color {
            "binary" => ColorModel::Binary,
            "bluegreen" => ColorModel::BlueGreen,
            "rainbow" => ColorModel::Rainbow,
            "topographical" => ColorModel::Topographical,
            _ => ColorModel::Binary,
        };

        return Coloring { data : data, 
                          colormod : coloring,
                          image: None,
                          imagevec: None,
                          waterlevel: 0.0, //TODO : make this variable
                        };
    }

    // -- Getters --

    pub fn get_image(&mut self) -> &RgbaImage {
        if self.image == None {
            self.image = Some(RgbaImage::new(self.data.width, self.data.height));
            match &self.colormod {
                ColorModel::Binary => self.data_to_binary(),
                ColorModel::BlueGreen => self.data_to_blue_green(self.waterlevel),
                ColorModel::Rainbow => self.data_to_rainbow(),
                _ => self.data_to_binary(),
            }
        }

        match &self.image {
            Some(x) => return &x,
            None => panic!("Shouldn't be possible"), //TODO : Should be an error here
        }
    }

    pub fn get_vec(&mut self) -> Vec<u8> {
        if self.imagevec == None {
            self.imagevec = Some(Vec::new());

            match &self.colormod {
                ColorModel::BlueGreen => self.data_to_blue_green_vec(self.waterlevel),
                ColorModel::Rainbow => self.data_to_rainbow_vec(),
                ColorModel::Topographical => self.data_to_topographical_vec(self.waterlevel),
                _ => self.data_to_rainbow_vec(),
            }
        }

        match &self.imagevec {
            Some(x) => return x.to_vec(),
            None => return Vec::new(),
        }
    }

    // -- Data Manipulation --

    //pub fn scale_brightness(&self) {
    //    let map_vec = &self.data;

    //    for x in &map_vec.data {
    //        {}   
    //    }
    //}

    // -- Data to Image Conversions --

    pub fn data_to_binary(&mut self) {
        let map_vec = self.data;
        let image = match &mut self.image {
            None => panic!("Shouldn't be possible"), //TODO: improve this
            Some(x) => x,
        };

        let mut counterx = 0;
        let mut countery = 0;

        for x in &map_vec.data {
            if x <= &0.0 {
                image.put_pixel(counterx, countery, Rgba([0, 0, 0, 255]));
            } else {
                image.put_pixel(counterx, countery, Rgba([255, 255, 255, 255]));
            }

            counterx = (counterx + 1) % map_vec.width;
            if counterx == 0 { countery += 1; }
        }
    }

    pub fn data_to_bluegreen_rgba(datapoint: &f64, water: &f64) -> [u8; 4] {
        let mut rgbvalue = (datapoint * 125.0) as u8;
        if rgbvalue > 125 {
            rgbvalue = 250;
        } else {
            rgbvalue = rgbvalue + 125;
        }

        if datapoint < water {
            return [0, 0, rgbvalue, 255];
        } else {
            return [0, rgbvalue, 0, 255];
        }
    }

    /*
    pub fn data_to_bluegreen_output(&mut self, waterperc: f64, output: OutputFormat) {
        let map_vec = self.data;
        let image = match &mut self.image {
            Some(x) => x,
            _ => panic!("Shouldn't be possible"),
        };

        let mut tempdata = map_vec.data.clone();
        let initvalue = (tempdata.len() as f64) * waterperc;
        let water = util::quickselect(&mut tempdata, initvalue as u32);

        match output {
            Image => (),
            ImageVec => (),
        }
    }
    */

    pub fn data_to_blue_green(&mut self, waterperc: f64) {
        let map_vec = self.data;
        let image = match &mut self.image {
            None => panic!("Shouldn't be possible"),
            Some(x) => x,
        };

        //let mut counterx = 0;
        //let mut countery = 0;

        let mut tempdata = map_vec.data.clone();
        let initvalue = (tempdata.len() as f64) * waterperc;
        let water = util::quickselect(&mut tempdata, initvalue as u32);

        //image.enumerate_pixels().map(|(x, y, p)| (x, y, p));

        for (x, y, p) in image.enumerate_pixels_mut() {
            //let Rgba([r, g, b, a]) = p;
            //*r = 100;
            //*g = 100;
            //*b = 100;
            let new_point = map_vec.get_pixel(x,y);
            *p = Rgba(Coloring::data_to_bluegreen_rgba(&new_point, &water));

            //let new_point = map_vec.get_pixel(x, y);
            //mage.put_pixel(x, y, Rgba(Coloring::data_to_bluegreen_rgba(&new_point, &water)));
        }

        //for (x, y, p) in image.enumerate_pixels() {
        //    image.put_pixel(x, y, Rgba([0, 100, 100, 255]));
        //}

        //for point in &map_vec.data {
        //    let rgba_vec = Coloring::data_to_bluegreen_rgba(&point, &water);
            //image.put_pixel(counterx, countery, Rgba(rgba_vec));

            //counterx = (counterx + 1) % map_vec.width;
            //if counterx == 0 { countery += 1; }
        //}
    }

    pub fn data_to_blue_green_vec(&mut self, waterperc: f64) {
        let map_vec = self.data;
        let imagevec = match &mut self.imagevec {
            None => panic!("Shouldn't be possible"),
            Some(x) => x,
        };

        let mut tempdata = map_vec.data.clone();
        let initvalue = (tempdata.len() as f64) * waterperc;
        let water = util::quickselect(&mut tempdata, initvalue as u32);

        for x in &map_vec.data {
            let rgba_vec = Coloring::data_to_bluegreen_rgba(&x, &water);
            imagevec.append(&mut rgba_vec.to_vec());
        }
    }

    pub fn data_to_rainbow(&mut self) {
        let map_vec = &self.data;
        let image = match &mut self.image {
            None => panic!("Shouldn't be possible"),
            Some(x) => x,
        };

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

        let mut counterx = 0;
        let mut countery = 0;

        for x in &map_vec.data {
            let greenv: u8 = normalize(x, 0.0);
            let bluev: u8 = normalize(x, 0.33);
            let redv: u8 = normalize_edge(x);

            image.put_pixel(counterx, countery, Rgba([redv, greenv, bluev, 255]));

            counterx = (counterx + 1) % map_vec.width;
            if counterx == 0 { countery += 1; }
        }
    }

    pub fn data_to_rainbow_vec(&mut self) {
        let map_vec = &self.data;
        let imagevec = match &mut self.imagevec {
            None => panic!("Shouldn't be possible"),
            Some(x) => x,
        };

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

        for x in &map_vec.data {
            let greenv: u8 = normalize(x, 0.0);
            let bluev: u8 = normalize(x, 0.33);
            let redv: u8 = normalize_edge(x);

            imagevec.append(&mut vec![redv, greenv, bluev, 255]);
        }
    }

    pub fn data_to_topographical_vec(&mut self, water: f64) {
        let map_vec = &self.data;
        let imagevec = match &mut self.imagevec {
            None => panic!("Shouldn't be possible"),
            Some(x) => x,
        };

        let waterperc = water;

        let mut tempdata = map_vec.data.clone();
        let initvalue = (tempdata.len() as f64) * waterperc;
        let waterheight = util::quickselect(&mut tempdata, initvalue as u32);

        for x in &map_vec.data {
            let fract = (x * 10.0).fract();

            if x < &waterheight {
                imagevec.append(&mut vec![0, 0, 255, 255]);
            } else if (fract < 0.1) | (fract > 0.9) {
                imagevec.append(&mut vec![0, 0, 0, 255]);
            } else {
                imagevec.append(&mut vec![255, 255, 255, 255]);
            }
        }
    }
}
