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

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = console)]
    fn log(s: &str);
}

macro_rules! console_log {
    ($($t:tt)*) => (log(&format_args!($($t)*).to_string()))
}

#[wasm_bindgen]
pub fn makeimage_rainbow(ctx: &CanvasRenderingContext2d, width: u32, height: u32, chaos: f64) -> Result<(), JsValue> {
    #[cfg(debug_assertions)]
    console_error_panic_hook::set_once();

    //Todo: Disallow very large sizes

    let mut dx = DiamondSquare::new(width, height);
    dx.run(0.5);

    let mapdata = dx.get_data();
    let coloring = Coloring::new(mapdata);

    console_log!("Algorithm: Rainbow");

    let mut imagevec = coloring.data_to_rainbow_vec();

    let imagedata = ImageData::new_with_u8_clamped_array_and_sh(Clamped(&mut imagevec), dx.dim, dx.dim)?;
    ctx.put_image_data(&imagedata, 0.0, 0.0)
}        

#[wasm_bindgen]
pub fn makeimage_bluegreen(ctx: &CanvasRenderingContext2d, width: u32, height: u32, chaos: f64, waterlevel: f64) -> Result<(), JsValue> {
    #[cfg(debug_assertions)]
    console_error_panic_hook::set_once();

    let mut dx = DiamondSquare::new(width, height);
    dx.run(chaos);

    let mapdata = dx.get_data();
    let coloring = Coloring::new(mapdata);

    console_log!("Algorithm: bluegreen");

    let mut imagevec = coloring.data_to_blue_green_vec(waterlevel);

    let imagedata = ImageData::new_with_u8_clamped_array_and_sh(Clamped(&mut imagevec), dx.dim, dx.dim)?;
    ctx.put_image_data(&imagedata, 0.0, 0.0)
}
