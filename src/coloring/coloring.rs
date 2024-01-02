use image::{RgbaImage, Rgba};

use crate::map_data::MapData;
use crate::utilities::util::util;

use crate::ColorArgs;

pub struct Coloring<'a> {
    pub data: &'a MapData,
    pub colormod: ColorModel,
    pub image: Option<RgbaImage>,
    pub imagevec: Option<Vec<u8>>,
    pub waterlevel: f64,
    pub darklevel: u8,
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
    pub fn new<'a>(data: &'a MapData, colorargs: ColorArgs) -> Coloring<'a> {
        let coloring = match colorargs.get_coloring() {
            1 => ColorModel::BlueGreen,
            2 => ColorModel::Rainbow,
            3 => ColorModel::Topographical,
            4 => ColorModel::Binary,
            _ => ColorModel::Binary,
        };

        return Coloring { data : data, 
                          colormod : coloring,
                          image: None,
                          imagevec: None,
                          waterlevel: colorargs.get_waterlevel(),
                          darklevel: (colorargs.get_darklevel() * 255.0) as u8,
                        };
    }

    // -- Getters --
    pub fn init_image(&mut self) {
        self.image = Some(RgbaImage::new(self.data.width, self.data.height));
    }

    pub fn get_image(&mut self) -> &RgbaImage {
        if self.image == None {
            self.image = Some(RgbaImage::new(self.data.width, self.data.height));
            match &self.colormod {
                ColorModel::Binary =>           self.data_to_binary(),
                ColorModel::BlueGreen =>        self.data_to_blue_green(self.waterlevel),
                ColorModel::Rainbow =>          self.data_to_rainbow(),
                ColorModel::Topographical =>    self.data._to_topographical(self.waterlevel),
                _ =>                            self.data_to_binary(),
            }
        }

        //self.image is an Option set to Some immediately above and should never panic here
        return self.image.as_ref().unwrap();
    }

    pub fn get_vec(&mut self) -> Vec<u8> {
        if self.imagevec == None {
            self.imagevec = Some(Vec::new());
        
            match &self.colormod {
                ColorModel::BlueGreen =>        self.data_to_blue_green_vec(self.waterlevel),
                ColorModel::Rainbow =>          self.data_to_rainbow_vec(),
                ColorModel::Topographical =>    self.data_to_topographical_vec(self.waterlevel),
                _ =>                            self.data_to_blue_green_vec(self.waterlevel),
            }
        }

        //self.imagevec is an option set to Some immediately above and should (hopefully) never panic here
        return self.imagevec.as_ref().unwrap().to_vec();
    }

    pub fn get_waterlevel(&self, waterperc: f64) -> f64 {
        let mut tempdata = self.data.data.clone();
        let initvalue = (tempdata.len() as f64) * waterperc;
        let water = util::quickselect(&mut tempdata, initvalue as u32);
        return water;
    }

    // -- Data Manipulation --

    //pub fn scale_brightness(&self) {
    //    let map_vec = &self.data;

    //    for x in &map_vec.data {
    //        {}   
    //    }
    //}

    // -- Data to Image Conversions --

    fn data_to_binary(&mut self) {
        let map_vec = self.data;

        if self.image == None { self.init_image(); }
        let image = self.image.as_mut().unwrap();

        let mut counterx = 0;
        let mut countery = 0;

        for x in &map_vec.data {
            if x <= &0.0 {
                image.put_pixel(counterx, countery, Rgba([0, 0, 0, self.darklevel]));
            } else {
                image.put_pixel(counterx, countery, Rgba([255, 255, 255, self.darklevel]));
            }

            counterx = (counterx + 1) % map_vec.width;
            if counterx == 0 { countery += 1; }
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

    fn data_to_bluegreen_rgba(datapoint: &f64, water: &f64, dark: u8) -> [u8; 4] {
        let mut rgbvalue = (datapoint * 125.0) as u8;

        if rgbvalue > 125       { rgbvalue = 250; } 
        else                    { rgbvalue = rgbvalue + 125; }

        if datapoint < water    { return [0, 0, rgbvalue, dark]; } 
        else                    { return [0, rgbvalue, 0, dark]; }
    }

    fn data_to_blue_green(&mut self, waterperc: f64) {
        let map_vec = self.data;
        let water = self.get_waterlevel(waterperc);

        if self.image == None { self.init_image(); }
        let image = self.image.as_mut().unwrap();

        for (x, y, p) in image.enumerate_pixels_mut() {
            let new_point = map_vec.get_pixel(x,y);
            *p = Rgba(Coloring::data_to_bluegreen_rgba(&new_point, &water, self.darklevel));
        }
    }

    fn data_to_blue_green_vec(&mut self, waterperc: f64) {
        let map_vec = self.data;
        let water = self.get_waterlevel(waterperc);

        if self.imagevec == None { self.imagevec = Some(Vec::new()) }
        let imagevec = self.imagevec.as_mut().unwrap();

        for x in &map_vec.data {
            let rgba_vec = Coloring::data_to_bluegreen_rgba(&x, &water, self.darklevel);
            imagevec.append(&mut rgba_vec.to_vec());
        }
    }

    fn data_to_rainbow_rgba(x: &f64) -> [u8; 4] {
        fn normalize(value: &f64, low: f64) -> u8 {
            if value < &low                         { return 0; }
            else if value < &(low + 0.16)           { return ((value - low) * 6.0 * 255.0) as u8; }
            else if value < &(low + 0.5)            { return 255; }
            else if value < &(low + 0.66)           { return ((1.0 - ((value - 0.5) * 6.0)) * 255.0) as u8; }
            else                                    { return 0; }
        }

        fn normalize_edge(value: &f64) -> u8 {
            if (value < &0.16) | (value > &0.84)    { return 255; }
            else if value < &0.33                   { return ((1.0 - ((value - 0.16) * 6.0)) * 255.0) as u8; }
            else if value > &0.66                   { return ((value - 0.66) * 6.0 * 255.0) as u8; }
            else                                    { return 0; }
        }

        return [ normalize(x, 0.0), normalize(x, 0.33), normalize_edge(x), 255]
    }

    fn data_to_rainbow(&mut self) {
        let map_vec = self.data;

        if self.image == None { self.init_image(); }
        let image = self.image.as_mut().unwrap();

        for (x, y, p) in image.enumerate_pixels_mut() {
            let new_point = map_vec.get_pixel(x, y);
            *p = Rgba(Coloring::data_to_rainbow_rgba(&new_point))
        }
    }

    fn data_to_rainbow_vec(&mut self) {
        let map_vec = self.data;

        if self.imagevec == None { self.imagevec = Some(Vec::new()) }
        let imagevec = self.imagevec.as_mut().unwrap();

        for x in &map_vec.data {
            let rgba_vec = Coloring::data_to_rainbow_rgba(x);
            imagevec.append(&mut rgba_vec.to_vec());
        }
    }

    fn data_to_topographical_rgba(datapoint: &f64, waterheight: &f64) -> [u8; 4] {
        let fract = (datapoint * 10.0).fract();

        if (fract < 0.1) | (fract > 0.9)       { return [0, 0, 0,   255] }
        else if datapoint < &waterheight       { return [0, 0, 255, 255] }
        else                                   { return [255, 255, 255, 255] }
    }

    fn data_to_topographical(&mut self, waterperc: f64) {
        let map_vec = self.data;
        let water = self.get_waterlevel(waterperc);

        if self.image == None { self.init_image(); }
        let image = self.image.as_mut().unwrap();

        for (x, y, p) in image.enumerate_pixels_mut() {
            let new_point = map_vec.get_pixel(x,y);
            *p = Rgba(Coloring::data_to_topographical_rgba(&new_point, &water));
        }
    }

    fn data_to_topographical_vec(&mut self, waterperc: f64) {
        let map_vec = self.data;
        let water = self.get_waterlevel(waterperc);

        if self.imagevec == None { self.imagevec = Some(Vec::new()) }
        let imagevec = self.imagevec.as_mut().unwrap();

        for x in &map_vec.data {
            let rgba_vec = Coloring::data_to_topographical_rgba(x, &water);
            imagevec.append(&mut rgba_vec.to_vec());
        }
    }
}
