use crate::map_data::MapData;
use crate::algorithms::{GetData, Run, Size, Save, ImageAlg};

use image::RgbImage;
use rand::Rng;

pub struct DiamondSquareBorderless {
    image : RgbImage,
    data : MapData,
    dim : u32,
}

impl DiamondSquareBorderless {
    pub fn new(width: u32, height: u32) -> DiamondSquareBorderless {
        let bsize = DiamondSquareBorderless::size(width, height);

        let vec_size = bsize * bsize;
        let backing_data : Vec<f64> = vec!(0.0; vec_size as usize);

        return DiamondSquareBorderless { image : RgbImage::new(bsize, bsize), data : MapData::new(backing_data, bsize, bsize), dim : bsize }
    }

    fn diamond(&mut self, stepsize : u32, chaos: f64) {
        let half = stepsize / 2;
        let mut rng = rand::thread_rng(); 

        let start = self.dim / 6;

        for x in (start..self.dim + start).step_by(stepsize as usize) {
            for y in (start..self.dim + start).step_by(stepsize as usize) {
                let z : f64 = (self.data.get_pixel_modular(x, y) + 
                              self.data.get_pixel_modular(x + stepsize, y) + 
                              self.data.get_pixel_modular(x, y + stepsize) + 
                              self.data.get_pixel_modular(x + stepsize, y + stepsize)) / 4.0;
                self.data.put_pixel_modular(x + half, y + half, z + rng.gen_range(-chaos..chaos));
            }
        }
    }

    fn square(&mut self, stepsize : u32, chaos: f64) {
        let mut z: f64;
        let mut rng = rand::thread_rng();

        let half = stepsize / 2;
        let start = self.dim / 6;

        for x in (start..self.dim + start).step_by(stepsize as usize) {
            for y in (start..self.dim + start).step_by(stepsize as usize) {
                let midx = x + half;
                let midy = y + half;

                z = (self.data.get_pixel_modular(midx, midy) +
                    self.data.get_pixel_modular(midx, y - half) +
                    self.data.get_pixel_modular(x, y) +
                    self.data.get_pixel_modular(x, y + stepsize)) / 4.0;
                self.data.put_pixel_modular(x + half, y, z + rng.gen_range(-chaos..chaos));

                z = (self.data.get_pixel_modular(midx, midy) +
                    self.data.get_pixel_modular(x, y) +
                    self.data.get_pixel_modular(x, y + stepsize) +
                    self.data.get_pixel_modular(x - half, midy)) / 4.0;
                self.data.put_pixel_modular(x, y + half, z + rng.gen_range(-chaos..chaos));

                z = (self.data.get_pixel_modular(midx, midy) +
                    self.data.get_pixel_modular(x + stepsize, y) +
                    self.data.get_pixel_modular(x + stepsize, y + stepsize) +
                    self.data.get_pixel_modular(midx + stepsize, midy)) / 4.0;
                self.data.put_pixel_modular(x + stepsize, y + half, z + rng.gen_range(-chaos..chaos));
                
                z = (self.data.get_pixel_modular(midx, midy) +
                    self.data.get_pixel_modular(x, y + stepsize) +
                    self.data.get_pixel_modular(x + stepsize, y + stepsize) +
                    self.data.get_pixel_modular(midx, midy + stepsize)) / 4.0;
                self.data.put_pixel_modular(x + half, y + stepsize, z + rng.gen_range(-chaos..chaos));
            }
        }
    }

    //pub fn save(&self, image: RgbImage, name: std::path::PathBuf) {
    //    let save_path = Path::new(config::OUTPUT_DIR).join(name.as_path());
    //    image.save(save_path).expect("Couldn't save image");
    //}
}

impl GetData for DiamondSquareBorderless {
    fn get_data(&self) -> &MapData {
        return &self.data;
    }
}

impl Run for DiamondSquareBorderless {
    fn run(&mut self, chaos: f64, damping: f64, blocksize: u32) {
        let mut rng = rand::thread_rng();

        let size = self.dim;
        println!("{}", size);
        let mut stepsize = size / 3;
        let half = stepsize / 2;

        for x in (half..(size-half+1)).step_by(stepsize as usize) {
            for y in (half..(size-half+1)).step_by(stepsize as usize) {
                self.data.put_pixel(x, y, rng.gen_range(0.0..1.0));
                //self.data.put_pixel(x, y, 0.5);
            }
        }

        let mut chaos: f64 = chaos;
        let damping: f64 = 0.8;

        while (stepsize) >= 2 {

            self.diamond(stepsize, chaos);
            self.square(stepsize, chaos);
        
            stepsize = stepsize / 2;
            chaos = chaos * damping;
        }
    }
}

/*
impl ToImage for DiamondSquareBorderless {
    fn to_image(&self, coloring: &str) -> RgbImage {
        let image = self.data.to_image(coloring);
        return image;
    }
}
*/

impl Save for DiamondSquareBorderless {}

impl Size for DiamondSquareBorderless {
    fn size(width: u32, height: u32) -> u32 {
        let largest = std::cmp::max(width, height);
        let mut x : u32 = 3;
        while x < largest {
            x = x * 2;
        }
        x = x;
        return x
    }
}

impl ImageAlg for DiamondSquareBorderless {}
