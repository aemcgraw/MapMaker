use crate::map_data::MapData;
use crate::algorithms::{GetData, GetDim, Run, Size, Save, ImageAlg};

use image::RgbImage;
use rand::distributions::{Distribution, Uniform};

pub struct DiamondSquare {
    image : RgbImage,
    data : MapData,
    pub dim : u32,
}

impl DiamondSquare {
    //The Diamond-Square Algorithm is only (currently) implemented against 
    // square maps of side length in pixels of the form ((2 ^ n) + 1).
    //To this end if the requested image has a different size we produce a map of 
    // the smallest size that can contain the requested size and then simply
    // compress the resulting map to the requested size.
    pub fn new(width: u32, height: u32) -> DiamondSquare {
        let bsize = DiamondSquare::size(width, height);

        let vec_size = bsize * bsize;
        let backing_data : Vec<f64> = vec!(0.0; vec_size as usize);
        return DiamondSquare { image : RgbImage::new(bsize, bsize), data : MapData::new(backing_data, bsize, bsize), dim : bsize }
    }

    fn diamond(&mut self, stepsize : u32, chaos: f64) {
        let newsize = stepsize / 2;
        let mut rng = rand::thread_rng(); 

        let sampler = Uniform::new_inclusive(-chaos, chaos);

        for x in (0..self.dim - 1).step_by(stepsize as usize) {
            for y in (0..self.dim - 1).step_by(stepsize as usize) {
                let z : f64 = (self.data.get_pixel(x, y) + 
                              self.data.get_pixel(x + stepsize, y) + 
                              self.data.get_pixel(x, y + stepsize) + 
                              self.data.get_pixel(x + stepsize, y + stepsize)) / 4.0;
                self.data.put_pixel(x + newsize, y + newsize, z + sampler.sample(&mut rng));
            }
        }
    }

    fn square(&mut self, stepsize : u32, chaos: f64) {
        let mut z: f64;
        let mut rng = rand::thread_rng();

        let sampler = Uniform::new_inclusive(-chaos, chaos);

        let half = stepsize / 2;
        for x in (half..self.dim - 1).step_by(stepsize as usize) {
            z = (self.data.get_pixel(x - half, 0) +
                         self.data.get_pixel(x + half, 0) +
                         self.data.get_pixel(x, half)) / 3.0;
            self.data.put_pixel(x, 0, z + sampler.sample(&mut rng));
        }

        for x in (half..self.dim - 1).step_by(stepsize as usize) {
            z = (self.data.get_pixel(x - half, self.dim - 1) +
                         self.data.get_pixel(x + half, self.dim - 1) +
                         self.data.get_pixel(x, self.dim - 1 - half)) / 3.0;
            self.data.put_pixel(x, self.dim - 1, z + sampler.sample(&mut rng));
        }

        for y in (half..self.dim - 1).step_by(stepsize as usize) {
            z = (self.data.get_pixel(0, y - half) +
                         self.data.get_pixel(0, y + half) +
                         self.data.get_pixel(half, y)) / 3.0;
            self.data.put_pixel(0, y, z + sampler.sample(&mut rng));
        }

        for y in (half..self.dim - 1).step_by(stepsize as usize) {
            z = (self.data.get_pixel(self.dim - 1, y - half) +
                         self.data.get_pixel(self.dim - 1, y + half) +
                         self.data.get_pixel(self.dim - 1 - half, y)) / 3.0;
            self.data.put_pixel(self.dim - 1, y, z + sampler.sample(&mut rng));
        }

        for x in (half..self.dim - 1 - half).step_by(stepsize as usize) {
            for y in (half..self.dim - 1 - half).step_by(stepsize as usize) {
                let midx = x + half;
                let midy = y + half;

                z = (self.data.get_pixel(midx, midy) +
                    self.data.get_pixel(midx, y - half) +
                    self.data.get_pixel(x, y) +
                    self.data.get_pixel(x + stepsize, y)) / 4.0;
                self.data.put_pixel(x + half, y, z + sampler.sample(&mut rng));

                z = (self.data.get_pixel(midx, midy) +
                    self.data.get_pixel(x, y) +
                    self.data.get_pixel(x, y + stepsize) +
                    self.data.get_pixel(x - half, midy)) / 4.0;
                self.data.put_pixel(x, y + half, z + sampler.sample(&mut rng));

                z = (self.data.get_pixel(midx, midy) +
                    self.data.get_pixel(x + stepsize, y) +
                    self.data.get_pixel(x + stepsize, y + stepsize) +
                    self.data.get_pixel(midx + stepsize, midy)) / 4.0;
                self.data.put_pixel(x + stepsize, y + half, z + sampler.sample(&mut rng));
                
                z = (self.data.get_pixel(midx, midy) +
                    self.data.get_pixel(x, y + stepsize) +
                    self.data.get_pixel(x + stepsize, y + stepsize) +
                    self.data.get_pixel(midx, midy + stepsize)) / 4.0;
                self.data.put_pixel(x + half, y + stepsize, z + sampler.sample(&mut rng));
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
}


impl GetData for DiamondSquare {
    fn get_data(&self) -> &MapData {
        return &self.data;
    }
}


impl GetDim for DiamondSquare {
    fn get_dim(&self) -> u32 {
        return self.dim;
    }
}

impl Run for DiamondSquare {
    fn run(&mut self, chaos: f64, damping: f64, blocksize: u32) {
        let mut rng = rand::thread_rng();
        let sampler = Uniform::new_inclusive(0.0, 1.0);

        //blocksize must be a power of 2 and less than or equal to the edge length of the map
        let mut blocksize = blocksize;
        if (blocksize as f32).log2().fract() != 0.0 {
            let mut x: u32 = 1;
            while x < blocksize && (x * 2) < self.dim {
                x = x * 2;
            }
            blocksize = x;
        }

        for x in (0..self.dim).step_by(blocksize as usize) {
            for y in (0..self.dim).step_by(blocksize as usize) {
                self.data.put_pixel(x, y, sampler.sample(&mut rng));
            }
        }

        let mut chaos: f64 = chaos;
        let mut stepsize = blocksize;
        while (stepsize) >= 2 {

            self.diamond(stepsize, chaos);
            self.square(stepsize, chaos);
        
            stepsize = stepsize / 2;
            chaos = chaos * damping;
        }
    }
}

impl Save for DiamondSquare {}

//Returns 1 more than the smallest power of 2 greater than or equal to max(width, height)
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
