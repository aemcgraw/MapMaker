pub mod algorithms;
pub mod map_data;
pub mod config;
pub mod utilities;
pub mod coloring;
pub mod wasm_structs;

use wasm_structs::MapArgs;
use wasm_structs::ColorArgs;

use wasm_bindgen::prelude::*;
use wasm_bindgen::Clamped;
use web_sys::{CanvasRenderingContext2d, ImageData};

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
