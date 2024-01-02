pub mod algorithms;
pub mod map_data;
pub mod config;
pub mod utilities;
pub mod coloring;

use wasm_bindgen::prelude::*;
use wasm_bindgen::Clamped;
use web_sys::{CanvasRenderingContext2d, ImageData};

use rand::Rng;

use algorithms::ImageAlg;
use algorithms::diamond_square::DiamondSquare;
use algorithms::diamond_square_borderless::DiamondSquareBorderless;
//use util::kdtree::KDTree;

use coloring::coloring::Coloring;

//Allows logging to the browser console with the console_log! macro
#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = console)]
    fn log(s: &str);
}

macro_rules! console_log {
    ($($t:tt)*) => (log(&format_args!($($t)*).to_string()))
}

#[wasm_bindgen]
pub struct MapArgs {
    pub width: u32,
    pub height: u32,
    pub chaos: f64,
    pub damping: f64,
    pub blocksize: u32,
    pub seed: u32,
}

#[wasm_bindgen]
impl MapArgs {
    #[wasm_bindgen(constructor)]
    pub fn new(width: u32, height: u32, chaos: f64, damping: f64, blocksize: u32, seed: u32) -> MapArgs {
        MapArgs { width, height, chaos, damping, blocksize, seed }
    }

    pub fn get_width(&self) -> u32 { return self.width }
    pub fn get_height(&self) -> u32 { return self.height }
    pub fn get_chaos(&self) -> f64 { return self.chaos }
    pub fn get_damping(&self) -> f64 { return self.damping }
    pub fn get_blocksize(&self) -> u32 { return self.blocksize }
    pub fn get_seed(&self) -> u64 {
        if self.seed == 0 {
            let mut rng = rand::thread_rng();
            return rng.gen_range(0..u64::MAX);
        } else {
            return self.seed.into();
        }   
    }
}

#[wasm_bindgen]
pub struct ColorArgs {
    pub coloring: u32,
    pub darklevel: f64,
    pub waterlevel: f64,
}

#[wasm_bindgen]
impl ColorArgs {
    #[wasm_bindgen(constructor)]
    pub fn new(coloring: u32, darklevel: f64, waterlevel: f64) -> ColorArgs {
        ColorArgs { coloring, darklevel, waterlevel }
    }

    pub fn get_coloring(&self) -> u32 { return self.coloring }
    pub fn get_darklevel(&self) -> f64 { return self.darklevel }
    pub fn get_waterlevel(&self) -> f64 { return self.waterlevel }
}

fn initialize_algorithm(width: u32, height: u32, seed: u64, algorithm: &str) -> Box<dyn ImageAlg> {
    match algorithm {
        "DiamondSquare" | "ds" => Box::new(DiamondSquare::new(width, height, seed)),
        "DiamondSquareBorderless" | "dsb" => Box::new(DiamondSquareBorderless::new(width, height, seed)),
         _ => {
            println!("Provided algorithm {} not recognized. Defaulting to DiamondSquare", algorithm);
            Box::new(DiamondSquare::new(width, height, seed))
        }
    }
}

#[wasm_bindgen]
pub fn makeimage(ctx: &CanvasRenderingContext2d, mapargs: MapArgs, colorargs: ColorArgs, algorithm: &str) -> Result<(), JsValue> {
    #[cfg(debug_assertions)]
    console_error_panic_hook::set_once();

    //console_log!("Algorithm: {}", coloring);

    let mut dx = initialize_algorithm(mapargs.get_width(), mapargs.get_height(), mapargs.get_seed(), algorithm);

    dx.run(mapargs.get_chaos(), mapargs.get_damping(), mapargs.get_blocksize());

    let mapdata = dx.get_data();
    let mut colordata = Coloring::new(mapdata, colorargs);
    let mut imagevec = colordata.get_vec();

    let dim = dx.get_dim();
    let imagedata = ImageData::new_with_u8_clamped_array_and_sh(Clamped(&mut imagevec), dim, dim)?;
    ctx.put_image_data(&imagedata, 0.0, 0.0)
}
