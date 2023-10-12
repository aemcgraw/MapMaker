use crate::map_data::MapData;
use crate::algorithms::{GetData, GetDim, Run, Size, Save, ImageAlg};

use rand::Rng;
use rand::distributions::{Distribution, Uniform};
use rand::rngs::StdRng;
use rand::SeedableRng;

pub struct DiamondSquareBorderless {
    data : MapData,
    dim : u32,
    algrng : StdRng,
}

impl DiamondSquareBorderless {
    pub fn new(width: u32, height: u32, seed: u64) -> DiamondSquareBorderless {
        let bsize = DiamondSquareBorderless::size(width, height);

        let vec_size = bsize * bsize;
        let backing_data : Vec<f64> = vec!(0.0; vec_size as usize);
        let this_rng = SeedableRng::seed_from_u64(seed);
        return DiamondSquareBorderless { data : MapData::new(backing_data, bsize, bsize), dim : bsize, algrng : this_rng }
    }

    fn diamond(&mut self, stepsize : u32, chaos: f64) {
        let half = stepsize / 2;

        let sampler = Uniform::new_inclusive(-chaos, chaos);
        let start = self.dim / 6;

        for x in (start..self.dim + start).step_by(stepsize as usize) {
            for y in (start..self.dim + start).step_by(stepsize as usize) {
                let z : f64 = (self.data.get_pixel_modular(x, y) + 
                              self.data.get_pixel_modular(x + stepsize, y) + 
                              self.data.get_pixel_modular(x, y + stepsize) + 
                              self.data.get_pixel_modular(x + stepsize, y + stepsize)) / 4.0;
                self.data.put_pixel_modular(x + half, y + half, z + sampler.sample(&mut self.algrng));
            }
        }
    }

    fn square(&mut self, stepsize : u32, chaos: f64) {
        let mut z: f64;
        let sampler = Uniform::new_inclusive(-chaos, chaos);

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
                self.data.put_pixel_modular(x + half, y, z + sampler.sample(&mut self.algrng));

                z = (self.data.get_pixel_modular(midx, midy) +
                    self.data.get_pixel_modular(x, y) +
                    self.data.get_pixel_modular(x, y + stepsize) +
                    self.data.get_pixel_modular(x - half, midy)) / 4.0;
                self.data.put_pixel_modular(x, y + half, z + sampler.sample(&mut self.algrng));

                z = (self.data.get_pixel_modular(midx, midy) +
                    self.data.get_pixel_modular(x + stepsize, y) +
                    self.data.get_pixel_modular(x + stepsize, y + stepsize) +
                    self.data.get_pixel_modular(midx + stepsize, midy)) / 4.0;
                self.data.put_pixel_modular(x + stepsize, y + half, z + sampler.sample(&mut self.algrng));
                
                z = (self.data.get_pixel_modular(midx, midy) +
                    self.data.get_pixel_modular(x, y + stepsize) +
                    self.data.get_pixel_modular(x + stepsize, y + stepsize) +
                    self.data.get_pixel_modular(midx, midy + stepsize)) / 4.0;
                self.data.put_pixel_modular(x + half, y + stepsize, z + sampler.sample(&mut self.algrng));
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


impl GetDim for DiamondSquareBorderless {
    fn get_dim(&self) -> u32 {
        return self.dim;
    }
}

impl Run for DiamondSquareBorderless {
    fn run(&mut self, chaos: f64, damping: f64, _blocksize: u32) {
        //let mut rng = rand::thread_rng();

        let size = self.dim;
        println!("{}", size);
        let mut stepsize = size / 3;
        let half = stepsize / 2;

        for x in (half..(size-half+1)).step_by(stepsize as usize) {
            for y in (half..(size-half+1)).step_by(stepsize as usize) {
                self.data.put_pixel(x, y, self.algrng.gen_range(0.0..1.0));
                //self.data.put_pixel(x, y, 0.5);
            }
        }

        let mut chaos: f64 = chaos;
        let damping: f64 = damping;

        while (stepsize) >= 2 {

            self.diamond(stepsize, chaos);
            self.square(stepsize, chaos);
        
            stepsize = stepsize / 2;
            chaos = chaos * damping;
        }
    }
}

impl Save for DiamondSquareBorderless {}

/* 
In order to achieve a borderless map the 
size given for width and height must be
some multiple of 3.
Such as : 3, 6, 12, 24, 48, 96, 192, 384, 768
This restriction should be fixed in the future
*/
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
