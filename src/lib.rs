pub mod algorithms;
pub mod map_data;
pub mod config;
pub mod util;
pub mod coloring;

use wasm_bindgen::prelude::*;
use wasm_bindgen::Clamped;
use web_sys::{CanvasRenderingContext2d, ImageData};

use rand::distributions::Uniform;
use rand::Rng;

use algorithms::ImageAlg;
use algorithms::diamond_square::DiamondSquare;
use algorithms::diamond_square_borderless::DiamondSquareBorderless;
use util::kdtree::KDTree;

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
    pub waterlevel: f64,
    pub seed: u32,
}

#[wasm_bindgen]
impl MapArgs {
    #[wasm_bindgen(constructor)]
    pub fn new(width: u32, height: u32, chaos: f64, damping: f64, blocksize: u32, waterlevel: f64, seed: u32) -> MapArgs {
        MapArgs { width, height, chaos, damping, blocksize, waterlevel, seed }
    }

    pub fn get_width(&self) -> u32 { return self.width }
    pub fn get_height(&self) -> u32 { return self.height }
    pub fn get_chaos(&self) -> f64 { return self.chaos }
    pub fn get_damping(&self) -> f64 { return self.damping }
    pub fn get_blocksize(&self) -> u32 { return self.blocksize }
    pub fn get_waterlevel(&self) -> f64 { return self.waterlevel }
    pub fn get_seed(&self) -> u64 {
        if self.seed == 0 {
            let mut rng = rand::thread_rng();
            return rng.gen_range(0..u64::MAX);
        } else {
            return self.seed.into();
        }   
    }
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
pub fn makeimage(ctx: &CanvasRenderingContext2d, mapargs: MapArgs, algorithm: &str, coloring: &str) -> Result<(), JsValue> {
    #[cfg(debug_assertions)]
    console_error_panic_hook::set_once();

    console_log!("Algorithm: {}", coloring);

    let mut dx = initialize_algorithm(mapargs.get_width(), mapargs.get_height(), mapargs.get_seed(), algorithm);

    dx.run(mapargs.get_chaos(), mapargs.get_damping(), mapargs.get_blocksize());

    let mapdata = dx.get_data();
    let colordata = Coloring::new(mapdata);
    let mut imagevec = match coloring {
        "BlueGreen" => colordata.data_to_blue_green_vec(mapargs.get_waterlevel()),
        "Rainbow" => colordata.data_to_rainbow_vec(),
        "Topographical" => colordata.data_to_topographical_vec(Some(mapargs.get_waterlevel())),
        _ => colordata.data_to_blue_green_vec(mapargs.get_waterlevel())
    };

    let dim = dx.get_dim();
    let imagedata = ImageData::new_with_u8_clamped_array_and_sh(Clamped(&mut imagevec), dim, dim)?;
    ctx.put_image_data(&imagedata, 0.0, 0.0)

    //return imagevec;
}

//#[wasm_bindgen]
//pub fn updateimage(ctx: &CanvasRendeingContext2d, coloring: &str) -> Result<(), JsValue> {
//    #[cfg(debug_assertions)]
//    console_error_panic_hook::set_once();

//    console_log!("Algorithm: {}", coloring);

//    let mut dx = initialize_a
//}
