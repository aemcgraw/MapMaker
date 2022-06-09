use crate::map_data::MapData;
use crate::algorithms::{GetData, Run, Size, Save, ImageAlg};

use image::RgbImage;
use rand::Rng;

pub struct DiamondSquare {
    image : RgbImage,
    data : MapData,
    pub dim : u32,
}

impl DiamondSquare {
    pub fn new(width: u32, height: u32) -> DiamondSquare {
        let bsize = DiamondSquare::size(width, height);

        let vec_size = bsize * bsize;
        let backing_data : Vec<f64> = vec!(0.0; vec_size as usize);
        return DiamondSquare { image : RgbImage::new(bsize, bsize), data : MapData::new(backing_data, bsize, bsize), dim : bsize }
    }

    fn diamond(&mut self, stepsize : u32, chaos: f64) {
        let newsize = stepsize / 2;
        let mut rng = rand::thread_rng(); 

        for x in (0..self.dim - 1).step_by(stepsize as usize) {
            for y in (0..self.dim - 1).step_by(stepsize as usize) {
                let z : f64 = (self.data.get_pixel(x, y) + 
                              self.data.get_pixel(x + stepsize, y) + 
                              self.data.get_pixel(x, y + stepsize) + 
                              self.data.get_pixel(x + stepsize, y + stepsize)) / 4.0;
                self.data.put_pixel(x + newsize, y + newsize, z + rng.gen_range(-chaos..chaos));
            }
        }
    }

    fn square(&mut self, stepsize : u32, chaos: f64) {
        let mut z: f64;
        let mut rng = rand::thread_rng();

        let half = stepsize / 2;
        for x in (half..self.dim - 1).step_by(stepsize as usize) {
            z = (self.data.get_pixel(x - half, 0) +
                         self.data.get_pixel(x + half, 0) +
                         self.data.get_pixel(x, half)) / 3.0;
            self.data.put_pixel(x, 0, z + rng.gen_range(-chaos..chaos));
        }

        for x in (half..self.dim - 1).step_by(stepsize as usize) {
            z = (self.data.get_pixel(x - half, self.dim - 1) +
                         self.data.get_pixel(x + half, self.dim - 1) +
                         self.data.get_pixel(x, self.dim - 1 - half)) / 3.0;
            self.data.put_pixel(x, self.dim - 1, z + rng.gen_range(-chaos..chaos));
        }

        for y in (half..self.dim - 1).step_by(stepsize as usize) {
            z = (self.data.get_pixel(0, y - half) +
                         self.data.get_pixel(0, y + half) +
                         self.data.get_pixel(half, y)) / 3.0;
            self.data.put_pixel(0, y, z + rng.gen_range(-chaos..chaos));
        }

        for y in (half..self.dim - 1).step_by(stepsize as usize) {
            z = (self.data.get_pixel(self.dim - 1, y - half) +
                         self.data.get_pixel(self.dim - 1, y + half) +
                         self.data.get_pixel(self.dim - 1 - half, y)) / 3.0;
            self.data.put_pixel(self.dim - 1, y, z + rng.gen_range(-chaos..chaos));
        }

        for x in (half..self.dim - 1 - half).step_by(stepsize as usize) {
            for y in (half..self.dim - 1 - half).step_by(stepsize as usize) {
                let midx = x + half;
                let midy = y + half;

                z = (self.data.get_pixel(midx, midy) +
                    self.data.get_pixel(midx, y - half) +
                    self.data.get_pixel(x, y) +
                    self.data.get_pixel(x + stepsize, y)) / 4.0;
                self.data.put_pixel(x + half, y, z + rng.gen_range(-chaos..chaos));

                z = (self.data.get_pixel(midx, midy) +
                    self.data.get_pixel(x, y) +
                    self.data.get_pixel(x, y + stepsize) +
                    self.data.get_pixel(x - half, midy)) / 4.0;
                self.data.put_pixel(x, y + half, z + rng.gen_range(-chaos..chaos));

                z = (self.data.get_pixel(midx, midy) +
                    self.data.get_pixel(x + stepsize, y) +
                    self.data.get_pixel(x + stepsize, y + stepsize) +
                    self.data.get_pixel(midx + stepsize, midy)) / 4.0;
                self.data.put_pixel(x + stepsize, y + half, z + rng.gen_range(-chaos..chaos));
                
                z = (self.data.get_pixel(midx, midy) +
                    self.data.get_pixel(x, y + stepsize) +
                    self.data.get_pixel(x + stepsize, y + stepsize) +
                    self.data.get_pixel(midx, midy + stepsize)) / 4.0;
                self.data.put_pixel(x + half, y + stepsize, z + rng.gen_range(-chaos..chaos));
            }
        }
    }

    //fn square_old(&mut self, stepsize : u32) {
    //    for x in (stepsize..self.dim - 1).step_by((stepsize * 2) as usize) {
    //        for y in (stepsize..self.dim - 1).step_by((stepsize * 2) as usize) {
    //            let mut scale = 4.0;
    //
    //            if !self.data.in_bounds(x - stepsize, y) |
    //                    !self.data.in_bounds(x, y + stepsize) |
    //                    !self.data.in_bounds(x + stepsize, y) |
    //                    !self.data.in_bounds(x, y - stepsize) {
    //                scale = 3.0;
    //               }

    //            let z : f64 = (self.data.get_pixel(x - stepsize, y) +
    //                          self.data.get_pixel(x, y + stepsize) +
    //                          self.data.get_pixel(x + stepsize, y) +
    //                          self.data.get_pixel(x, y - stepsize)) / scale;
    //            self.data.put_pixel(x, y, z);
    //        }
    //    }
    //}

/*
    pub fn to_imagedata(&self, coloring: &str) -> Vec<u8> {
        let image = self.data.to_imagedata(coloring);
        return image;
    }

}
*/

/*
impl ToImage for DiamondSquare {
    fn to_image(&self, algo: &str) -> RgbImage {
        let image = self.data.to_image(algo);
        return image;
    }
*/
}


impl GetData for DiamondSquare {
    fn get_data(&self) -> &MapData {
        return &self.data;
    }
}

impl Run for DiamondSquare {
    fn run(&mut self, chaos: f64) {
        let mut rng = rand::thread_rng();

        let init = self.dim - 1;
        self.data.put_pixel(0, 0, rng.gen_range(0.0..1.0));
        self.data.put_pixel(0, init, rng.gen_range(0.0..1.0));
        self.data.put_pixel(init, 0, rng.gen_range(0.0..1.0));
        self.data.put_pixel(init, init, rng.gen_range(0.0..1.0));

        let mut chaos: f64 = chaos;
        let damping: f64 = 0.8;

        let mut stepsize = self.dim - 1;
        while (stepsize) >= 2 {

            self.diamond(stepsize, chaos);
            self.square(stepsize, chaos);
        
            stepsize = stepsize / 2;
            chaos = chaos * damping;
        }
    }
}

impl Save for DiamondSquare {}

impl Size for DiamondSquare {
    fn size(width: u32, height: u32) -> u32 {
        let largest = std::cmp::max(width, height);
        let mut x : u32 = 1;
        while x < largest {
            x = x * 2;
        }
        x = x + 1;
        return x
    }
}

impl ImageAlg for DiamondSquare {}
