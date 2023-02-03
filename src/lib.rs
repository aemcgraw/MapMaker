pub mod algorithms;
pub mod map_data;
pub mod config;
pub mod util;
pub mod coloring;

use wasm_bindgen::prelude::*;
use wasm_bindgen::Clamped;
use web_sys::{CanvasRenderingContext2d, ImageData};

use algorithms::Run;
use algorithms::ImageAlg;
use algorithms::diamond_square::DiamondSquare;
use algorithms::GetData;

use coloring::coloring::Coloring;

//fn initialize_algorithm(width: u32, height: u32, algorithm: &str) -> Box<dyn ImageAlg> {
//    match algorithm {
//        "DiamondSquare" | "ds" => Box::new(DiamondSquare::new(width, height)),
        //"DiamondSquareBorderless" | "dsb" => Box::new(DiamondSquareBorderless::new(width, height)),
//        _ => {
//            println!("Provided algorithm {} not recognized. Defaulting to DiamondSquare", algorithm);
//            Box::new(DiamondSquare::new(width, height))
//        }
//    }
//}


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
    pub blocksize: u32
}

#[wasm_bindgen]
impl MapArgs {
    #[wasm_bindgen(constructor)]
    pub fn new(width: u32, height: u32, chaos: f64, damping: f64, blocksize: u32) -> MapArgs {
        MapArgs { width, height, chaos, damping, blocksize }
    }

    pub fn get_width(&self) -> u32 { return self.width }
    pub fn get_height(&self) -> u32 { return self.height }
    pub fn get_chaos(&self) -> f64 { return self.chaos }
    pub fn get_damping(&self) -> f64 { return self.damping }
    pub fn get_blocksize(&self) -> u32 { return self.blocksize }
}

#[wasm_bindgen]
pub fn makeimage_rainbow(ctx: &CanvasRenderingContext2d, mapargs: MapArgs) -> Result<(), JsValue> {
    #[cfg(debug_assertions)]
    console_error_panic_hook::set_once();

    console_log!("Algorithm: Rainbow");

    //Todo: Disallow very large sizes

    let mut dx = DiamondSquare::new(mapargs.get_width(), mapargs.get_height());
    dx.run(mapargs.get_chaos(), mapargs.get_damping(), mapargs.get_blocksize());

    let mapdata = dx.get_data();
    let coloring = Coloring::new(mapdata);
    let mut imagevec = coloring.data_to_rainbow_vec();

    let imagedata = ImageData::new_with_u8_clamped_array_and_sh(Clamped(&mut imagevec), dx.dim, dx.dim)?;
    ctx.put_image_data(&imagedata, 0.0, 0.0)
}        

#[wasm_bindgen]
pub fn makeimage_bluegreen(ctx: &CanvasRenderingContext2d, mapargs: MapArgs, waterlevel: f64) -> Result<(), JsValue> {
    #[cfg(debug_assertions)]
    console_error_panic_hook::set_once();

    console_log!("Algorithm: bluegreen");

    let mut dx = DiamondSquare::new(mapargs.get_width(), mapargs.get_height());
    dx.run(mapargs.get_chaos(), mapargs.get_damping(), mapargs.get_blocksize());

    let mapdata = dx.get_data();
    let coloring = Coloring::new(mapdata);
    let mut imagevec = coloring.data_to_blue_green_vec(waterlevel);

    let imagedata = ImageData::new_with_u8_clamped_array_and_sh(Clamped(&mut imagevec), dx.dim, dx.dim)?;
    ctx.put_image_data(&imagedata, 0.0, 0.0)
}
